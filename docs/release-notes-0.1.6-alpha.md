# NepaliToolkit 0.1.6 alpha

Preeti output that draws correctly.

## What changed

- **The ra-foot.** A consonant followed by र is now written with the font's
  ra-foot key: `k|` for प्र, `s|` for क्र, `b|` for द्र, `a|` for ब्र. The old
  output used a half letter and a separate र (`K/`), which the font draws with a
  visible halant.
- **Retroflex letters** use their own foot glyph: ट्र, ठ्र, ड्र and ढ्र are
  `6«` and its siblings.
- **रु and रू** use their dedicated glyphs, `?` and `¿`, instead of र followed by
  a detached matra.
- **Short-i after a foot** lands after the whole cluster: प्रि is `lk|`, ट्रि is
  `l6«`.
- The corpus is now 100 pairs and includes all of these, checked against an
  independent converter and replayed by `cargo test` in both directions.

Nothing else changed. Dates, the tray badge, channels and the update logic are
the same as 0.1.5.

## Update

Open Settings, keep Alpha selected and press Check, then Install and restart.

## Download

- [Installer](https://github.com/NepaliTools/NepaliToolkit/releases/download/v0.1.6-alpha/NepaliToolkit-0.1.6-alpha-win64-setup.exe).
  Per-user, no administrator.
- [Portable zip](https://github.com/NepaliTools/NepaliToolkit/releases/download/v0.1.6-alpha/NepaliToolkit-0.1.6-alpha-win64-portable.zip).
- [SHA256SUMS](https://github.com/NepaliTools/NepaliToolkit/releases/download/v0.1.6-alpha/SHA256SUMS.txt).

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
