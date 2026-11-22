# Releasing

Every release that users can update from must be signed with the same Tauri
updater key. The public key lives in `crates/host/tauri.conf.json`. The private
key lives outside the repository, at `$HOME\.tauri\nepalitoolkit.key` on the
author's machine. It was generated without a password for the local alpha; if a
password is added later, set `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` for the build.

Losing the private key means existing installs cannot verify a new release. They
would have to download a new installer by hand. Back the key up somewhere safe
and offline. Never commit it, paste it into an issue, or put it in `TAURI_*`
variables that a CI log can print.

## Channels

The app offers **Stable**, **Beta** and **Alpha** in Settings. Each one reads a
separate static manifest:

```
https://nepalitools.pages.dev/updates/stable.json
https://nepalitools.pages.dev/updates/beta.json
https://nepalitools.pages.dev/updates/alpha.json
```

`updates/latest.json` always mirrors alpha. It exists because 0.1.2, the first
build with an updater, reads that one file and has no channel setting.

A channel is a promotion track, not a version line. Every release is built and
published to alpha first:

```powershell
powershell -ExecutionPolicy Bypass -File tools\release.ps1 -Version 0.1.4 -Channel alpha
```

When a build has been out long enough, promote the same signed installer without
rebuilding:

```powershell
powershell -ExecutionPolicy Bypass -File tools\promote.ps1 -From alpha -To beta
powershell -ExecutionPolicy Bypass -File tools\promote.ps1 -From beta -To stable
```

Promotion copies the manifest, so the version, URL and signature do not change.
Stable therefore never receives bytes that did not first ship on alpha. A channel
with no manifest yet is not an error: the app logs `no manifest on channel beta`
and shows "No Beta release yet".

Keep the version numbers increasing across all channels. A user who drops from
alpha to stable is offered nothing until stable passes the version they already
have; the updater never downgrades.

## Prepare

1. Bump the version in `Cargo.toml` and `crates/host/tauri.conf.json`. They must
   match; the updater compares the installed app version with the manifest.
2. Move the changelog section from `[Unreleased]` to a dated version.
3. Update the install links and version in `README.md`, `wiki/Installation.md`,
   `docs/release-notes-<version>-alpha.md`, and the site.
4. Run `cargo test --workspace` and `cargo check -p nepalitoolkit`.

## Build and publish

The release script builds, signs, writes the manifest, creates the GitHub
release, and deploys the site:

```powershell
powershell -ExecutionPolicy Bypass -File tools\release.ps1 -Version 0.1.3
```

`-Channel` defaults to `alpha`; the tag becomes `v0.1.3-alpha`, or `v0.2.0` for a
stable release. `-NotesFile` defaults to
`docs\release-notes-<version>-<channel>.md`.

It expects:

- `docs/release-notes-0.1.3-alpha.md`;
- the private key at `$HOME\.tauri\nepalitoolkit.key`;
- the site checkout at `..\nepalitools-site` (override with `-Site`);
- an authenticated `gh` with `repo` scope.

It writes `target/release/bundle/nsis/NepaliToolkit_0.1.3_x64-setup.exe` and
`.sig`, then puts these in `dist/release/`:

- `NepaliToolkit-0.1.3-alpha-win64-setup.exe` (the signed installer, renamed);
- `NepaliToolkit-0.1.3-alpha-win64-portable.zip`;
- `latest.json`;
- `SHA256SUMS.txt`.

`latest.json` points at the GitHub release asset and carries the signature from
the `.sig` file. The same JSON is written to `nepalitools-site/updates/latest.json`
and deployed to `https://nepalitools.pages.dev/updates/latest.json`, which is the
endpoint baked into the app. The site is the stable endpoint because GitHub's
`/releases/latest` ignores prereleases.

`-SkipPublish` builds and prepares everything without creating the release.
`-SkipSiteDeploy` writes the manifest without deploying the site.

## Verify an update

1. Install the previous version.
2. Open Settings, press Check for updates.
3. Confirm the version, press Install and restart.
4. Confirm the new version in Settings and that the log has a fresh
   `start: lang=...` line after the process restarts.

The first test of this path used an internal 0.1.1 build and a site-hosted copy of
the 0.1.2 installer; the signed bytes were then published unchanged in the 0.1.2
GitHub release.

## Roll back

The update manifest is a static file, so a bad release can be stopped by pointing
`updates/latest.json` back at the previous release, or by removing the `version`
entry. Deleting the GitHub release is not enough; the site manifest is what apps
read.

Do not enable `allowDowngrades` to roll users back. The version comparator keeps
downgrades out on purpose. Publish a fixed, higher version instead. A user who
already installed the bad build can only move forward.

The updater signature is not a code-signing certificate. Windows SmartScreen still
warns because the exe is unsigned. The signature proves the installer came from
this project's key; it does not prove the publisher to Windows.

## Key rotation

Rotating the updater key breaks existing installs, because they pin the old public
key. If a rotation becomes necessary, publish one more release signed with the old
key whose notes tell users to install the new build manually, then ship the new
key in that build.
