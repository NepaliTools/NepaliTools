# Handoff: NepaliToolkit 0.1.2 released

Date: 2026-11-22
Toolkit: `C:\Users\aaxyat\Documents\Projects\NepaliToolkit` (branch `main`)
Site: `C:\Users\aaxyat\Documents\Projects\nepalitools-site` (branch `main`)

## State

0.1.2 is published and live.

- Release: https://github.com/NepaliTools/NepaliTools/releases/tag/v0.1.2-alpha
  (prerelease). Assets: signed installer 5,221,935 bytes, portable zip 4,691,879
  bytes, `latest.json`, `.sig`, `SHA256SUMS.txt`.
- Site deployed at commit `03848de6`, manifest committed as `3cf1ebd`:
  `https://nepalitools.pages.dev/updates/latest.json` serves version 0.1.2 and
  points at the GitHub installer asset.
- The installed 0.1.2 app, checking the production endpoint, logs
  `update: checking` then `update: up to date` and shows "You are on the latest
  alpha."
- The site's install section serves the 0.1.2 links; the 0.1.0 links are gone.

Verified after deploy: release page 200, all four asset URLs 200 with expected
sizes, live manifest 200 with version 0.1.2, site 200 with `v0.1.2-alpha` and no
`v0.1.0-alpha`.

## How it was released

`tools/release.ps1 -Version 0.1.2` built and signed, then hung while hashing.
`Get-FileHash` does not return on this machine; the script now uses
`certutil -hashfile` for the checksums. Because of the hang, the manifest,
checksums, release and deploy were finished by hand in the same shape the script
produces. A future run of the script should now complete on its own.

The script needs: the updater key at `$HOME\.tauri\nepalitoolkit.key` (empty
password), `gh` with `repo` scope, and the site checkout at
`..\nepalitools-site`. See `docs/RELEASING.md`.

## The updater

- `tauri-plugin-updater` 2.14.1, endpoint
  `https://nepalitools.pages.dev/updates/latest.json`, `requireSignedVersion`,
  Windows `installMode: quiet` (`/S /R`: no installer window, app restarts).
- Settings has an Updates card. The check is manual only. `check_update` and
  `install_update` are in `crates/host/src/main.rs`; the UI is
  `crates/host/ui`.
- Public key in `crates/host/tauri.conf.json`. Private key
  `C:\Users\aaxyat\.tauri\nepalitoolkit.key`: copy to `~/.tauri/` on the Mac and
  back it up offline. Losing it means existing installs cannot verify a new
  release.
- Evidence: `docs/evidence/updater-available.png`, `updater-current.png`,
  `panel-settings-en.png`; `ACCEPTANCE.md` rows 18 to 22.

## Remaining

- Row 21 of `ACCEPTANCE.md` cites `docs/evidence/updater-portable.png`, which
  does not exist yet. Launch the 0.1.2 portable exe, open Settings and capture
  the card marked `portable` with no Install button, then add the file.
- The installer still closes after copying instead of showing a finish page.
- Windows code signing is still absent; the updater signature is not SmartScreen
  trust.
- The 0.1.0 release cannot update itself. Its users install 0.1.2 once.
- macOS menu-bar build is the next milestone in `ROADMAP.md`. The Rust core is
  platform-neutral; the host is Windows-specific today.

## Machine notes

- `Get-FileHash` hangs indefinitely in this shell. Use
  `certutil -hashfile <file> SHA256`.
- The panel can only be driven through WebView2's remote debugger when the app is
  started with `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9223`;
  a normal autostart launch has no port. Helpers: `tools/cdp-eval.ps1`,
  `tools/capture-panel.ps1`, `tools/ui-step.ps1`.
- `python -m http.server` binds fail on some ports here (WinError 10013); 8811
  and 8813 worked.
