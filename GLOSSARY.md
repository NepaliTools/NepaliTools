# NepaliToolkit

A tray-resident host that exposes Nepali tools as modules; V1 ships Today and Converter. The names below are this project's vocabulary, not UI copy.

## Calendars

**BS (Bikram Sambat, विक्रम संवत्)**:
Nepal's official civil calendar. Its months are solar and their lengths vary by year according to a published table.
_Avoid_: Nepali calendar, Vikram Sambat, Bikram Samvat

**AD (Gregorian)**:
The civil calendar of the operating system clock; the counterpart calendar in every conversion.
_Avoid_: English date, Christian calendar

**BS date**:
A year, month and day triple in BS. It is the unit the Converter produces and consumes.
_Avoid_: Nepali date

**Patro (पात्रो)**:
A Nepali calendar or almanac. "The printed patro" is the external reference BS data is checked against.
_Avoid_: calendar (unqualified)

**Today**:
The date the tray reports: the machine's local date by default, or the Nepal date when the Kathmandu setting is on.
_Avoid_: current date

**Nepal date**:
The calendar date at Asia/Kathmandu (UTC+05:45), independent of the machine's timezone.
_Avoid_: Nepali time

**Projected year**:
A BS year after 2090 with a single source and no published gazette. Results in such years are labelled projected.
_Avoid_: estimated year, unverified year

## Host and modules

**Host**:
The tray application itself: tray icon, panel, module registry, settings. It is not a module.
_Avoid_: shell, core app

**Module**:
A self-contained user-facing tool in the host, such as Today or Converter. A module can be enabled or disabled; a disabled module is absent from the UI.
_Avoid_: plugin, feature, tool, utility

**Panel**:
The popover opened by clicking the tray icon; it shows the enabled modules.
_Avoid_: window, popup, flyout

**Tray**:
The macOS menu bar or Windows notification area, where the Host's icon lives.
_Avoid_: menubar, system tray, status bar

**Converter**:
The module that translates a BS date to an AD date, or the reverse.
_Avoid_: date converter

**Text**:
The module that converts Nepali text between Preeti keys and Unicode Devanagari,
and holds the shortcut mode.
_Avoid_: font converter, unicode tool

## Text encoding

**Preeti**:
A legacy Nepali font that draws Devanagari glyphs on ordinary ASCII keys. The
bytes are Latin letters, so text typed in Preeti looks like `g]kfn` anywhere the
font is absent. Preeti is an encoding here, not a font file.
_Avoid_: Preeti font, legacy font (when the encoding is meant)

**Unicode handling**:
Unicode stores the Devanagari characters themselves, in logical order, so the
text renders anywhere without a special font. This project converts to and from
Unicode; it never depends on a font being installed.
_Avoid_: UTF-8 (that is an encoding of Unicode, not the characters)

**Matra**:
A dependent vowel sign attached to a consonant, such as the `ा` in `का`. Preeti
types some matras before their consonant.
_Avoid_: vowel (that is the independent letter)

**Reph**:
The `र्` written above a consonant in words such as धर्म. Preeti types it after
the consonant it belongs to (`{`), Unicode stores it before.
_Avoid_: reph mark, half ra

**Half form**:
A consonant with an explicit halant, such as `क्`, used as the first member of a
cluster. Preeti has dedicated uppercase keys for many of them.
_Avoid_: half letter, conjunct (that is the whole cluster)

**In-place conversion**:
Replacing the selected text in another application with its converted form, using
the clipboard and synthetic keystrokes. The panel is never a target.
_Avoid_: global replace, hotkey conversion

## Display

**Devanagari digits**:
The numerals ०१२३४५६७८९, used for every BS date in both interface languages, and for all numbers in Nepali mode.
_Avoid_: Nepali numerals, Hindi numerals
