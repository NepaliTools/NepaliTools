## What changed

## Why

## How verified

- [ ] `cargo test --workspace`
- [ ] `cargo build --release -p nepalitoolkit`
- [ ] Checked the panel on Windows (screenshot attached if the UI changed)

## Checks

- [ ] Date logic stays in `crates/core`; no calendar arithmetic in JavaScript
- [ ] No runtime network calls
- [ ] No new UI dependencies (no npm, no bundled browser)
- [ ] ADR added or updated if the architecture changed
- [ ] `GLOSSARY.md` updated if a domain term changed
- [ ] Date table regenerated with `tools/fetch-bs-data.ps1` if data changed
