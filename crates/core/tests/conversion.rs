//! Acceptance tests for the conversion core: full-range round trips, external
//! anchors, formatting, error behaviour, and the cross-source audit trail.

use std::collections::{BTreeMap, BTreeSet};

use npt_core::*;

const SOURCE_NDC: &str = include_str!("../../../data/sources/nepali-date-converter.csv");
const SOURCE_BIK: &str = include_str!("../../../data/sources/bikram-sambat.csv");
const SOURCE_PY: &str = include_str!("../../../data/sources/nepali-datetime.csv");
const SOURCE_NEP: &str = include_str!("../../../data/sources/nepdate.csv");
const OVERRIDES: &str = include_str!("../../../data/overrides.csv");
const SNAPSHOT: &str = include_str!("../../../data/bs-months.csv");

fn parse_wide(text: &str) -> BTreeMap<i32, [u8; 12]> {
    let mut lines = text.lines();
    let header: Vec<&str> = lines.next().expect("header").trim_end_matches('\r').split(',').collect();
    assert_eq!(
        header[1..].join(","),
        "Baisakh,Jestha,Ashadh,Shrawan,Bhadra,Ashwin,Kartik,Mangsir,Poush,Magh,Falgun,Chaitra",
        "source header must name the twelve months in order"
    );
    let mut map = BTreeMap::new();
    for line in lines {
        let line = line.trim_end_matches('\r');
        if line.is_empty() {
            continue;
        }
        let fields: Vec<&str> = line.split(',').collect();
        assert_eq!(fields.len(), 13, "row shape: {line}");
        let year: i32 = fields[0].parse().unwrap();
        let mut row = [0u8; 12];
        for (slot, field) in row.iter_mut().zip(&fields[1..]) {
            *slot = field.parse().unwrap();
            assert!((29..=32).contains(&*slot), "impossible length in {line}");
        }
        map.insert(year, row);
    }
    map
}

fn parse_overrides(text: &str) -> BTreeMap<(i32, u8), (u8, String)> {
    let mut map = BTreeMap::new();
    for line in text.lines().skip(1) {
        let line = line.trim_end_matches('\r');
        if line.is_empty() {
            continue;
        }
        let fields: Vec<&str> = line.splitn(6, ',').collect();
        assert_eq!(fields.len(), 6, "override row shape: {line}");
        let year: i32 = fields[0].parse().unwrap();
        let month: u8 = fields[1].parse().unwrap();
        let chosen: u8 = fields[2].parse().unwrap();
        let rule = fields[3].to_string();
        assert!(
            !fields[4].is_empty(),
            "override must record the competing values: {line}"
        );
        assert!(
            !fields[5].trim().is_empty(),
            "override must carry a note or citation: {line}"
        );
        assert!(
            map.insert((year, month), (chosen, rule)).is_none(),
            "duplicate override for {year}-{month}"
        );
    }
    map
}

#[test]
fn snapshot_covers_1975_to_2100() {
    let map = parse_wide(SNAPSHOT);
    assert_eq!(map.len(), 126);
    assert_eq!(map.keys().next(), Some(&1975));
    assert_eq!(map.keys().next_back(), Some(&2100));
    for (year, row) in &map {
        assert_eq!(bs_year_months(*year), Some(*row), "table/CSV mismatch at {year}");
        let sum: u32 = row.iter().map(|&d| d as u32).sum();
        assert!(sum == 365 || sum == 366, "{year} has {sum} days");
    }
}

#[test]
fn anchors_match_external_references() {
    let cases = [
        ((1975, 1, 1), (1918, 4, 13)),
        ((2000, 1, 1), (1943, 4, 14)),
        ((2082, 1, 1), (2025, 4, 14)),
        ((2083, 1, 1), (2026, 4, 14)),
        ((2083, 8, 5), (2026, 11, 21)),
    ];
    for ((by, bm, bd), (ay, am, ad)) in cases {
        let bs = BsDate { year: by, month: bm, day: bd };
        let expected = AdDate { year: ay, month: am, day: ad };
        assert_eq!(bs_to_ad(bs), Ok(expected), "BS {by}-{bm}-{bd}");
        assert_eq!(ad_to_bs(expected), Ok(bs), "AD {ay}-{am}-{ad}");
    }
}

