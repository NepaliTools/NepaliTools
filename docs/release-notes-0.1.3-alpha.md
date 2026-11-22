# NepaliToolkit 0.1.3 alpha

Choose which release channel the update check follows.

![The Channel setting and the update card](https://raw.githubusercontent.com/NepaliTools/NepaliToolkit/v0.1.3-alpha/docs/evidence/updater-channel-alpha.png)

## What is new

- **Stable, Beta and Alpha.** Settings now has a Channel choice. Each channel is
  its own manifest on nepalitools.pages.dev, so the check reads
  `updates/stable.json`, `updates/beta.json` or `updates/alpha.json`.
- **Channels are promotion tracks, not separate versions.** A build is published
  to alpha first. Moving it to beta or stable republishes the same signed
  installer under that channel, so nothing reaches stable that did not first ship
  on alpha.
- **An empty channel says so.** If the chosen channel has no release yet, the
  card reads "No Stable release yet" instead of claiming you are up to date.
- **The choice is remembered.** It is stored in `settings.json` and survives
  restarts. New installs start on alpha while every published build is an alpha.
- **Checks are still manual.** Nothing polls in the background. Switching channel
  clears the last result and waits for you to press Check.

Alpha is the only channel with a release today. Beta and stable appear the first
time a build is promoted with `tools/promote.ps1`.

## If you have 0.1.0

The 0.1.0 alpha has no updater. Install 0.1.3 once; from then on Settings handles
new builds.

## If you have 0.1.2

0.1.2 reads a single manifest and has no channel choice. Settings, Check for
updates, Install and restart works as before and updates to 0.1.3. The channel
menu arrives with that build.

## Download

- [Installer](https://github.com/NepaliTools/NepaliToolkit/releases/download/v0.1.3-alpha/NepaliToolkit-0.1.3-alpha-win64-setup.exe).
  Per-user, no administrator. Adds a Start Menu shortcut and an uninstaller.
- [Portable zip](https://github.com/NepaliTools/NepaliToolkit/releases/download/v0.1.3-alpha/NepaliToolkit-0.1.3-alpha-win64-portable.zip).
  Extract and run `NepaliToolkit.exe`.
- [SHA256SUMS](https://github.com/NepaliTools/NepaliToolkit/releases/download/v0.1.3-alpha/SHA256SUMS.txt).
- [latest.json](https://github.com/NepaliTools/NepaliToolkit/releases/download/v0.1.3-alpha/latest.json),
  the manifest, also served as `updates/alpha.json` on the site.

Both downloads need WebView2. Windows 11 includes it. The installer embeds the
WebView2 bootstrapper for older Windows 10 machines.

The build is unsigned, so Windows may show "Windows protected your PC". Choose
More info, then Run anyway.

## Checksums

`SHA256SUMS.txt` covers the installer, the portable zip, the manifest and the
signature. Verify on Windows with:

```powershell
certutil -hashfile .\NepaliToolkit-0.1.3-alpha-win64-setup.exe SHA256
```

## Known limits

- The build is unsigned.
- The updater only replaces the installed copy. Portable users download the new
  zip and keep their own folder.
- Years after 2090 remain single-source projections.
- Windows only. macOS is the next milestone.
- The installer still closes after copying instead of showing a finish page; open
  NepaliToolkit from the Start Menu after installing.

## Privacy

The date data ships inside the binary. The only network request is the update
check you start, and it fetches only the manifest for your channel and, if you
install, the signed installer. No telemetry, no account, nothing about you is
sent.

## License

MIT. See
[LICENSE](https://github.com/NepaliTools/NepaliToolkit/blob/main/LICENSE).
