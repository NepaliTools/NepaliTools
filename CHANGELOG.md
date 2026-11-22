# Changelog

All notable changes are recorded here. The format follows Keep a Changelog, and
the project uses Semantic Versioning.

## [0.1.5] - 2026-11-22

### Fixed

- Settings written by 0.1.3 no longer reset on upgrade. The `modules` object
  gained a `text` field, and without a serde default for it the whole settings
  file failed to parse, silently restoring language, channel, Nepal date,
  autostart and module toggles to their defaults. Missing fields now take their
  documented defaults and every other saved value survives.

## [0.1.4] - 2026-11-22

Text module: Preeti and Unicode in both directions, plus an in-place shortcut.

### Added

- Text module: a live two-way converter between the legacy Preeti encoding and
  Unicode Devanagari, with copy and swap. Unrepresentable characters stay as they
  are and are listed under the box.
- `npt_core::preeti`: dependency-free conversion with a vendored key table
  (`data/preeti-map.tsv`) and a corpus of 81 pairs (`data/preeti-corpus.tsv`)
  replayed in both directions by `cargo test`.
- Global shortcut: `Ctrl+Alt+P` converts the selection in place; in split mode
  `Ctrl+Alt+P` is Unicode to Preeti and `Ctrl+Alt+U` the reverse. The clipboard is
  restored, the panel is never a target, and a refused copy is reported instead
  of pasting stale content.
- `data/SOURCES.md` records the Preeti table source, version and SHA256.

### Fixed

- The Channel dropdown drew its rows white while inheriting light text, so Stable
  and Beta were nearly invisible in dark mode. The popup and its options now set
  their own colours in both themes.

## [0.1.3] - 2026-11-22

Update channels.

### Added

- Channel choice in Settings: Stable, Beta and Alpha. Each reads its own signed
  manifest (`updates/stable.json`, `updates/beta.json`, `updates/alpha.json`) and
  the choice is stored in `settings.json`.
- A channel with no release yet says so instead of claiming the app is current.
- `tools/promote.ps1` moves a published release to a slower channel without
  rebuilding it, and `tools/release.ps1` gained `-Channel`.
- `updates/alpha.json` on the site; `updates/latest.json` stays as the alias that
  0.1.2 reads.

### Changed

- The updater endpoint is chosen at runtime instead of coming only from
  `tauri.conf.json`. The configured endpoint remains the fallback for builds that
  predate channels.
- Release tags follow the channel: `v0.1.3-alpha`, `v0.2.0-beta`, `v0.2.0`.

## [0.1.2] - 2026-11-21

The first build with updates.

### Added

- Updates section in Settings. Check for updates reads a signed release manifest,
  downloads the signed NSIS installer from GitHub Releases, verifies the signature,
  installs quietly and restarts the app. Nothing checks on a timer or at startup.
- `updates/latest.json` on nepalitools.pages.dev, the updater public key in
  `tauri.conf.json`, and updater signatures on every release build.
- `tools/release.ps1` and `docs/RELEASING.md` for signed releases.
- The portable build points at the release page instead of trying to replace
  itself.

### Changed

- The only network request at runtime is the update check the user starts.
  README, SECURITY, the site and the wiki say so.
- The installer description no longer claims there is no network.
- The update check reads the manifest for the chosen channel, and the channel is
  saved with the other settings.

## [0.1.0] - 2026-11-21

Initial public alpha.

### Added

- Branded per-user NSIS installer and a portable zip, published as the 0.1.0
  alpha release.
- Tray host in Rust with Tauri 2 and a plain HTML/CSS/JS panel. No Node, npm or
  bundled browser.
- `npt-core`, a dependency-free Bikram Sambat engine: BS to AD conversion,
  formatting, Devanagari digits, weekdays and range errors.
- Today module with today's BS date, weekday and AD equivalent, plus copy.
- Converter module with live conversion from 1975 to 2100 BS in both directions,
  a Today button, copy on either result and explicit out-of-range errors.
- Modules page that enables or disables the compiled-in modules.
- Settings: English interface by default with a Nepali toggle, a Nepal date
  (UTC+05:45) override, and start with Windows.
- Tray icon with the BS day in Devanagari, rendered at 16, 20, 24 and 32 px and
  chosen by display scale. Full-date tooltip and midnight refresh.
- Single instance, `Ctrl+Alt+N`, and autostart through
  `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`.
- A committed BS table built from four sources, with majority resolution, cited
  overrides and a cross-source audit test.
- 12 automated tests, an acceptance record with screenshots, and architecture
  decision records.
