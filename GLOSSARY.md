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

## Display

**Devanagari digits**:
The numerals ०१२३४५६७८९, used for every BS date in both interface languages, and for all numbers in Nepali mode.
_Avoid_: Nepali numerals, Hindi numerals
