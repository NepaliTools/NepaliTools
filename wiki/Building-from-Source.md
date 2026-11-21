# Building from source

## Prerequisites

- Rust with the MSVC toolchain:
  `rustup default stable-x86_64-pc-windows-msvc`
- The WebView2 runtime (present on Windows 11; install the Evergreen runtime on
  older Windows 10).
- PowerShell 5.1 or later for the helper scripts.
- No Node.js or npm at any point.

## Commands

```powershell
cargo test --workspace                     # 12 tests, including the data audit
cargo build --release -p nepalitoolkit     # target\release\nepalitoolkit.exe
```

The portable build is that single file; copy it wherever you like. The UI is
plain HTML/CSS/JS under `crates/host/ui` and is embedded at compile time, so there
is no frontend build step.

## Layout

```
crates/core/       npt-core: BS conversion, formatting, no dependencies
crates/host/       Tauri 2 app: tray, panel, commands, settings
crates/host/ui/    vanilla HTML/CSS/JS panel
crates/host/src/badges.rs   generated per-DPI tray assets
data/              bs-months.csv, sources/, overrides.csv, SOURCES.md
tools/             fetch-bs-data, make-badges, make-icons, cdp-eval, ui-step, screenshot
docs/adr/          architecture decisions
docs/evidence/     acceptance screenshots
wiki/              these pages
```

## Regenerating assets

```powershell
powershell -ExecutionPolicy Bypass -File tools\make-badges.ps1   # 4 sizes × 32 days
powershell -ExecutionPolicy Bypass -File tools\make-icons.ps1    # app icon + ico
```

`make-badges.ps1` also rewrites `crates/host/src/badges.rs`. Both scripts are
ASCII-only so Windows PowerShell 5.1 cannot misread them.

## Verification switches

| Switch | Effect |
|---|---|
| `--show` | Opens the panel at startup (used for screenshots). |
| `NPT_TODAY=YYYY-MM-DD` | Pretends today is that date; logged as `date override:`. |
| `NPT_CLOCK_OFFSET_SECS=<n>` | Shifts the clock by `n` seconds; used to run the real midnight path quickly. |
| `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9223` | Enables `tools/cdp-eval.ps1` for UI checks. |

These are test aids; none is required to run the app.

## CI

`.github/workflows/ci.yml` runs `cargo test --workspace` and a release build on
`windows-latest` for every push and pull request.
