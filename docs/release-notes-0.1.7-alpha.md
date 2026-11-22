# NepaliToolkit 0.1.7 alpha

Preeti conversion checked against a real 166-page report.

## What changed

The converter was run over 23,779 words of a real Preeti report, alongside an
independent converter and the MIT nepalibhasha key map. Everything that did not
survive the round trip was fixed.

- **More legacy glyphs.** The report mixes Preeti, Sagarmatha and two other old
  fonts, and each uses a few keys differently. Those keys now convert: `¬` and
  `Ò` for ु, `¢` for द्ध, `Ý` for ठ्ठ, `›` for द्र, `<` for ्र, `Ø` for ्य, `Œ`
  for त्, `·` and `∙` for ङ, `÷` for a slash, `Ü` for a percent sign, `ˆ` and
  `¥` for half फ and the reph, `æ` and `Æ` for quotes.
- **Punctuation keys.** A slash no longer becomes र, a percent no longer becomes
  ५, and a period no longer becomes a danda. Devanagari digits become Preeti
  digit keys.
- **Colons and visargas.** A standalone `M` is a colon and loses the space before
  it; an `M` that closes a word stays a visarga, so पुनः and क्रमशः still read
  correctly.
- **Vowel signs in visual order.** The bindu now moves behind its matra before the
  vowels combine, so `cf+}` is औं, and matras typed the other way round make ो
  and ौ, so बमोजिम and बेमौसमी read correctly.
- **फ.** फ and its half form convert through `km` and `ˆ`, so आफ्नो and फ्राई
  work, and `k|m` is फ्र.

The corpus is now 126 pairs, including 26 variants taken from the report. On the
whole document, only typographic dashes and curly quotes are left as they
arrived.

## Update

Open Settings, keep Alpha selected and press Check, then Install and restart.

## Download

- [Installer](https://github.com/NepaliTools/NepaliToolkit/releases/download/v0.1.7-alpha/NepaliToolkit-0.1.7-alpha-win64-setup.exe).
  Per-user, no administrator.
- [Portable zip](https://github.com/NepaliTools/NepaliToolkit/releases/download/v0.1.7-alpha/NepaliToolkit-0.1.7-alpha-win64-portable.zip).
- [SHA256SUMS](https://github.com/NepaliTools/NepaliToolkit/releases/download/v0.1.7-alpha/SHA256SUMS.txt).

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
