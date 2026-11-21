# Contributing to NepaliToolkit

Thanks for taking the time to improve the app. This is a small, focused codebase;
the rules below keep it that way.

## Before you start

- Open an issue before a large change (new module, data source, architecture).
  Small fixes and documentation can go straight to a pull request.
- By participating you agree to the [Code of Conduct](CODE_OF_CONDUCT.md).
- Security problems go through [SECURITY.md](SECURITY.md), never a public issue.

## Setup

- Windows 10/11, the WebView2 runtime, and Rust with the MSVC toolchain
  (`rustup default stable-x86_64-pc-windows-msvc`).
- No Node.js or npm is used. The panel is plain HTML/CSS/JS.
- PowerShell 5.1 or later for the scripts under `tools/`.

```powershell
cargo test --workspace
cargo build --release -p nepalitoolkit
```

## Ground rules

1. **Date logic lives in `crates/core`.** No calendar arithmetic in JavaScript.
   The panel renders strings the core produces.
2. **No runtime network access.** The app must stay fully offline; data is
   embedded at compile time.
3. **The UI stays dependency-free.** No bundlers, frameworks or npm.
4. **Data is generated, not hand-edited.** Change sources or rules in
   `tools/fetch-bs-data.ps1`, then regenerate and commit the CSV, overrides and
   `data/SOURCES.md` together.
5. **Architecture changes need an ADR** in `docs/adr/`, and any decision that
   changes the glossary should update `GLOSSARY.md`.

## Data changes

```powershell
powershell -ExecutionPolicy Bypass -File tools\fetch-bs-data.ps1
cargo test --workspace
```

The script verifies pinned SHA256 hashes, cross-checks each lineage's epoch,
rejects impossible month lengths or year sums, and fails on fixture drift. Commit
`data/bs-months.csv`, `data/sources/*.csv`, `data/overrides.csv` and
`data/SOURCES.md` in the same change.

## Tests and evidence

- `cargo test --workspace` must pass before a pull request.
- If you touch the panel, take a screenshot and update `docs/evidence/` if the
  visible behaviour changes.
- Test-only switches that help verification: `--show`, `NPT_TODAY=YYYY-MM-DD`,
  `NPT_CLOCK_OFFSET_SECS=<n>`, and WebView2's
  `--remote-debugging-port` with `tools/cdp-eval.ps1`. Keep them working.

## Commits and pull requests

- Use conventional commit subjects (`feat:`, `fix:`, `docs:`, `chore:`).
- One logical change per pull request; unrelated refactors go separately.
- In the pull request, say what changed, why, and how you verified it.
- The CI workflow runs `cargo test` and a release build on Windows.

## Style

- `rustfmt` defaults; no custom configuration.
- Comments explain *why*, not *what*.
- Keep user-facing strings in `crates/host/ui/app.js` (UI copy) and the core
  (dates). Never hardcode a date rule in the UI.

## Wiki

The wiki pages are versioned in `wiki/` in the main repository. To publish them,
clone the wiki repository and copy the pages in:

```powershell
git clone https://github.com/NepaliTools/NepaliTools.wiki.git
Copy-Item wiki\*.md NepaliTools.wiki\
cd NepaliTools.wiki
git add .
git commit -m "docs: sync wiki"
git push
```

Wiki links are page-relative (`[FAQ](FAQ)`); images use absolute raw URLs from
`main` so they render inside the wiki.
