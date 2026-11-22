# NepaliToolkit 0.1.10 alpha

आफ्नो, joined the way the font draws it.

## What changed

0.1.9 wrote the half फ as फ plus an explicit halant. The font draws that with a
visible halant (आफ्‌नो), not the joined letter.

The converter now uses the fonts' own half-fa glyph, the same private-use key the
source report uses in its text, so आफ्नो and आफ्नै draw joined. The corpus gained
both words, and the Preeti box screenshot shows the joined form.

## Update

Open Settings, keep Alpha selected and press Check, then Install and restart.

## Download

- [Installer](https://github.com/NepaliTools/NepaliToolkit/releases/download/v0.1.10-alpha/NepaliToolkit-0.1.10-alpha-win64-setup.exe).
  Per-user, no administrator.
- [Portable zip](https://github.com/NepaliTools/NepaliToolkit/releases/download/v0.1.10-alpha/NepaliToolkit-0.1.10-alpha-win64-portable.zip).
- [SHA256SUMS](https://github.com/NepaliTools/NepaliToolkit/releases/download/v0.1.10-alpha/SHA256SUMS.txt).

Both need WebView2. The build is unsigned, so Windows may show "Windows protected
your PC"; choose More info, then Run anyway.

## Known limits

- The half-fa key is a private-use character, so the copied Preeti text needs a
  legacy font that maps it, as every document in this family does.
- Some fonts use `m` after a ra-foot for a ligature that no public table
  documents. Those few tokens keep a literal `m`.
- Typographic dashes and curly quotes pass through unchanged.
- The shortcut only preserves the text part of the clipboard.
- The build is unsigned. Windows only; macOS is the next milestone.

MIT. See [LICENSE](https://github.com/NepaliTools/NepaliToolkit/blob/main/LICENSE).
