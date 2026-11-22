# Handoff: NepaliToolkit

Updated 2026-11-22. Toolkit `main` and site `main`; run `git log -1` for the
exact heads.

A Windows tray app for Bikram Sambat: today's Nepali date in Devanagari on the
icon, a two-way BS and AD converter, English or Nepali interface, no telemetry.
Rust core with no dependencies, Tauri 2 host, plain HTML and CSS panel.

## Released

| Version | Channel | Date | Notes |
|---|---|---|---|
| 0.1.3 | alpha | 2026-11-22 | Update channels: Stable, Beta, Alpha. Current release. |
| 0.1.2 | alpha | 2026-11-21 | Manual update check, signed installer, quiet install. |
| 0.1.0 | alpha | 2026-11-21 | First public alpha. No updater. |

Each release is a GitHub prerelease with a signed NSIS installer, a portable zip,
`latest.json`, the `.sig` and `SHA256SUMS.txt`.

## How updates work

- Settings has a Channel choice and a Check for updates button. Nothing checks on
  a timer or at startup.
- The check reads `https://nepalitools.pages.dev/updates/<channel>.json`. That
  static manifest carries the version, notes, installer URL and signature.
- Install downloads the signed installer from GitHub Releases, verifies it
  against the public key in `crates/host/tauri.conf.json`, then runs the NSIS
  updater quietly (`/S /R`) and restarts the app.
- `requireSignedVersion` binds the version in the signature to the manifest, so a
  tampered manifest cannot pair a new version with an old installer.
- Portable builds are marked in the card and never offered the installer. They
  cannot replace themselves.
- `updates/latest.json` mirrors alpha for 0.1.2, the first build with an updater
  and no channel setting.

Channels are promotion tracks, not version lines. A build ships to alpha first;
promotion copies the manifest, so the bytes and signature never change:

```powershell
powershell -ExecutionPolicy Bypass -File tools\promote.ps1 -From alpha -To beta
powershell -ExecutionPolicy Bypass -File tools\promote.ps1 -From beta -To stable
```

An empty channel answers 404, which the app shows as "No Beta release yet". That
is a normal state, not an error.

## Releasing

Requirements: the updater key at `$HOME\.tauri\nepalitoolkit.key` (empty
password), `gh` with `repo` scope, and the site checkout at `..\nepalitools-site`.

1. Bump `version` in `Cargo.toml` and `crates/host/tauri.conf.json` (they must
   match).
2. Write `docs/release-notes-<version>-<channel>.md`.
3. Update install links in `README.md`, `wiki/Installation.md` and the site.
4. `cargo test --workspace`.
5. Commit before releasing, because `gh release create` tags the remote default
   branch tip, not the local working tree.
6. Run `tools\release.ps1 -Version 0.1.4 -Channel alpha`. It builds, signs,
   writes the manifest and checksums, creates the release, and deploys the site.
   `-SkipPublish` and `-SkipSiteDeploy` split the steps.

The updater key is the trust root. Losing it means existing installs cannot
verify a new release and everyone reinstalls by hand. Keep an offline backup and
never commit it.

## Unreleased, waiting for a batch

- Channel dropdown rows now follow the theme; the popup drew white rows with
  light text, so Stable and Beta were unreadable in dark mode. Fixed in
  `crates/host/ui/styles.css`, verified against the open popup. No release cut.

Releases are batched: keep changes under `[Unreleased]` in `CHANGELOG.md` and cut
a version only when several changes are worth shipping.

## Next

- Promote a settled 0.1.3 to beta, then stable, with `promote.ps1`.
- The installer closes after copying instead of showing a finish page. Fix in
  the NSIS hook if it keeps bothering people.
- No code signing. The updater signature proves the project built the installer;
  it does not satisfy SmartScreen.
- macOS menu-bar build is the next milestone in `ROADMAP.md`. The core is
  platform neutral; the host is Windows only. Signing needs a Mac.
- The GitHub wiki repo was never created, so `wiki/*.md` is only in the repo.
  Save one page in the web UI, then clone `NepaliTools.wiki.git` and push the
  pages.
- Years after 2090 are single-source projections and stay labelled as such until
  a gazette exists.

## Where things are

- Core engine: `crates/core` (conversion, formatting, Devanagari digits, tests).
- Host: `crates/host/src/main.rs` (tray, settings, commands, updater), panel in
  `crates/host/ui`.
- Data: `data/bs-months.csv` plus `data/SOURCES.md`; refresh with
  `tools/fetch-bs-data.ps1`, the only build-time network step.
- Evidence: `docs/evidence/`, indexed by `ACCEPTANCE.md`.
- Release mechanics: `docs/RELEASING.md`.
- Brand: `assets/brand/BRAND.md`.

## Machine notes

These cost hours once, all on this Windows box:

- PowerShell 5.1 `ConvertTo-Json -Depth` hangs once an object carries a multi-KB
  string. `tools/release.ps1` encodes the manifest by hand instead.
- `Compress-Archive` was replaced by `ZipFile::CreateFromDirectory`; it stalls on
  a 5 MB folder.
- Use `certutil -hashfile <file> SHA256`; `Get-FileHash` is fine but the release
  script keeps one hashing path.
- The panel cannot be driven by CDP unless it starts with
  `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9223`. A normal
  autostart launch has no port.
- Capturing the panel: a visible console steals focus and the panel hides.
  Run `tools\ui-step.ps1` with `-WindowStyle Hidden`. `tools\capture-panel.ps1`
  re-shows a hidden panel, which resets its scroll to the top.
- The settings screen is taller than the panel. To photograph the update card,
  hide the rows above it with inline styles, then capture.
- `python -m http.server` fails to bind some ports here (WinError 10013). 8811
  and 8813 work.
- WebView2 `157.0.4341.x`, 125% scaling, Windows 11 26200. The tray badge is
  chosen from the scale, and the log prints which asset it picked.
