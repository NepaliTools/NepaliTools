# Installation

## Requirements

- Windows 10 or Windows 11.
- The WebView2 runtime. It ships with Windows 11. On older Windows 10 machines,
  install the
  [Evergreen runtime](https://developer.microsoft.com/microsoft-edge/webview2/).
- No Node.js, no npm, no separate runtime.

## Get the app

There is no installer and no binary release yet. Build the portable executable:

```powershell
git clone https://github.com/NepaliTools/NepaliTools.git
cd NepaliTools
cargo build --release -p nepalitoolkit
Copy-Item target\release\nepalitoolkit.exe dist\NepaliToolkit.exe
```

Run `dist\NepaliToolkit.exe`. It is a single file, so you can move it anywhere.

Because the build is unsigned, Windows SmartScreen may show "unknown publisher"
the first time. Choose More info, then Run anyway.

## First run

1. The app starts in the notification area. Windows 11 usually puts new icons in
   the hidden-icons overflow. Click `^` and drag the icon onto the taskbar to pin
   it.
2. Left click the icon, or press `Ctrl+Alt+N`, to open the panel.
3. Autostart is on by default. Turn it off in Settings, under Start with Windows.

The icon shows the BS day in Devanagari digits. Hovering it shows
`मंसिर ५, २०८३ - Saturday`.

## Where things live

| What | Path |
|---|---|
| Settings | `%APPDATA%\com.aaxyat.nepalitoolkit\settings.json` |
| Diagnostic log | `%APPDATA%\com.aaxyat.nepalitoolkit\app.log` |
| Autostart entry | `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`, value `NepaliToolkit` |

## Uninstall

1. Open Settings and turn off Start with Windows first.
2. Quit from the panel footer or the tray menu.
3. Delete the executable.
4. Optionally delete `%APPDATA%\com.aaxyat.nepalitoolkit\`.

Removing the app while autostart is still on leaves a Run entry pointing at a
missing file. Turning autostart off first avoids that.
