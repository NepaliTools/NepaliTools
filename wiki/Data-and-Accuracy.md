# Data and accuracy

The app is offline, so its accuracy is the accuracy of the table it ships.

## Sources

`data/bs-months.csv` is built from four separate Bikram Sambat tables:

| Source | Version | Range (BS) | License |
|---|---|---|---|
| nepali-date-converter | 3.4.2 | 2000 to 2090 | MIT |
| bikram-sambat | 1.8.1 | 1970 to 2090 | MIT |
| nepali-datetime | 1.0.8.5 | 1975 to 2100 | MIT |
| nepdate | 1.0.0-rc.1 | 1901 to 2199 | MIT (data: RajuPrasai/NepDate) |

`data/SOURCES.md` records the URL, retrieval date and SHA256 of each archive. The
fetch script refuses to run if a hash changes.

## How a month length is chosen

1. Unanimous: use the shared value.
2. Strict majority: use the majority value.
3. Tie (2 against 2): take the `bikram-sambat` / `nepali-date-converter` lineage
   and record the competing values.
4. 2091 to 2100: no gazette and no second coherent table, because one source
   contains an impossible 364-day year at 2096. `nepdate` is authoritative there
   and every row is marked single-source.

Every non-unanimous month lands in `data/overrides.csv` with the chosen value, the
rule, the competing values and a note. That is 27 disputed months and 120
single-source months.

## Epoch cross-check

Each lineage converts its own epoch to the same anchor without sharing code:

- `bikram-sambat`: 1970-01-01 BS is 1913-04-13 AD, so 1975-01-01 is 1918-04-13
- `nepdate`: 1901-01-01 BS is 1844-04-11 AD, so 1975-01-01 is 1918-04-13
- `nepali-datetime`: `REFERENCE_DATE_AD` is 1918-04-13

## What the tests prove

`cargo test --workspace` runs 12 tests:

- every day in both directions round-trips, 46,022 days each way;
- the BS sequence is strictly increasing as AD advances;
- external anchors (1975, 2000, 2082, 2083) match;
- invalid and out-of-range input returns errors and never clamps;
- the four committed source tables produce exactly the recorded overrides. An
  unexplained disagreement fails the build.

## Refreshing the table

The only step that uses the network:

```powershell
powershell -ExecutionPolicy Bypass -File tools\fetch-bs-data.ps1
cargo test --workspace
```

Commit the CSV, the per-source snapshots, the overrides and `SOURCES.md`
together.

## Known limits

- BS 2091 to 2100 is projection, labelled in the UI.
- Data is as good as the published tables. A gazette correction needs a new
  snapshot.
- No tithi, holidays or panchang calculations.
