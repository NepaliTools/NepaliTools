# NepaliToolkit 0.1.8 alpha

The Preeti box now reads as Nepali.

## What changed

The Text module draws each box in the font its text belongs to:

- the Preeti box uses **Sagarmatha**, so the stored keys (`k|sfz`) appear as
  प्रकाश while you type or paste them;
- the Unicode box uses **Kalimati**, the Devanagari face most Nepali readers
  expect.

Both fonts are used from the system. Nothing is bundled, and if a font is missing
the old behaviour returns: the Preeti box shows the raw keys and the Unicode box
uses the default Devanagari face.

The labels above each box still say which side is which, since both now read as
Nepali.

## Update

Open Settings, keep Alpha selected and press Check, then Install and restart.

## Download

- [Installer](https://github.com/NepaliTools/NepaliToolkit/releases/download/v0.1.8-alpha/NepaliToolkit-0.1.8-alpha-win64-setup.exe).
  Per-user, no administrator.
- [Portable zip](https://github.com/NepaliTools/NepaliToolkit/releases/download/v0.1.8-alpha/NepaliToolkit-0.1.8-alpha-win64-portable.zip).
- [SHA256SUMS](https://github.com/NepaliTools/NepaliToolkit/releases/download/v0.1.8-alpha/SHA256SUMS.txt).

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
