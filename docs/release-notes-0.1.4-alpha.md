# NepaliToolkit 0.1.5 alpha

Preeti and Unicode, in the panel and in any app.

![The Text module converting Preeti to Unicode](https://raw.githubusercontent.com/NepaliTools/NepaliToolkit/v0.1.5-alpha/docs/evidence/panel-text-unicode.png)

## What is new

- **Text module.** A third tab converts Nepali text between the legacy Preeti
  encoding and Unicode Devanagari. Type or paste on either side and the other
  follows; Copy and Swap are one click away. It runs offline.
- **Nothing is guessed.** Preeti has no key for a few characters, and some text
  cannot be represented exactly. Those characters are left untouched and listed
  under the box, so nothing silently turns into wrong Nepali.
- **`Ctrl+Alt+P` converts the selection in place.** Select text in Notepad, a
  browser or an editor, press the chord, and the selection is replaced with the
  converted text. The app copies, converts, pastes and puts your clipboard back
  the way it was.
- **Two shortcut modes.** Auto follows the script of the selection. Split mode
  gives one key per direction: `Ctrl+Alt+P` for Unicode to Preeti and
  `Ctrl+Alt+U` for the reverse. The mode is saved with the other settings.
- **The panel is never a target.** The shortcut refuses to run while the panel has
  focus and says so instead of typing into the converter box.

The corpus behind the converter is 81 pairs in `data/preeti-corpus.tsv`, each
checked against an independent MIT-licensed converter where it covers the case.
`cargo test` replays every pair in both directions. The key table's source and
hash are recorded in `data/SOURCES.md`.

This release also fixes the Channel dropdown, whose rows were white on white in
dark mode.

## Update from 0.1.3

Open Settings, keep Alpha selected and press Check, then Install and restart. The
build is signed, installs without a window and restarts itself. 0.1.4 is the
first build with the Text module; older alphas cannot update themselves, so
install once by hand.

## Download

- [Installer](https://github.com/NepaliTools/NepaliToolkit/releases/download/v0.1.5-alpha/NepaliToolkit-0.1.5-alpha-win64-setup.exe).
  Per-user, no administrator.
- [Portable zip](https://github.com/NepaliTools/NepaliToolkit/releases/download/v0.1.5-alpha/NepaliToolkit-0.1.5-alpha-win64-portable.zip).
- [SHA256SUMS](https://github.com/NepaliTools/NepaliToolkit/releases/download/v0.1.5-alpha/SHA256SUMS.txt).

Both need WebView2. The build is unsigned, so Windows may show "Windows protected
your PC"; choose More info, then Run anyway.

## Known limits

- The shortcut only preserves the text part of the clipboard. An image or rich
  content on the clipboard is lost when it runs.
- Elevated windows and applications that refuse synthetic input do not accept the
  in-place shortcut. The Text tab always works.
- The build is unsigned.
- The updater only replaces the installed copy.
- Years after 2090 remain single-source projections.
- Windows only. macOS is the next milestone.

MIT. See [LICENSE](https://github.com/NepaliTools/NepaliToolkit/blob/main/LICENSE).