#[test]
fn every_ad_day_round_trips() {
    let (first, last) = ad_bounds();
    let mut days = days_from_civil(first.year, first.month, first.day);
    let end = days_from_civil(last.year, last.month, last.day);
    let mut count = 0u32;
    while days <= end {
        let ad = civil_from_days(days);
        let bs = ad_to_bs(ad).unwrap_or_else(|e| panic!("{ad:?}: {e}"));
        assert_eq!(bs_to_ad(bs), Ok(ad), "round trip failed for {ad:?}");
        count += 1;
        days += 1;
    }
    assert!(count > 46_000, "range only covered {count} days");
    assert_eq!(ad_to_bs(civil_from_days(end + 1)), Err(DateError::OutOfRange));
    assert_eq!(ad_to_bs(civil_from_days(days_from_civil(first.year, first.month, first.day) - 1)), Err(DateError::OutOfRange));
}

#[test]
fn mapping_is_strictly_monotonic() {
    let (first, last) = ad_bounds();
    let mut days = days_from_civil(first.year, first.month, first.day);
    let end = days_from_civil(last.year, last.month, last.day);
    let mut previous: Option<BsDate> = None;
    while days <= end {
        let ad = civil_from_days(days);
        let bs = ad_to_bs(ad).unwrap_or_else(|e| panic!("{ad:?}: {e}"));
        if let Some(prev) = previous {
            assert!(bs > prev, "BS sequence not increasing: {prev:?} then {bs:?}");
        }
        previous = Some(bs);
        days += 1;
    }
}

#[test]
fn every_bs_day_round_trips() {
    let mut count = 0u32;
    for year in BS_MIN_YEAR..=BS_MAX_YEAR {
        for month in 1..=12u8 {
            let length = bs_days_in_month(year, month).unwrap();
            for day in 1..=length {
                let bs = BsDate { year, month, day };
                let ad = bs_to_ad(bs).unwrap_or_else(|e| panic!("{bs:?}: {e}"));
                assert_eq!(ad_to_bs(ad), Ok(bs), "round trip failed for {bs:?}");
                count += 1;
            }
        }
    }
    assert!(count > 46_000, "range only covered {count} BS days");
}

#[test]
fn invalid_and_out_of_range_are_rejected() {
    assert_eq!(
        bs_to_ad(BsDate { year: 1974, month: 12, day: 30 }),
        Err(DateError::OutOfRange)
    );
    assert_eq!(
        bs_to_ad(BsDate { year: 2101, month: 1, day: 1 }),
        Err(DateError::OutOfRange)
    );
    assert_eq!(
        bs_to_ad(BsDate { year: 2083, month: 13, day: 1 }),
        Err(DateError::Invalid)
    );
    assert_eq!(
        bs_to_ad(BsDate { year: 2083, month: 8, day: 30 }),
        Err(DateError::Invalid)
    );
    assert_eq!(
        ad_to_bs(AdDate { year: 1918, month: 4, day: 12 }),
        Err(DateError::OutOfRange)
    );
    assert_eq!(
        ad_to_bs(AdDate { year: 2026, month: 2, day: 30 }),
        Err(DateError::Invalid)
    );
    assert_eq!(
        ad_to_bs(AdDate { year: 2026, month: 13, day: 1 }),
        Err(DateError::Invalid)
    );
}

#[test]
fn weekdays_are_correct() {
    assert_eq!(weekday_name(AdDate { year: 2026, month: 11, day: 21 }, Lang::Np), "शनिबार");
    assert_eq!(weekday_name(AdDate { year: 2026, month: 11, day: 21 }, Lang::En), "Saturday");
    assert_eq!(weekday_name(AdDate { year: 2025, month: 4, day: 14 }, Lang::En), "Monday");
    assert_eq!(weekday_name(AdDate { year: 2000, month: 1, day: 1 }, Lang::En), "Saturday");
}

