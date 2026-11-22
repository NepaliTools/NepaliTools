//! Bikram Sambat (BS) conversion and Nepali formatting.
//!
//! Every date rule in the application lives here. The month-length table is a
//! compile-time snapshot of `data/bs-months.csv`, itself built and audited by
//! `tools/fetch-bs-data.ps1` (see `data/SOURCES.md`). No network, no clock, no
//! platform calls: the host passes in a civil date.

use std::fmt;
use std::sync::LazyLock;

pub mod preeti;

/// First supported BS year (inclusive).
pub const BS_MIN_YEAR: i32 = 1975;
/// Last supported BS year (inclusive). Years after 2090 are projected data.
pub const BS_MAX_YEAR: i32 = 2100;
/// Last BS year backed by more than one independent source.
pub const BS_VERIFIED_MAX_YEAR: i32 = 2090;

const SNAPSHOT: &str = include_str!("../../../data/bs-months.csv");
const MONTH_COUNT: usize = 12;

const MONTHS_EN: [&str; MONTH_COUNT] = [
    "Baisakh", "Jestha", "Ashadh", "Shrawan", "Bhadra", "Ashwin", "Kartik", "Mangsir", "Poush",
    "Magh", "Falgun", "Chaitra",
];
const MONTHS_NP: [&str; MONTH_COUNT] = [
    "बैशाख", "जेठ", "असार", "साउन", "भदौ", "असोज", "कात्तिक", "मंसिर", "पुष", "माघ", "फागुन",
    "चैत",
];
const AD_MONTHS_EN: [&str; MONTH_COUNT] = [
    "January", "February", "March", "April", "May", "June", "July", "August", "September",
    "October", "November", "December",
];
const AD_MONTHS_NP: [&str; MONTH_COUNT] = [
    "जनवरी", "फेब्रुअरी", "मार्च", "अप्रिल", "मे", "जुन", "जुलाई", "अगस्ट", "सेप्टेम्बर",
    "अक्टोबर", "नोभेम्बर", "डिसेम्बर",
];
const WEEKDAYS_EN: [&str; 7] = [
    "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday",
];
const WEEKDAYS_NP: [&str; 7] = [
    "आइतबार", "सोमबार", "मंगलबार", "बुधबार", "बिहीबार", "शुक्रबार", "शनिबार",
];

/// Language for display strings. Numerals follow the same switch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Np,
    En,
}

/// A civil (Gregorian) date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AdDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

/// A Bikram Sambat date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BsDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

/// Why a date could not be converted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateError {
    /// Outside 1975-01-01 ..= 2100-12-last BS (or the matching AD span).
    OutOfRange,
    /// A month or day that cannot exist.
    Invalid,
}

impl fmt::Display for DateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DateError::OutOfRange => write!(f, "date is outside the supported BS 1975-2100 range"),
            DateError::Invalid => write!(f, "date does not exist"),
        }
    }
}

impl std::error::Error for DateError {}

/// A full description of one day, formatted for both languages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day {
    pub bs: BsDate,
    pub ad: AdDate,
    pub bs_np: String,
    pub bs_en: String,
    pub ad_np: String,
    pub ad_en: String,
    pub weekday_np: &'static str,
    pub weekday_en: &'static str,
    /// True when the BS year has no second independent source (after 2090).
    pub projected: bool,
}

struct Table {
    years: Vec<[u8; MONTH_COUNT]>,
    /// Day offset of each year's first day, from 1975-01-01.
    year_start: Vec<i64>,
    total_days: i64,
    /// Days since 1970-01-01 of 1975-01-01 BS (= 1918-04-13 AD).
    epoch: i64,
}

impl Table {
    fn parse() -> Self {
        let mut years = Vec::new();
        let mut lines = SNAPSHOT.lines();
        let header = lines.next().expect("snapshot has a header");
        let header: Vec<&str> = header.trim_end_matches('\r').split(',').collect();
        assert_eq!(header.len(), MONTH_COUNT + 1, "snapshot header shape");
        assert_eq!(header[0], "bs_year");
        for line in lines {
            let line = line.trim_end_matches('\r');
            if line.is_empty() {
                continue;
            }
            let mut fields = line.split(',');
            let year: i32 = fields.next().expect("year").parse().expect("year is a number");
            let expect = BS_MIN_YEAR + years.len() as i32;
            assert_eq!(year, expect, "snapshot years must be contiguous");
            let mut row = [0u8; MONTH_COUNT];
            for (index, slot) in row.iter_mut().enumerate() {
                *slot = fields
                    .next()
                    .unwrap_or_else(|| panic!("year {year} month {}", index + 1))
                    .parse()
                    .expect("month length is a number");
                assert!(
                    (29..=32).contains(&*slot),
                    "year {year} month {} has impossible length {slot}",
                    index + 1
                );
            }
            assert!(fields.next().is_none(), "year {year} has extra fields");
            years.push(row);
        }
        assert_eq!(
            years.len() as i32,
            BS_MAX_YEAR - BS_MIN_YEAR + 1,
            "snapshot must cover 1975..=2100"
        );

        let mut year_start = Vec::with_capacity(years.len());
        let mut offset = 0i64;
        for (index, row) in years.iter().enumerate() {
            let sum: u32 = row.iter().map(|&d| d as u32).sum();
            assert!(
                sum == 365 || sum == 366,
                "BS year {} has {sum} days",
                BS_MIN_YEAR + index as i32
            );
            year_start.push(offset);
            offset += sum as i64;
        }

        Table {
            years,
            year_start,
            total_days: offset,
            epoch: days_from_civil(1918, 4, 13),
        }
    }

