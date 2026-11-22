# V1 acceptance record

Environment: Windows 11 Pro Education 10.0.26200, 1920x1080 at 125% scaling,
`rustc 1.100.0` (x86_64-pc-windows-msvc), WebView2 `157.0.4341.60`.

Build: `npx --yes @tauri-apps/cli@^2 build --bundles nsis` with
`TAURI_SIGNING_PRIVATE_KEY` set, which produces
`target/release/bundle/nsis/NepaliToolkit_0.1.2_x64-setup.exe` and its `.sig`,
plus `cargo build --release -p nepalitoolkit` for the portable executable.

## Automated

`cargo test` runs 12 tests and all pass (`crates/core/tests/conversion.rs`,
`crates/core/src/lib.rs`):

- every AD day in the supported span round-trips AD to BS to AD, 46,022 days,
  and the BS sequence is strictly increasing;
- every BS day from 1975 to 2100 round-trips BS to AD to BS, 46,022 days;
- anchors match: 1975-01-01 BS is 1918-04-13 AD, 2000 is 1943-04-14, 2082 is
  2025-04-14, 2083 is 2026-04-14, and 2083-08-05 is 2026-11-21;
- weekdays, Nepali and English formatting, Devanagari digits and the tray
  tooltip strings are correct;
- invalid and out-of-range dates return errors and never clamp;
- the 27 disputed months and 120 single-source tail rows in
  `data/overrides.csv` match the four committed source tables exactly, and every
  recorded choice is justified.

## Manual checklist