#[test]
fn formatting_is_nepali_first() {
    let bs = BsDate { year: 2083, month: 8, day: 5 };
    assert_eq!(format_bs(bs, Lang::Np), "मंसिर ५, २०८३");
    assert_eq!(format_bs(bs, Lang::En), "Mangsir 5, 2083");
    let ad = AdDate { year: 2026, month: 11, day: 21 };
    assert_eq!(format_ad(ad, Lang::Np), "२१ नोभेम्बर २०२६");
    assert_eq!(format_ad(ad, Lang::En), "21 November 2026");
    assert_eq!(tooltip(ad, Lang::Np).unwrap(), "मंसिर ५, २०८३ - शनिबार");
    assert_eq!(
        tooltip(ad, Lang::En).unwrap(),
        "मंसिर ५, २०८३ - Saturday",
        "the BS date stays Nepali in English mode"
    );
    assert_eq!(
        tooltip(AdDate { year: 2026, month: 11, day: 22 }, Lang::Np).unwrap(),
        "मंसिर ६, २०८३ - आइतबार"
    );
    assert_eq!(format_digits(2083, Lang::Np), "२०८३");
    assert_eq!(format_digits(0, Lang::Np), "०");
}

#[test]
fn projected_years_are_flagged() {
    let last_verified = bs_to_ad(BsDate { year: 2090, month: 12, day: 1 }).unwrap();
    assert!(!describe(last_verified).unwrap().projected);
    let projected = bs_to_ad(BsDate { year: 2091, month: 1, day: 1 }).unwrap();
    assert!(describe(projected).unwrap().projected);
}

#[test]
fn every_disagreement_is_recorded_and_every_record_is_justified() {
    let sources = [
        ("nepali-date-converter", parse_wide(SOURCE_NDC)),
        ("bikram-sambat", parse_wide(SOURCE_BIK)),
        ("nepali-datetime", parse_wide(SOURCE_PY)),
        ("nepdate", parse_wide(SOURCE_NEP)),
    ];
    let overrides = parse_overrides(OVERRIDES);
    let mut seen = BTreeSet::new();
    let mut disputed = 0usize;
    let mut single = 0usize;

    for year in BS_MIN_YEAR..=BS_MAX_YEAR {
        for month in 1..=12u8 {
            let votes: Vec<(&str, u8)> = sources
                .iter()
                .filter_map(|(name, map)| map.get(&year).map(|row| (*name, row[month as usize - 1])))
                .collect();
            assert!(votes.len() >= 2, "{year}-{month} has too few sources");
            let unique: BTreeSet<u8> = votes.iter().map(|(_, v)| *v).collect();
            let key = (year, month);

            if year > BS_VERIFIED_MAX_YEAR {
                let (chosen, rule) = overrides.get(&key).unwrap_or_else(|| panic!("{key:?} missing"));
                assert_eq!(*rule, "single-source");
                assert_eq!(*chosen, votes.iter().find(|(n, _)| *n == "nepdate").unwrap().1);
                single += 1;
                seen.insert(key);
                continue;
            }

            if unique.len() == 1 {
                assert!(overrides.get(&key).is_none(), "{key:?} is unanimous but has an override");
                continue;
            }

            let (chosen, rule) = overrides
                .get(&key)
                .unwrap_or_else(|| panic!("unrecorded disagreement at {key:?}"));
            let chosen_count = votes.iter().filter(|(_, v)| v == chosen).count();
            let rival_count = votes
                .iter()
                .filter(|(_, v)| v != chosen)
                .map(|(_, v)| votes.iter().filter(|(_, w)| w == v).count())
                .max()
                .unwrap_or(0);
            match rule.as_str() {
                "majority" => assert!(
                    chosen_count > rival_count,
                    "{key:?} claims majority but {chosen_count} vs {rival_count}"
                ),
                "tie-bik" => {
                    assert_eq!(chosen_count, rival_count, "{key:?} claims a tie");
                    assert_eq!(
                        *chosen,
                        votes.iter().find(|(n, _)| *n == "bikram-sambat").unwrap().1,
                        "{key:?} tie must take the bikram-sambat value"
                    );
                    assert_eq!(chosen_count, 2, "{key:?} tie expects two lineages");
                }
                other => panic!("unknown rule {other} at {key:?}"),
            }
            disputed += 1;
            seen.insert(key);
        }
    }

    for key in overrides.keys() {
        assert!(seen.contains(key), "override for {key:?} is not justified by any disagreement");
    }
    assert_eq!(overrides.len(), disputed + single);
    assert_eq!(disputed, 27);
    assert_eq!(single, 120);
}
