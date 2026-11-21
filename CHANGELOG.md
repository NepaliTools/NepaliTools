# Changelog

All notable changes are recorded here. The format follows Keep a Changelog, and
the project uses Semantic Versioning.

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
