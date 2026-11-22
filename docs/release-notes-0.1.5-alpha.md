# NepaliToolkit 0.1.5 alpha

Preeti and Unicode, in the panel and in any app, with your settings intact.

![The Text module converting Preeti to Unicode](https://raw.githubusercontent.com/NepaliTools/NepaliToolkit/v0.1.5-alpha/docs/evidence/panel-text-unicode.png)

## What is new

0.1.5 contains the Text module: a third tab that converts Nepali text between the
legacy Preeti encoding and Unicode Devanagari, live in both directions, with Copy
and Swap. Characters Preeti cannot write are left untouched and listed under the
boxes instead of being guessed.

`Ctrl+Alt+P` converts the selection in any other application in place: the app
copies it, converts it, pastes it back and restores your clipboard. Auto mode
follows the script of the selection; the Text module can switch to one key per
direction instead, `Ctrl+Alt+P` for Unicode to Preeti and `Ctrl+Alt+U` for the
reverse. The shortcut refuses to run inside the panel and reports an empty
selection rather than converting old clipboard content.

It also fixes the Channel dropdown, whose rows were white on white in dark mode.

## What 0.1.5 fixes over 0.1.4

0.1.4 could reset your settings on first launch. The settings file gained a module
flag, and a file written by 0.1.3 failed to parse as a whole, so language, update
channel, Nepal date, autostart and module toggles came back as defaults. Missing
fields now take their defaults and every other saved value survives. If 0.1.4
already reset your settings once, set them again after this update; they will stay.

## Update

Open Settings, keep Alpha selected and press Check, then Install and restart.
0.1.3 and 0.1.4 can both update themselves. Anyone still on 0.1.0 installs once by
hand.

## Download

- [Installer](https://github.com/NepaliTools/NepaliToolkit/releases/download/v0.1.5-alpha/NepaliToolkit-0.1.5-alpha-win64-setup.exe).
  Per-user, no administrator.
- [Portable zip](https://github.com/NepaliTools/NepaliToolkit/releases/download/v0.1.5-alpha/NepaliToolkit-0.1.5-alpha-win64-portable.zip).
- [SHA256SUMS](https://github.com/NepaliTools/NepaliToolkit/releases/download/v0.1.5-alpha/SHA256SUMS.txt).

Both need WebView2. The build is unsigned, so Windows may show "Windows protected
your PC"; choose More info, then Run anyway.

## Known limits

- The shortcut only preserves the text part of the clipboard.
- Elevated windows and applications that refuse synthetic input do not accept it.
- The build is unsigned.
- The updater only replaces the installed copy.
- Years after 2090 remain single-source projections.
- Windows only. macOS is the next milestone.

MIT. See [LICENSE](https://github.com/NepaliTools/NepaliToolkit/blob/main/LICENSE).
