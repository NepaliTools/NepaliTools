# Features

## The tray icon

- Shows today's Bikram Sambat day in Devanagari digits on a crimson tile.
- Rendered at 16, 20, 24 and 32 px. The size matching your display scale is used,
  so two-digit days stay legible.
- The tooltip carries the full date and weekday, for example
  `मंसिर ५, २०८३ - Saturday`.
- Rebuilt at local midnight, or at Kathmandu midnight when the Nepal date setting
  is on.

## Opening the panel

- Left click the tray icon.
- Or press `Ctrl+Alt+N` from anywhere.
- Right click the icon for the native menu: Modules, Settings, Quit.
- `Esc`, a click outside, or a second click on the icon closes it.

The header always shows today: the full BS date, the weekday, and the AD
equivalent.

## Today module

The date, large, with a copy button. It answers one question: what is today in
Nepali?

## Converter module

- AD to BS: type in the date field or use the calendar picker.
- BS to AD: choose year, month and day.
- Both directions convert live. Changing one side updates the other.
- Today fills both sides with today's date.
- Copy puts the visible result on the clipboard.
- A date outside BS 1975 to 2100 shows a red error and clears the results instead
  of clamping. Results after 2090 are marked as projected data.

The BS pickers always show Nepali months and Devanagari digits, in both interface
languages: `२०८३` / `मंसिर` / `५`.

## Modules page

Turn Today or Converter on or off. A disabled module is not initialised and its
tab disappears. With one module enabled the tab bar hides. The header keeps
showing today either way.

## Settings

| Setting | Effect |
|---|---|
| Language | English (default) or Nepali for the interface, weekday and AD date. The BS date stays Nepali either way. |
| Nepal date | Uses Asia/Kathmandu (UTC+05:45) instead of the machine's date, so the tray matches Nepal even when you travel. Off by default. |
| Start with Windows | Writes or removes the autostart entry. |
| Shortcut | Shows `Ctrl+Alt+N`, or marks it unavailable if another app owns it. |
| Version | The running build. Select Back to leave the page. |

## Language rule in one line

The interface and the AD date follow the language setting. The Nepali date is
always Nepali.
