# FAQ

### How do I show the Nepali date in the Windows taskbar?

Run NepaliToolkit and pin its tray icon. Windows 11 keeps new icons in the `^`
overflow at first. The icon shows the BS day in Devanagari, and hovering it shows
the full date.

### Can it convert BS to AD and AD to BS offline?

Yes. Both directions use the bundled table. The app makes no network calls and
has no telemetry.

### Which years are supported?

BS 1975 to 2100, which is 1918-04-13 to 2044-04-13 in the Gregorian calendar.
Results after 2090 are labelled *projected* because no official gazette exists
for them yet.

### Why is 2091 to 2100 called projected?

Nepal publishes the official calendar only a short time ahead. Beyond the
published range, month lengths come from a single table and can change when the
official calendar appears. The app says so instead of presenting a guess as fact.

### Why does the icon show the day instead of the month?

A single day number stays readable at 16 px, and a month name does not. The full
date is in the tooltip and the panel.

### Is there a macOS version?

Not yet. The host is Tauri 2 and the date core is platform-neutral. macOS is the
next milestone on the [Roadmap](Roadmap).

### Does it need Node.js, npm or Electron?

No. It is Rust plus the system webview, with a plain HTML/CSS/JS panel.

### How do I convert Preeti text to Unicode?

Open the Text tab and paste the text; the Unicode appears in the other box. To
convert a selection in another application, press `Ctrl+Alt+P`. It runs offline.

### Does the shortcut touch my clipboard?

It saves the clipboard text, copies your selection, converts it, pastes the result
and puts the text back. An image or rich content on the clipboard is not preserved.
It refuses to run while the panel has focus, and with nothing selected it reports
that instead of converting old clipboard content.

### Is the data trustworthy?
The table comes from four sources. A strict majority decides each month, and
every disagreement is written to `data/overrides.csv` with its competing values.
The test suite fails if an unexplained disagreement appears.
See [Data and accuracy](Data-and-Accuracy).

### What gets stored about me?

A settings file and a local diagnostic log in
`%APPDATA%\com.aaxyat.nepalitoolkit\`. Nothing leaves the machine.

### Can I use it in English?

Yes. English is the default interface. The Nepali date is still written in
Nepali (`मंसिर ५, २०८३`), because that is the date itself. Switch to Nepali in
Settings for a fully Nepali interface.
