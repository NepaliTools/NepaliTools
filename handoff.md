# Handoff: NepaliToolkit updater, stopped before publish

Date: 2026-11-21
Toolkit: `C:\Users\aaxyat\Documents\Projects\NepaliToolkit` (branch `main`)
Site: `C:\Users\aaxyat\Documents\Projects\nepalitools-site`

## Where it stopped

The 0.1.2 release run was cancelled after the signed installer and portable zip
were built, and before any GitHub release, tag or site deploy. Nothing is
published.

What exists locally:

- The updater is implemented, built and tested end to end: an internal 0.1.1
  build checked the site manifest, installed the signed 0.1.2 installer quietly
  through `/S /R`, and restarted as 0.1.2. The 0.1.2 app is installed on this
  Windows machine and shows "You are on the latest alpha" against the site's
  current manifest.
- `dist/release/` has `NepaliToolkit-0.1.2-alpha-win64-setup.exe` (5,221,005
  bytes), its `.sig`, and `NepaliToolkit-0.1.2-alpha-win64-portable.zip`. The
  `SHA256SUMS.txt` there is still the 0.1.0 one and `latest.json` is missing,
  because the run stopped before those steps. Do not use the stale checksums;
  rerun the release script.
- The toolkit source, docs, screenshots and release script are committed in the
  local commit below. The site's 0.1.2 docs and screenshots are committed in its
  own local commit. Neither repository was pushed.

## Live state

- GitHub has only `v0.1.0-alpha`. There is no `v0.1.2-alpha` release or tag.
- The live site still installs 0.1.0. Its `/updates/latest.json` is the test
  manifest that points at `/updates/test/NepaliToolkit_0.1.2_x64-setup.exe`; that
  test file was removed. The only updater-enabled build in the wild is the local
  0.1.2 install, which sees the manifest's 0.1.2 and offers nothing. Do not hand
  out the internal 0.1.1 build against this manifest: its download would 404.
- The site checkout has the 0.1.2 install links and docs, but they were not
  deployed. Deploying them before the 0.1.2 release exists would produce 404
  install links.

## The updater

- `tauri-plugin-updater` 2.14.1; endpoint
  `https://nepalitools.pages.dev/updates/latest.json`; `requireSignedVersion`;
  Windows `installMode: quiet` (`/S /R`: no installer window, app restarts).
- Settings has an Updates card. The check is manual only; there is no startup or
  timer check. The app makes one network request, and only when the button is
  pressed. `check_update` and `install_update` live in
  `crates/host/src/main.rs`; the UI is `crates/host/ui`.
- The public key is in `crates/host/tauri.conf.json`. The private key is
  `C:\Users\aaxyat\.tauri\nepalitoolkit.key` (348 bytes, empty password). Copy it
  to the Mac at `~/.tauri/nepalitoolkit.key` and back it up offline. It is the
  update trust root: losing it means existing installs cannot verify a new
  release, and rotating it means those users must reinstall by hand. Never commit
  it.
- `tools/release.ps1` builds, signs, writes `latest.json` and `SHA256SUMS`,
  creates the GitHub prerelease and deploys the site. It needs Windows for the
  NSIS bundle, `gh` authenticated with `repo` scope, the key above, and the site
  checkout at `..\nepalitools-site`.
- Evidence: `docs/evidence/updater-available.png`,
  `docs/evidence/updater-current.png`, `docs/evidence/panel-settings-en.png`;
  tests are recorded in `ACCEPTANCE.md` rows 18 to 22. The site uses the same
  settings screenshot with `?v=5`.

## Resume on Windows

1. `gh auth status` and confirm the site checkout is clean.
2. `powershell -ExecutionPolicy Bypass -File tools\release.ps1 -Version 0.1.2`
3. The script rebuilds and signs, writes the final `latest.json` and checksums,
   creates `v0.1.2-alpha`, uploads the installer, portable zip, signature,
   manifest and checksums, and deploys the site.
4. Verify: the release page; `https://nepalitools.pages.dev/updates/latest.json`
   points at the GitHub asset; the 0.1.2 app says it is current; an internal
   0.1.1 build can check and install 0.1.2.
5. The site's install links are 0.1.2. The README, wiki, CHANGELOG, ROADMAP,
   SECURITY and release notes already describe it.

## On the Mac

- The app and the NSIS release are Windows-only. The Mac can hold the repo, edit
  the Rust core, docs and site, and run the data scripts, but signing and
  bundling the Windows installer needs Windows or a Windows CI runner. The
  current GitHub Actions workflow only runs tests and a release `cargo build`; it
  does not bundle or sign.
- The site is static and deploys from any machine with Wrangler and the
  Cloudflare credentials. Do not deploy the local site until the 0.1.2 release
  exists.
- The commits are local. Push both repositories before switching, or clone after
  pushing:
  - `NepaliTools/NepaliTools` (public)
  - `NepaliTools/nepalitools-site` (private)
- The private updater key does not travel with the repository. Copy it
  separately, and keep an offline backup.

## Open items

- Publish 0.1.2 with `tools/release.ps1` and verify the update path from an
  internal 0.1.1 build.
- Point the live manifest at the published 0.1.2 release, or leave the site at
  0.1.0 until the release exists.
- Test the portable 0.1.2 UI: it should mark itself `portable` and never offer
  the installer. Add the screenshot to `ACCEPTANCE.md` row 21 when done.
- The 0.1.0 release has no updater. Its users install 0.1.2 once; 0.1.2 is the
  first build whose Settings can update.
- The installer still closes after copying instead of showing a finish page;
  this is unchanged from 0.1.0.
- Windows code signing is still absent. The updater signature proves the
  installer came from this project's key; it does not satisfy SmartScreen.

## Decisions worth keeping

- The site is the stable manifest endpoint because GitHub's `/releases/latest`
  ignores prereleases. The release assets still live on GitHub.
- The manifest and installer are signed with the same Tauri minisign key. The
  manifest is not itself signed, so `requireSignedVersion` binds the version in
  the signature to the version in the manifest.
- The updater is manual by request. No background polling, no silent download.
  The install itself is silent once the user asks for it.
- The portable build is deliberately not offered the installer; it cannot replace
  itself without turning into an installed copy.
