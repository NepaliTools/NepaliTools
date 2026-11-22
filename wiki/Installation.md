# Installation

## Requirements

- Windows 10 or Windows 11, 64-bit.
- The WebView2 runtime. It ships with Windows 11. On older Windows 10 machines,
  install the
  [Evergreen runtime](https://developer.microsoft.com/microsoft-edge/webview2/).
- Rust with the MSVC toolchain, only if you build from source.
- No Node.js, no npm, no separate runtime.

## Get the app

Download the 0.1.8 alpha from the
[release page](https://github.com/NepaliTools/NepaliTools/releases/tag/v0.1.8-alpha).

- **Installer:** download the setup exe and run it. It installs for the current
  user, adds a Start Menu shortcut and an uninstaller, and needs no administrator.
- **Portable zip:** download, extract, and run `NepaliToolkit.exe`. It runs from
  any folder.

Both need WebView2. The installer embeds the bootstrapper. The build is unsigned,
so SmartScreen may show "Windows protected your PC"; choose More info,
then Run anyway.

Older alphas cannot update themselves. Install 0.1.8 once; from then on Settings
has Check for updates and the channel menu. The check is manual, the installer is
signed, and it runs quietly and restarts the app. The portable build asks you to
download the new zip instead.

## Update

1. Open Settings.
2. Choose a channel: Stable, Beta or Alpha.
3. Press Check. If a newer build exists on that channel, press Install and
   restart. The app downloads the signed installer, verifies it, installs it
   without opening an installer window, and starts the new version.

There is no background check and no automatic download. The only network request
is this one. Alpha receives every build; beta and stable receive the same signed
installer later, when it is promoted, so they are often empty. A channel with no
release says so instead of reporting that you are current.

## Build from source

```powershell
git clone https://github.com/NepaliTools/NepaliTools.git
cd NepaliTools
cargo build --release -p nepalitoolkit
Copy-Item target\release\nepalitoolkit.exe dist\NepaliToolkit.exe
```

Run `dist\NepaliToolkit.exe`. It is a single file, so you can move it anywhere.

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
3. Installed copy: open Settings, Apps, Installed apps, find NepaliToolkit and
   choose Uninstall, or run `%LOCALAPPDATA%\NepaliToolkit\uninstall.exe`.
4. Portable copy: delete `NepaliToolkit.exe` from the folder you extracted.
5. Optionally delete `%APPDATA%\com.aaxyat.nepalitoolkit\`.

Removing the app while autostart is still on leaves a Run entry pointing at a
missing file. Turning autostart off first avoids that.
