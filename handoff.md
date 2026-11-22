# Handoff: NepaliToolkit

Updated 2026-11-22. Toolkit `main` and site `main`; run `git log -1` for the
exact heads.

A Windows tray app for Bikram Sambat: today's Nepali date in Devanagari on the
icon, a BS and AD converter, a Preeti and Unicode text converter, English or
Nepali interface, no telemetry. Rust core with no dependencies, Tauri 2 host,
plain HTML and CSS panel.

## Released

| Version | Channel | Date | Notes |
|---|---|---|---|
| 0.1.10 | alpha | 2026-11-22 | Joined half फ (आफ्नो); Preeti in Sagarmatha, Unicode in Kalimati. Current. |
| 0.1.9 | alpha | 2026-11-22 | Half फ attempt with an explicit halant. Superseded. |
| 0.1.8 | alpha | 2026-11-22 | Preeti in Sagarmatha, Unicode in Kalimati. |
| 0.1.7 | alpha | 2026-11-22 | Real-document Preeti variants and punctuation. |
| 0.1.6 | alpha | 2026-11-22 | Correct ra-foot and ru glyphs. |
| 0.1.5 | alpha | 2026-11-22 | Text module and the settings migration fix. |
| 0.1.4 | alpha | 2026-11-22 | Text module; reset settings saved by 0.1.3. Superseded. |
| 0.1.3 | alpha | 2026-11-22 | Update channels: Stable, Beta, Alpha. |
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
- Portable builds never install; the card says to download the new zip.
- `updates/latest.json` mirrors alpha for 0.1.2, the first build with an updater
  and no channel setting.

Channels are promotion tracks, not version lines. A build ships to alpha first;
promotion copies the manifest, so the bytes and signature never change:

```powershell
powershell -ExecutionPolicy Bypass -File tools\promote.ps1 -From alpha -To beta
powershell -ExecutionPolicy Bypass -File tools\promote.ps1 -From beta -To stable
```

An empty channel answers 404, which the app shows as "No Beta release yet".

## Text module

- `npt_core::preeti` converts both ways: `preeti_to_unicode`, `unicode_to_preeti`,
  `detect`. The key table is `data/preeti-map.tsv`, vendored from an MIT source
  and filtered to keys an independent converter confirmed. Provenance is in
  `data/SOURCES.md`.
- `data/preeti-corpus.tsv` holds 126 pairs; 26 are real-document variants marked `preeti`. `crates/core/tests/preeti.rs` replays
  them; rows marked `unicode` are extensions (mixed English, independent ऐ) and
  are skipped by the reverse round-trip.
- Unrepresentable characters are left in place and returned in
  `Conversion.unconverted`; the panel lists them.
- Unicode to Preeti uses the ra-foot for a consonant plus र (`k|`, `s|`, `6«`) and the dedicated रु and रू glyphs (`?`, `¿`). A half letter plus a separate र draws with a visible halant in the font.
- The in-place shortcut copies the selection, converts, pastes and restores the
  clipboard. It waits for the hotkey modifiers to be released first, or the
  target sees Alt+C. It refuses while the panel has focus. Only clipboard text is
  restored.
- Mode lives in `settings.shortcut_mode` (`auto` or `split`). Auto registers
  `Ctrl+Alt+P`; split adds `Ctrl+Alt+U`.

## Releasing

Requirements: the updater key at `$HOME\.tauri\nepalitoolkit.key` (empty
password), `gh` with `repo` scope, and the site checkout at `..\nepalitools-site`.

1. Bump `version` in `Cargo.toml` and `crates/host/tauri.conf.json` (they must
   match).
2. Write `docs/release-notes-<version>-<channel>.md`.
3. Update install links in `README.md`, `wiki/Installation.md` and the site.
4. `cargo test --workspace`.
5. Commit and push **before** releasing. `gh release create` tags the remote
   default branch tip, not the working tree; if the tag lands on the previous
   commit, move it with
   `gh api --method PATCH repos/NepaliTools/NepaliTools/git/refs/tags/<tag> -f sha=<sha> -F force=true`.
6. Run `tools\release.ps1 -Version 0.2.1 -Channel alpha`. It builds, signs,
   writes the manifest and checksums, creates the release, and deploys the site.

The updater key is the trust root. Losing it means existing installs cannot
verify a new release. Keep an offline backup and never commit it.

## Next

- Promote a settled 0.1.4 to beta, then stable, with `promote.ps1`.
- The installer closes after copying instead of showing a finish page.
- No code signing. The updater signature is not SmartScreen trust.
- Only Preeti is supported. Kantipur, Sagarmatha and the other legacy fonts need
  their own tables; the corpus and rules would carry over.
- macOS menu-bar build is the next milestone in `ROADMAP.md`. Signing needs a Mac.
- The GitHub wiki repo was never created, so `wiki/*.md` is only in the repo.
  Save one page in the web UI, then push the pages.
- Years after 2090 are single-source projections.

## Where things are

- Core: `crates/core` (`lib.rs` for dates, `preeti.rs` for text), tests in
  `crates/core/tests/preeti.rs`.
- Host: `crates/host/src/main.rs` (tray, settings, commands, updater, in-place
  shortcut), panel in `crates/host/ui`.
- Data: `data/bs-months.csv`, `data/preeti-map.tsv`, `data/preeti-corpus.tsv`,
  provenance in `data/SOURCES.md`.
- Evidence: `docs/evidence/`, indexed by `ACCEPTANCE.md`.
- Release mechanics: `docs/RELEASING.md`. Decisions: `docs/adr/`.

## Machine notes

These cost hours once, all on this Windows box:

- PowerShell 5.1 `ConvertTo-Json -Depth` hangs once an object carries a multi-KB
  string. `tools/release.ps1` encodes the manifest by hand.
- `Compress-Archive` was replaced by `ZipFile::CreateFromDirectory`.
- Use `certutil -hashfile <file> SHA256`; `Get-FileHash` is avoided.
- The panel cannot be driven by CDP unless it starts with
  `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9223`.
- Capturing the panel: a visible console steals focus and the panel hides. Run
  `tools\ui-step.ps1` with `-WindowStyle Hidden`.
- Global chord injection needs the modifiers released first; `wait_for_modifiers`
  handles it. Injecting while Alt is held produces Alt+C and copies nothing.
- Raising another window from a background script needs AttachThreadInput plus
  SetForegroundWindow; a plain click only works if the target is already on top.
- Windows 11 Notepad is tabbed. Opening a file may activate a tab in the user's
  existing window, so any scripted typing must check the exact window title first.
- `Get-Process | Where MainWindowTitle` can match a launcher PID that exits; find
  windows by title, not by the PID returned from Start-Process.
