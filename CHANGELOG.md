# Changelog

All notable changes are recorded here. The format follows Keep a Changelog, and
the project uses Semantic Versioning.

## [Unreleased]

## [0.1.0] - 2026-11-21

Initial public release.

### Added

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