    fn index(&self, year: i32) -> Option<usize> {
        if !(BS_MIN_YEAR..=BS_MAX_YEAR).contains(&year) {
            return None;
        }
        Some((year - BS_MIN_YEAR) as usize)
    }
}

static TABLE: LazyLock<Table> = LazyLock::new(Table::parse);

/// Howard Hinnant's civil-date algorithm. Days since 1970-01-01.
pub const fn days_from_civil(year: i32, month: u8, day: u8) -> i64 {
    let y = if month <= 2 { year as i64 - 1 } else { year as i64 };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (month as i64 + 9) % 12;
    let doy = (153 * mp + 2) / 5 + day as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// Inverse of [`days_from_civil`].
pub const fn civil_from_days(days: i64) -> AdDate {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    AdDate {
        year: (if month <= 2 { y + 1 } else { y }) as i32,
        month: month as u8,
        day: day as u8,
    }
}

/// True when the triplet is a real Gregorian date.
pub fn is_valid_ad(ad: AdDate) -> bool {
    if ad.year < 1 || !(1..=12).contains(&ad.month) || ad.day < 1 {
        return false;
    }
    ad.day <= ad_days_in_month(ad.year, ad.month)
}

/// Days in a Gregorian month.
pub const fn ad_days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

/// Gregorian leap year.
pub const fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// Month lengths of a BS year, or `None` outside the supported range.
pub fn bs_year_months(year: i32) -> Option<[u8; MONTH_COUNT]> {
    TABLE.index(year).map(|index| TABLE.years[index])
}

/// Days in a BS month.
pub fn bs_days_in_month(year: i32, month: u8) -> Option<u8> {
    if !(1..=12).contains(&month) {
        return None;
    }
    bs_year_months(year).map(|row| row[month as usize - 1])
}

/// Convert a BS date to AD. Errors instead of clamping.
pub fn bs_to_ad(bs: BsDate) -> Result<AdDate, DateError> {
    let index = TABLE.index(bs.year).ok_or(DateError::OutOfRange)?;
    if !(1..=12).contains(&bs.month) {
        return Err(DateError::Invalid);
    }
    let length = TABLE.years[index][bs.month as usize - 1];
    if bs.day < 1 || bs.day > length {
        return Err(DateError::Invalid);
    }
    let mut offset = TABLE.year_start[index];
    for &days in &TABLE.years[index][..bs.month as usize - 1] {
        offset += days as i64;
    }
    offset += bs.day as i64 - 1;
    Ok(civil_from_days(TABLE.epoch + offset))
}

/// Convert an AD date to BS. Errors instead of clamping.
pub fn ad_to_bs(ad: AdDate) -> Result<BsDate, DateError> {
    if !is_valid_ad(ad) {
        return Err(DateError::Invalid);
    }
    let days = days_from_civil(ad.year, ad.month, ad.day) - TABLE.epoch;
    if days < 0 || days >= TABLE.total_days {
        return Err(DateError::OutOfRange);
    }
    let year_index = match TABLE.year_start.binary_search(&days) {
        Ok(index) => index,
        Err(next) => next - 1,
    };
    let mut remaining = days - TABLE.year_start[year_index];
    let row = &TABLE.years[year_index];
    let mut month = 1u8;
    for &length in row {
        if remaining < length as i64 {
            break;
        }
        remaining -= length as i64;
        month += 1;
    }
    Ok(BsDate {
        year: BS_MIN_YEAR + year_index as i32,
        month,
        day: (remaining + 1) as u8,
    })
}

/// Weekday of an AD date, 0 = Sunday.
pub const fn weekday_sunday0(ad: AdDate) -> usize {
    let days = days_from_civil(ad.year, ad.month, ad.day);
    (days.rem_euclid(7) + 4) as usize % 7
}

/// Month name in BS.
pub fn bs_month_name(month: u8, lang: Lang) -> &'static str {
    let index = (month as usize).saturating_sub(1).min(MONTH_COUNT - 1);
    match lang {
        Lang::Np => MONTHS_NP[index],
        Lang::En => MONTHS_EN[index],
    }
}

