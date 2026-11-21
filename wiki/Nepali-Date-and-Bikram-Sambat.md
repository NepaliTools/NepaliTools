# Nepali date and Bikram Sambat

Bikram Sambat (BS, विक्रम संवत्) is Nepal's official civil calendar. It runs
roughly 56 years and 8 months ahead of the Gregorian calendar, and its new year
falls in mid-April.

## Why the app needs a table

Gregorian month lengths follow a rule. BS month lengths do not. They are published
year by year, and a month can be 29, 30, 31 or 32 days depending on the year. Two
consequences follow:

1. A converter is only as good as its data. NepaliToolkit ships a committed
   snapshot and cites every source.
2. Future years are uncertain. Nepal publishes the official calendar only a short
   time ahead, so years after 2090 have no gazette yet. The app labels them
   projected.

## Months of the year

| # | Nepali | Transliteration |
|---|---|---|
| 1 | बैशाख | Baisakh |
| 2 | जेठ | Jestha |
| 3 | असार | Ashadh |
| 4 | साउन | Shrawan |
| 5 | भदौ | Bhadra |
| 6 | असोज | Ashwin |
| 7 | कात्तिक | Kartik |
| 8 | मंसिर | Mangsir |
| 9 | पुष | Poush |
| 10 | माघ | Magh |
| 11 | फागुन | Falgun |
| 12 | चैत | Chaitra |

## Weekdays

| Nepali | English |
|---|---|
| आइतबार | Sunday |
| सोमबार | Monday |
| मंगलबार | Tuesday |
| बुधबार | Wednesday |
| बिहीबार | Thursday |
| शुक्रबार | Friday |
| शनिबार | Saturday |

## Supported range and anchors

- Range: BS 1975-01-01 to 2100-12-30.
- In the Gregorian calendar: 1918-04-13 to 2044-04-13.
- Verified anchors:
  - BS 1975-01-01 is 1918-04-13 AD
  - BS 2000-01-01 is 1943-04-14 AD
  - BS 2082-01-01 is 2025-04-14 AD
  - BS 2083-01-01 is 2026-04-14 AD
- Years 2091 to 2100 are single-source projections, marked in the interface.

## Nepal time

Nepal uses UTC+05:45, one of the few 45-minute offsets in the world. By default
the app uses your machine's date, so the tray agrees with your clock. Turn on
Nepal date in Settings to always show Nepal's date instead.

## See also

- [Data and accuracy](Data-and-Accuracy)
- [Features](Features)
