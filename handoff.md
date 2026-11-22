# Handoff: NepaliToolkit 0.1.3 released (channels)

Date: 2026-11-22
Toolkit: `C:\Users\aaxyat\Documents\Projects\NepaliToolkit` (branch `main`)
Site: `C:\Users\aaxyat\Documents\Projects\nepalitools-site` (branch `main`)

## State

0.1.3 is published and live, with the Stable, Beta and Alpha channel choice.

- Release: https://github.com/NepaliTools/NepaliTools/releases/tag/v0.1.3-alpha
  (prerelease). Assets: signed installer 5,223,040 bytes, portable zip 4,694,718
  bytes, `latest.json`, `.sig`, `SHA256SUMS.txt`.
- Live manifests: `updates/alpha.json` and `updates/latest.json` serve 0.1.3;
  `updates/beta.json` and `updates/stable.json` are absent, which the app shows as
  "No Beta release yet" rather than an error.
- Installed 0.1.3 on this machine: alpha reports "You are on the latest release",
  beta reports "No Beta release yet", and the choice persists in `settings.json`.
- Site deployed as `cb7f93e8` from the release script.

The 0.1.3 tag was created by `gh release create` against the remote default
branch at the time, which was the 0.1.2 commit, because the channel work was not
pushed before the release. Push the repo and repoint the tag before treating the
tag as the source of the release.

## How it was released

`tools/release.ps1 -Version 0.1.3 -Channel alpha` completed end to end after two
PowerShell fixes:

- `ConvertTo-Json -Depth 8` hangs indefinitely once the object carries the
  multi-KB release notes. The script now encodes the manifest with a small
  escaping helper instead.
- `Compress-Archive` was replaced by `ZipFile::CreateFromDirectory`. It was not
  the hang, but the archive is identical and faster.

## The updater

- `tauri-plugin-updater` 2.14.1, `requireSignedVersion`, Windows
  `installMode: quiet` (`/S /R`: no installer window, app restarts).
- The endpoint is chosen at runtime from the channel:
  `https://nepalitools.pages.dev/updates/{stable,beta,alpha}.json`. The endpoint in
  `tauri.conf.json` remains the fallback for 0.1.2, and `latest.json` mirrors
  alpha for that build.
- Channels are promotion tracks. A build ships to alpha first; `tools/promote.ps1
  -From alpha -To beta` copies the manifest without rebuilding, so stable never
  gets bytes that did not first ship on alpha.
- Public key in `crates/host/tauri.conf.json`. Private key
  `C:\Users\aaxyat\.tauri\nepalitoolkit.key`: copy to `~/.tauri/` on the Mac and
  back it up offline.
- Evidence: `docs/evidence/updater-channel-alpha.png`,
  `updater-channel-empty.png`, `updater-available.png`, `updater-current.png`;
  `ACCEPTANCE.md` rows 18 to 25.

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
