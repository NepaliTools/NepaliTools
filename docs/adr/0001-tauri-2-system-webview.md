# Tauri 2 with a system webview instead of a pure-Rust UI toolkit

Status: accepted

The app must ship on Windows and macOS and render Devanagari correctly. Tauri 2
keeps the shell in Rust (tray icon, window, settings, data, modules) and draws the
panel as plain HTML/CSS in the operating system's webview: WebView2 on Windows,
WKWebView on macOS. There is no frontend framework and no bundler.

The webview handles Devanagari shaping and system font fallback. Tauri supplies the
window materials (Mica and Acrylic on Windows 11, vibrancy on macOS) and the tray
plumbing, and it does not bundle Chromium. An app that draws `कात्तिक` as
disconnected glyphs is broken for its users, and webview text layout avoids that
risk.

## Considered options

- egui: pure Rust and small, but Indic shaping arrived recently (harfrust in 0.35,
  system font fallback in 0.37), and borderless tray popovers need tuning on each
  platform.
- Slint: no complex-script shaping, so Devanagari is out.
- Native AppKit and Win32 widgets: the most native result, at roughly double the
  UI work, and the macOS half cannot be tested from Windows.

## Consequences

- The panel is a web page. JavaScript must stay thin and hold no domain logic.
  Date arithmetic lives in Rust.
- Windows needs the WebView2 runtime, which ships on Windows 11. Tauri can include
  a bootstrapper.
- Moving to a canvas toolkit later would rewrite the UI layer only, not the Host
  or the date core.