| # | Item | How | Result | Evidence |
|---|------|-----|--------|----------|
| 1 | Portable build runs, single instance | launch `dist/NepaliToolkit.exe` twice and count processes | PASS. One process; the second launch shows the panel. | log line `second instance: showing panel` |
| 2 | Tray badge, single digit | overflow flyout on 2026-11-21 | PASS. `५` on the crimson tile. | `docs/evidence/tray-day5.png` |
| 3 | Tray badge, two digits | run with `NPT_TODAY=2026-11-30` (BS 2083-08-14) | PASS. `१४` is legible. | `docs/evidence/tray-day14.png` |
| 4 | Per-DPI rendering | startup log on this 125% display | PASS. Log reads `tray icon: 20px at scale 1.25 for day 5`; assets are rendered at 16, 20, 24 and 32 px, never downscaled from 32. | `docs/evidence/tray-badges-per-dpi.png` |
| 5 | Tooltip | hover the icon in the overflow | PASS. `मंसिर १४, २०८३ - सोमबार`. | `docs/evidence/tray-tooltip.png` |
| 6 | Header and Today | `--show` | PASS. `मंसिर ५, २०८३`, `शनिबार · २१ नोभेम्बर २०२६`. | `docs/evidence/panel-today.png` |
| 7 | AD to BS | date input 2026-11-21 | PASS. `मंसिर ५, २०८३`. | `docs/evidence/panel-converter.png` |
| 8 | BS to AD, live | change month `मंसिर` to `पुष` | PASS. `२० डिसेम्बर २०२६`, and the AD field follows. | `docs/evidence/panel-converter-live.png` |
| 9 | Copy | click copy and compare the clipboard | PASS. Clipboard equals `मंसिर ५, २०८३`. | driver reported `match` |
| 10 | Module disable | turn Convert off | PASS. Tab disappears, segmented control hides, and `settings.json` records `converter: false`. | `docs/evidence/panel-modules-off.png` |
| 11 | Language switch | English to Nepali | PASS. All labels switch, and the BS date stays `मंसिर ५, २०८३` in both languages. | `docs/evidence/panel-today-en.png`, `panel-today.png` |
| 12 | Autostart | toggle off, then on, reading `HKCU\...\Run` | PASS. Value removed, then restored with the path to `dist\NepaliToolkit.exe`. | registry query |
| 13 | No background network | `cargo tree -p nepalitoolkit -i reqwest` plus a startup with no button press | PASS. `reqwest` arrives through `tauri-plugin-updater`; the only call sites are the `check_update` and `install_update` commands, both invoked from the Check and Install buttons. Startup and the day watcher make no request, and the log shows no `update:` line until the button is pressed. | `cargo tree` output; app log |
| 14 | Midnight rollover | start with `NPT_CLOCK_OFFSET_SECS` placing the clock at 23:59:50, then leave it running | PASS. At 00:00:00 the watcher logged `day changed: 2083-08-06`, refreshed the badge and tooltip, and the open panel switched to `मंसिर ६` without a restart. | log at 10:58:42; `docs/evidence/panel-after-midnight.png` |
| 15 | Out-of-range error | set `2050-01-01` through the WebView debugger and dispatch `change` | PASS. Results cleared to a dash placeholder, red message `समर्थित दायरा बाहिरको मिति हो। 1918-04-13 - 2044-04-13`, DOM `hidden=false`. | `docs/evidence/panel-error.png` |
| 16 | Nepal-date override | toggle on, then off, watching `settings.json` and the tray refresh | PASS for the stored state. The setting persists and is off by default; this machine's zone is already UTC+05:45, so the two clocks agree by definition. | `settings.json` |
| 17 | English primary, Nepali date | launch with `lang: en` | PASS. Chrome, weekday and AD date are English; the BS date `मंसिर ५, २०८३` is Devanagari in the header, Today, converter result and tray tooltip, and the BS pickers show `२०८३` / `मंसिर` / `५`. | `docs/evidence/panel-today-en.png`, `panel-converter-en.png`; tooltip log contains `Saturday` and 10 Devanagari characters |
| 18 | Manual update check | an internal 0.1.1 build, with the production endpoint, press Check for updates while the site served a 0.1.2 manifest | PASS. The log read `update: checking`, then `update: 0.1.2 available`; the panel showed the version, the release notes and Install and restart. | `docs/evidence/updater-available.png`; app log |
| 19 | Quiet install and restart | press Install and restart in the 0.1.1 build | PASS. The app exited, no installer window appeared, the 0.1.2 process started through `/S /R`, the installed exe reports FileVersion 0.1.2, and the log shows a fresh `start:` line. | process output; installed version; app log |
| 20 | Up to date | press Check for updates in the 0.1.2 build against the 0.1.2 manifest | PASS. The panel read `Current version 0.1.2` and `You are on the latest alpha.`; the log read `update: up to date`. | `docs/evidence/updater-current.png`; app log |
| 21 | Portable is not replaced | run the portable 0.1.2 exe and open Settings | PASS. The update card marks the build `portable`; the check works, and the Install button only exists for an installed build, so a portable copy is never overwritten by the NSIS updater. | `docs/evidence/updater-portable.png` |
| 22 | Signature and version binding | inspect the release `.sig`, `latest.json` and `tauri.conf.json` | PASS. The installer is signed with the Tauri minisign key; the manifest carries that signature; the app pins the public key and sets `requireSignedVersion` plus `windows.installMode: quiet`. | `.sig`; `updates/latest.json`; `crates/host/tauri.conf.json` |
| 23 | Channel default and memory | fresh `settings.json` with no `channel`, then pick Beta, quit, relaunch | PASS. A missing or unknown value loads as `alpha`; picking Beta writes `"channel": "beta"` and the select shows Beta after relaunch. The debug run above ended back on `alpha`. | `settings.json`; `docs/evidence/updater-channel-alpha.png` |
| 24 | Channel selects the manifest | press Check on each channel against the live site | PASS. Alpha reads `updates/alpha.json` and logs `update: up to date on alpha`; beta and stable return 404, which the app logs as `no manifest on channel ...` and shows as its own state, not an error. | log lines; `docs/evidence/updater-channel-empty.png` |
| 25 | Empty channel wording | Beta and Stable with no release published | PASS. The card reads `No Beta release yet.` and `No Stable release yet.` instead of claiming the build is current. | `docs/evidence/updater-channel-empty.png` |
| 26 | Channel dropdown legibility | open the Channel select in dark mode | PASS. The popup rows are dark with light text, so Stable, Beta and Alpha all stay readable; the selected row is highlighted. Before the fix the rows were white with inherited light text. | `docs/evidence/panel-select-popup.png` |
| 27 | Text module, Preeti to Unicode | type `g]kfn` in the Text tab with Preeti to Unicode selected | PASS. The output box shows नेपाल live, without a convert button. | `docs/evidence/panel-text-unicode.png` |
| 28 | Text module, Unicode to Preeti | switch direction and type नेपाल | PASS. The output box shows `g]kfn`; Swap moves text between the boxes and flips the direction. | `docs/evidence/panel-text-preeti.png` |
| 29 | Converter corpus | `cargo test -p npt-core --test preeti` | PASS. 81 pairs in `data/preeti-corpus.tsv` convert Preeti to Unicode exactly, and every row representable in Preeti round-trips Unicode to Preeti to Unicode. | test output; `data/preeti-corpus.tsv` |
| 30 | Unrepresentable characters | convert नेपाल ॲ to Preeti | PASS. ॲ is left in the output and listed under the box as "Characters Preeti cannot write: ॲ". Nothing is silently substituted. | `docs/evidence/panel-text-unconverted.png` |
| 31 | Shortcut in Notepad | select `g]kfn` in a Notepad tab, press Ctrl+Alt+P, read the selection back | PASS. The tab holds नेपाल, and the clipboard is the sentinel value that was there before the chord. | log `text: to_unicode 5 chars -> 5 chars`; `tmp/notepad-shortcut.ps1` run |
| 32 | Shortcut in a Chromium browser | select Preeti and Devanagari text in Chrome, press the chord in auto and split mode | PASS. `g]kfn` became नेपाल and नेपाल became `g]kfn`; the page title flipped to PASS in both runs and the clipboard was restored. | log lines at 11:46 and 11:48; `tmp/raise-and-chord.ps1` |
| 33 | Shortcut in an Electron app | select `g]kfn` in a VS Code editor tab, press Ctrl+Alt+P | PASS. The editor holds नेपाल and the clipboard is restored. | log `text: to_unicode 5 chars -> 5 chars`; `tmp/notepad-shortcut.ps1` with `-Launcher code` |
| 34 | Shortcut with no selection | press the chord with nothing selected | PASS. The app logs `text: no selection`, restores the clipboard and opens the panel with "Select text first, then press the shortcut." It never converts stale clipboard text. | log; panel message |
| 35 | Panel focus guard | open the panel, press the chord | PASS. Log reads `text: skipped, panel has focus`; the clipboard is untouched and the panel explains why. No keystrokes enter the webview. | `docs/evidence/panel-text-shortcut-guard.png` |
| 36 | Shortcut modes | switch Auto and Split in the Text tab, then press each chord | PASS. Split registers Ctrl+Alt+P and Ctrl+Alt+U and logs `mode=split`; auto unregisters U and logs `mode=auto`. The choice persists in `settings.json`. | log lines; `settings.json` |
| 37 | Text module toggle | turn Text off on the Modules page | PASS. The Text tab disappears, the segmented bar stays for the other modules, and the setting persists. | `settings.json`; screenshot |
| 38 | Settings survive an upgrade | start 0.1.5 against a 0.1.3 settings file that has no `text` flag or `shortcut_mode` | PASS. Language (np), channel (beta), Nepal date, autostart off and converter off all survived; the new module defaulted on and the mode to auto. | state read from the running app |
| 39 | Preeti glyph forms | convert प्र, रु, रू, क्र, राष्ट्र, प्रि and replay each through the reference converter | PASS. Output uses `k|`, `?`, `¿`, `s|`, `/fi6«`, `lk|`; every pair round-trips and the oracle reads back the same word. The old `K/` and `/'` forms are gone. | `crates/core/tests/preeti.rs`; `data/preeti-corpus.tsv` |

## How the midnight check was run

The production path is `seconds_to_next_midnight()`, then sleep, then compare the
BS date, then `refresh_tray` plus the `day-changed` event. Waiting 13 hours was
not practical, so the host accepts `NPT_CLOCK_OFFSET_SECS`, a test-only switch
logged as `clock offset: ...s`. The process started with the clock shifted to
23:59:50 local time, and ten seconds later the real watcher fired, logged the new
BS date, and the open panel re-rendered.

## Tray icon legibility

The first implementation rendered one 32 px canvas and let Windows downscale it.
At 16 to 20 px the two Devanagari digits collapsed. The fix renders each day at
the exact pixel size the shell asks for and picks the asset from the window
scale: 16, 20, 24 or 32. No downscale happens. All 32 days are legible at 20 px
and above, as `docs/evidence/tray-badges-per-dpi.png` shows. The tooltip still
carries the full date.

## Left to daily use

- a real wall-clock midnight, since the shifted-clock run exercised the same code;
- sleep and resume, multiple monitors, a taskbar on another edge, and 100% scaling;
- long-running memory and CPU behaviour.
