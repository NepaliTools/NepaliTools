# Troubleshooting

## I cannot see the tray icon

Windows 11 hides new notification-area icons by default. Click the `^` chevron
next to the clock, find the crimson digit, and drag it onto the taskbar. The app
is running even while the icon is hidden.

## Windows says "unknown publisher"

The build is unsigned. Choose More info, then Run anyway. Signing needs a
certificate, and it only matters if the app is distributed beyond its author.

## The panel closes when I click somewhere else

The panel behaves like a menu, so a click outside closes it. Click the icon or
press `Ctrl+Alt+N` again to reopen it.

## The hotkey does nothing

Open Settings. If the shortcut row says "unavailable", another app owns
`Ctrl+Alt+N`. Close that app and restart NepaliToolkit, or use the tray icon.

## Autostart does not work

- Check that Start with Windows is on in Settings.
- Confirm the entry exists:
  `Get-ItemProperty 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Run' | Select NepaliToolkit`
- If the path points at an old location, turn the setting off and on again from
  the current build.
- Windows can delay startup apps. Task Manager shows the startup state on its
  Startup tab.

## The app does not start

- Check the log at `%APPDATA%\com.aaxyat.nepalitoolkit\app.log`.
- WebView2 must be installed. Without it the window cannot be created. Install
  the Evergreen runtime and try again.
- A second instance does not open a new app. It brings the existing panel up, by
  design.

## Two icons after moving the executable

The old autostart entry launched the old path. Quit both, delete the stale Run
entry, or turn autostart off and on in the current build, and start the new copy.

## A date looks wrong

- Years after 2090 are projections, and the converter marks them.
- Check whether Nepal date is on. It changes which day counts as today.
- If a specific BS month length looks wrong, open an issue with the BS year and
  month. The data audit will show which sources disagree and what was chosen.

## The converter rejects my date

The supported range is BS 1975-01-01 to 2100-12-30, which is 1918-04-13 to
2044-04-13 AD. Out-of-range input is an error, never a clamp.