/// Month name in the Gregorian calendar.
pub fn ad_month_name(month: u8, lang: Lang) -> &'static str {
    let index = (month as usize).saturating_sub(1).min(MONTH_COUNT - 1);
    match lang {
        Lang::Np => AD_MONTHS_NP[index],
        Lang::En => AD_MONTHS_EN[index],
    }
}

/// Weekday name.
pub fn weekday_name(ad: AdDate, lang: Lang) -> &'static str {
    let index = weekday_sunday0(ad);
    match lang {
        Lang::Np => WEEKDAYS_NP[index],
        Lang::En => WEEKDAYS_EN[index],
    }
}

/// Render a non-negative number, in Devanagari digits for Nepali.
pub fn format_digits(value: i64, lang: Lang) -> String {
    let text = value.to_string();
    match lang {
        Lang::En => text,
        Lang::Np => text
            .chars()
            .map(|c| match c {
                '0' => '०',
                '1' => '१',
                '2' => '२',
                '3' => '३',
                '4' => '४',
                '5' => '५',
                '6' => '६',
                '7' => '७',
                '8' => '८',
                '9' => '९',
                other => other,
            })
            .collect(),
    }
}

/// `मंसिर ५, २०८३` or `Mangsir 5, 2083`.
pub fn format_bs(bs: BsDate, lang: Lang) -> String {
    format!(
        "{} {}, {}",
        bs_month_name(bs.month, lang),
        format_digits(bs.day as i64, lang),
        format_digits(bs.year as i64, lang)
    )
}

/// `२१ नोभेम्बर २०२६` or `21 November 2026`.
pub fn format_ad(ad: AdDate, lang: Lang) -> String {
    format!(
        "{} {} {}",
        format_digits(ad.day as i64, lang),
        ad_month_name(ad.month, lang),
        format_digits(ad.year as i64, lang)
    )
}

/// Everything the panel needs about one day.
pub fn describe(ad: AdDate) -> Result<Day, DateError> {
    let bs = ad_to_bs(ad)?;
    Ok(Day {
        bs,
        ad,
        bs_np: format_bs(bs, Lang::Np),
        bs_en: format_bs(bs, Lang::En),
        ad_np: format_ad(ad, Lang::Np),
        ad_en: format_ad(ad, Lang::En),
        weekday_np: weekday_name(ad, Lang::Np),
        weekday_en: weekday_name(ad, Lang::En),
        projected: bs.year > BS_VERIFIED_MAX_YEAR,
    })
}

/// The tooltip line for the tray icon. The BS date is always rendered in Nepali,
/// in both UI languages; the weekday follows the UI language.
pub fn tooltip(ad: AdDate, lang: Lang) -> Result<String, DateError> {
    let day = describe(ad)?;
    let weekday = match lang {
        Lang::Np => day.weekday_np,
        Lang::En => day.weekday_en,
    };
    Ok(format!("{} - {}", day.bs_np, weekday))
}

/// The first and last AD dates the converter accepts.
pub fn ad_bounds() -> (AdDate, AdDate) {
    let first = bs_to_ad(BsDate { year: BS_MIN_YEAR, month: 1, day: 1 })
        .expect("snapshot starts with a real date");
    let last_month = 12;
    let last_day = bs_days_in_month(BS_MAX_YEAR, last_month).expect("last month exists");
    let last = bs_to_ad(BsDate { year: BS_MAX_YEAR, month: last_month, day: last_day })
        .expect("snapshot ends with a real date");
    (first, last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civil_round_trip() {
        for days in -100_000..100_000i64 {
            let date = civil_from_days(days);
            assert!(is_valid_ad(date), "{date:?} from day {days}");
            assert_eq!(days_from_civil(date.year, date.month, date.day), days);
        }
    }

    #[test]
    fn table_is_wired() {
        assert_eq!(bs_year_months(BS_MIN_YEAR - 1), None);
        assert_eq!(bs_year_months(BS_MAX_YEAR + 1), None);
        assert_eq!(bs_days_in_month(2083, 8), Some(29));
        assert_eq!(bs_days_in_month(2083, 0), None);
        assert_eq!(ad_bounds().0, AdDate { year: 1918, month: 4, day: 13 });
    }
}
