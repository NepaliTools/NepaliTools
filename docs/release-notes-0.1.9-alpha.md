# NepaliToolkit 0.1.9 alpha

Half फ joins again.

## What changed

Words such as आफ्नो and आफ्नै were converted with `ˆ` for the half फ. In Preeti
tables that key means फ्, but Sagarmatha draws it as a circumflex, so the word
came out as आ + a small mark + नो.

The converter now writes फ followed by an explicit halant (`cfkm\gf]`), which the
font joins into आफ्नो. Every other half letter already used its uppercase key and
was unaffected.

## Update

Open Settings, keep Alpha selected and press Check, then Install and restart.

## Download

- [Installer](https://github.com/NepaliTools/NepaliToolkit/releases/download/v0.1.9-alpha/NepaliToolkit-0.1.9-alpha-win64-setup.exe).
  Per-user, no administrator.
- [Portable zip](https://github.com/NepaliTools/NepaliToolkit/releases/download/v0.1.9-alpha/NepaliToolkit-0.1.9-alpha-win64-portable.zip).
- [SHA256SUMS](https://github.com/NepaliTools/NepaliToolkit/releases/download/v0.1.9-alpha/SHA256SUMS.txt).

Both need WebView2. The build is unsigned, so Windows may show "Windows protected
your PC"; choose More info, then Run anyway.

## Known limits

- Some fonts use `m` after a ra-foot for a ligature that no public table
  documents. Those few tokens keep a literal `m`.
- Typographic dashes and curly quotes pass through unchanged.
- The shortcut only preserves the text part of the clipboard.
- Elevated windows and applications that refuse synthetic input do not accept it.
- The build is unsigned.
- Windows only. macOS is the next milestone.

MIT. See [LICENSE](https://github.com/NepaliTools/NepaliToolkit/blob/main/LICENSE).
