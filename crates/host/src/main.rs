#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! NepaliToolkit host: tray icon, panel window, module registry and settings.
//! All date logic lives in `npt-core`; this file only moves dates and pixels.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use chrono::{Datelike, FixedOffset, Local, Timelike, Utc};
use npt_core::{AdDate, BsDate, DateError, Lang};
use serde::{Deserialize, Serialize};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{
    AppHandle, Emitter, Manager, PhysicalPosition, State, WindowEvent,
};
use tauri_plugin_autostart::ManagerExt as AutostartExt;
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tauri_plugin_updater::UpdaterExt;

const TRAY_ID: &str = "main";
const PANEL: &str = "panel";
const HOTKEY: &str = "Ctrl+Alt+N";

mod badges;

// ---------------------------------------------------------------- settings

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct Modules {
    today: bool,
    converter: bool,
    text: bool,
}

impl Default for Modules {
    fn default() -> Self {
        Modules { today: true, converter: true, text: true }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct Settings {
    lang: String,
    nepal_date: bool,
    autostart: bool,
    channel: String,
    /// `auto` follows the selection's script; `split` uses one key per direction.
    shortcut_mode: String,
    modules: Modules,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            lang: "en".into(),
            nepal_date: false,
            autostart: true,
            channel: "alpha".into(),
            shortcut_mode: "auto".into(),
            modules: Modules::default(),
        }
    }
}

/// Release channels, newest build first. A channel is a promotion track, not a
/// separate version line: `beta` and `stable` receive a copy of a manifest that
/// was first published to `alpha`.
const CHANNELS: [&str; 3] = ["stable", "beta", "alpha"];

fn normalize_channel(value: &str) -> &'static str {
    CHANNELS
        .into_iter()
        .find(|channel| *channel == value)
        .unwrap_or("alpha")
}

struct AppState {
    settings: Mutex<Settings>,
    path: PathBuf,
    hotkey_error: Mutex<Option<String>>,
    text_shortcut_error: Mutex<Option<String>>,
    shown_at: Mutex<Option<Instant>>,
}

impl AppState {
    fn load(app: &AppHandle) -> Self {
        let dir = app
            .path()
            .app_config_dir()
            .unwrap_or_else(|_| PathBuf::from("."));
        let path = dir.join("settings.json");
        let settings = fs::read_to_string(&path)
            .ok()
            .and_then(|text| serde_json::from_str::<Settings>(&text).ok())
            .unwrap_or_default();
        let mut settings = settings;
        settings.channel = normalize_channel(&settings.channel).into();
        AppState {
            settings: Mutex::new(settings),
            path,
            hotkey_error: Mutex::new(None),
            text_shortcut_error: Mutex::new(None),
            shown_at: Mutex::new(None),
        }
    }

    fn snapshot(&self) -> Settings {
        self.settings.lock().expect("settings lock").clone()
    }
}

fn save_settings(path: &Path, settings: &Settings) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let text = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    fs::write(path, text).map_err(|e| e.to_string())
}

/// Append-only diagnostics next to the settings file. No date data, no secrets.
fn log(message: &str) {
    let Ok(appdata) = std::env::var("APPDATA") else { return };
    let path = PathBuf::from(appdata)
        .join("com.aaxyat.nepalitoolkit")
        .join("app.log");
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(mut file) = fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(
            file,
            "[{}] {message}",
            Local::now().format("%Y-%m-%d %H:%M:%S")
        );
    }
}

fn update_settings(
    state: &AppState,
    mutate: impl FnOnce(&mut Settings),
) -> Result<Settings, String> {
    let mut guard = state.settings.lock().expect("settings lock");
    mutate(&mut guard);
    let settings = guard.clone();
    save_settings(&state.path, &settings)?;
    Ok(settings)
}

// ---------------------------------------------------------------- clock

fn nepal_now() -> chrono::DateTime<FixedOffset> {
    let offset = FixedOffset::east_opt(5 * 3600 + 45 * 60).expect("UTC+05:45");
    Utc::now().with_timezone(&offset) + chrono::Duration::seconds(clock_shift())
}

/// Test-only clock shift, in seconds. Zero unless `NPT_CLOCK_OFFSET_SECS` is set;
/// used to walk the production midnight path without waiting for a real midnight.
fn clock_shift() -> i64 {
    std::env::var("NPT_CLOCK_OFFSET_SECS")
        .ok()
        .and_then(|value| value.parse::<i64>().ok())
        .unwrap_or(0)
}

fn local_now() -> chrono::DateTime<Local> {
    Local::now() + chrono::Duration::seconds(clock_shift())
}

fn current_ad(settings: &Settings) -> AdDate {
    if let Ok(fake) = std::env::var("NPT_TODAY") {
        let parts: Vec<i32> = fake
            .split('-')
            .filter_map(|part| part.parse::<i32>().ok())
            .collect();
        if parts.len() == 3 {
            let date = AdDate {
                year: parts[0],
                month: parts[1] as u8,
                day: parts[2] as u8,
            };
            if npt_core::is_valid_ad(date) {
                return date;
            }
        }
    }
    if settings.nepal_date {
        let now = nepal_now();
        AdDate {
            year: now.year(),
            month: now.month() as u8,
            day: now.day() as u8,
        }
    } else {
        let now = local_now();
        AdDate {
            year: now.year(),
            month: now.month() as u8,
            day: now.day() as u8,
        }
    }
}

fn lang_of(settings: &Settings) -> Lang {
    if settings.lang == "en" {
        Lang::En
    } else {
        Lang::Np
    }
}

// ---------------------------------------------------------------- views

#[derive(Serialize, Clone)]
struct DateView {
    y: i32,
    m: u8,
    d: u8,
}

#[derive(Serialize, Clone)]
struct DayView {
    bs: DateView,
    ad: DateView,
    bs_np: String,
    ad_np: String,
    ad_en: String,
    weekday_np: String,
    weekday_en: String,
    projected: bool,
}

fn date_view_ad(ad: AdDate) -> DateView {
    DateView { y: ad.year, m: ad.month, d: ad.day }
}

fn date_view_bs(bs: BsDate) -> DateView {
    DateView { y: bs.year, m: bs.month, d: bs.day }
}

fn day_view(day: npt_core::Day) -> DayView {
    DayView {
        bs: date_view_bs(day.bs),
        ad: date_view_ad(day.ad),
        bs_np: day.bs_np,
        ad_np: day.ad_np,
        ad_en: day.ad_en,
        weekday_np: day.weekday_np.to_string(),
        weekday_en: day.weekday_en.to_string(),
        projected: day.projected,
    }
}

fn today_view(settings: &Settings) -> Result<DayView, String> {
    npt_core::describe(current_ad(settings))
        .map(day_view)
        .map_err(|e| e.to_string())
}

#[derive(Serialize, Clone)]
struct UiState {
    lang: String,
    nepal_date: bool,
    autostart: bool,
    hotkey: String,
    hotkey_ok: bool,
    hotkey_error: Option<String>,
    modules: Modules,
    today: DayView,
    bs_years: Vec<i32>,
    years_np: Vec<String>,
    days_np: Vec<String>,
    bs_calendar: Vec<[u8; 12]>,
    months_np: Vec<String>,
    ad_min: String,
    ad_max: String,
    version: String,
    portable: bool,
    channel: String,
    shortcut_mode: String,
    text_shortcut_ok: bool,
    text_shortcut_error: Option<String>,
}

/// The installed build lives in `%LOCALAPPDATA%\NepaliToolkit`; anything else is
/// the portable exe, which cannot replace itself with the NSIS installer.
fn is_portable() -> bool {
    let Ok(exe) = std::env::current_exe() else { return true };
    let exe = exe.canonicalize().unwrap_or(exe);
    let installed = std::env::var_os("LOCALAPPDATA")
        .map(|dir| PathBuf::from(dir).join("NepaliToolkit").join("NepaliToolkit.exe"))
        .and_then(|path| path.canonicalize().ok());
    match installed {
        Some(path) => exe != path,
        None => true,
    }
}

fn iso(ad: AdDate) -> String {
    format!("{:04}-{:02}-{:02}", ad.year, ad.month, ad.day)
}

fn build_state(app: &AppHandle, state: &AppState) -> Result<UiState, String> {
    let settings = state.snapshot();
    let today = today_view(&settings)?;
    let years: Vec<i32> = (npt_core::BS_MIN_YEAR..=npt_core::BS_MAX_YEAR).collect();
    let calendar: Vec<[u8; 12]> = years
        .iter()
        .filter_map(|&year| npt_core::bs_year_months(year))
        .collect();
    let years_np = years
        .iter()
        .map(|&year| npt_core::format_digits(year as i64, Lang::Np))
        .collect();
    let days_np = (1..=32)
        .map(|day| npt_core::format_digits(day, Lang::Np))
        .collect();
    let months_np = (1..=12)
        .map(|m| npt_core::bs_month_name(m, Lang::Np).to_string())
        .collect();
    let (first, last) = npt_core::ad_bounds();    let error = state.hotkey_error.lock().expect("hotkey lock").clone();
    // One lock, read once: two `.lock()` temporaries in the same struct literal
    // would both live to the end of the statement and deadlock.
    let shortcut_error = state
        .text_shortcut_error
        .lock()
        .expect("shortcut lock")
        .clone();
    Ok(UiState {
        lang: settings.lang.clone(),
        nepal_date: settings.nepal_date,
        autostart: settings.autostart,
        hotkey: HOTKEY.into(),
        hotkey_ok: error.is_none(),
        hotkey_error: error,
        modules: settings.modules.clone(),
        today,
        bs_years: years,
        years_np,
        days_np,
        bs_calendar: calendar,
        months_np,
        ad_min: iso(first),
        ad_max: iso(last),
        version: app.package_info().version.to_string(),
        portable: is_portable(),
        channel: settings.channel.clone(),
        shortcut_mode: settings.shortcut_mode.clone(),
        text_shortcut_ok: shortcut_error.is_none(),
        text_shortcut_error: shortcut_error,
    })
}

#[derive(Serialize)]
struct ConvView {
    bs: DateView,
    ad: DateView,
    bs_np: String,
    ad_np: String,
    ad_en: String,
    weekday_np: String,
    weekday_en: String,
    projected: bool,
}

fn conv_view(bs: BsDate, ad: AdDate) -> ConvView {
    let day = npt_core::describe(ad).expect("converted date is in range");
    ConvView {
        bs: date_view_bs(bs),
        ad: date_view_ad(ad),
        bs_np: day.bs_np,
        ad_np: day.ad_np,
        ad_en: day.ad_en,
        weekday_np: day.weekday_np.to_string(),
        weekday_en: day.weekday_en.to_string(),
        projected: bs.year > npt_core::BS_VERIFIED_MAX_YEAR,
    }
}

fn error_code(error: DateError) -> String {
    match error {
        DateError::Invalid => "invalid".into(),
        DateError::OutOfRange => "out_of_range".into(),
    }
}

// ---------------------------------------------------------------- tray

/// Windows asks for 16/20/24/32 px depending on the display scale; each size is
/// rendered at that exact size, because downscaling a 32 px canvas destroys the digits.
fn ui_scale(app: &AppHandle) -> f64 {
    app.get_webview_window(PANEL)
        .and_then(|window| window.scale_factor().ok())
        .unwrap_or(1.0)
}

fn menu_for(app: &AppHandle, lang: Lang) -> tauri::Result<Menu<tauri::Wry>> {
    let (modules, settings, quit) = match lang {
        Lang::Np => ("मोड्युल", "सेटिङ", "बन्द"),
        Lang::En => ("Modules", "Settings", "Quit"),
    };
    let modules_item = MenuItem::with_id(app, "modules", modules, true, None::<&str>)?;
    let settings_item = MenuItem::with_id(app, "settings", settings, true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", quit, true, None::<&str>)?;
    Menu::with_items(app, &[&modules_item, &settings_item, &quit_item])
}

fn refresh_tray(app: &AppHandle) -> Result<(), String> {
    let state = app.state::<AppState>();
    let settings = state.snapshot();
    let ad = current_ad(&settings);
    let day = npt_core::describe(ad).map_err(|e| e.to_string())?;
    let tooltip = npt_core::tooltip(ad, lang_of(&settings)).map_err(|e| e.to_string())?;
    log(&format!("tooltip: {tooltip}"));
    let scale = ui_scale(app);
    log(&format!(
        "tray icon: {}px at scale {scale:.2} for day {}",
        badges::size_for(scale),
        day.bs.day
    ));
    if let Some(tray) = app.tray_by_id(TRAY_ID) {
        tray.set_icon(Some(badges::image(day.bs.day, scale)))
            .map_err(|e| e.to_string())?;
        tray.set_tooltip(Some(tooltip)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn place_panel(app: &AppHandle, at: Option<PhysicalPosition<f64>>) {
    let Some(window) = app.get_webview_window(PANEL) else { return };
    let size = window
        .outer_size()
        .unwrap_or(tauri::PhysicalSize::new(380, 520));
    let monitor = window
        .current_monitor()
        .ok()
        .flatten()
        .or_else(|| window.primary_monitor().ok().flatten());
    let (mx, my, mw, mh) = match &monitor {
        Some(monitor) => (
            monitor.position().x,
            monitor.position().y,
            monitor.size().width as i32,
            monitor.size().height as i32,
        ),
        None => (0, 0, 1920, 1080),
    };
    let (mut x, mut y) = match at {
        Some(point) => (
            point.x as i32 - size.width as i32 / 2,
            point.y as i32 - size.height as i32 - 8,
        ),
        None => (
            mx + mw - size.width as i32 - 12,
            my + mh - size.height as i32 - 56,
        ),
    };
    x = x.clamp(mx + 8, mx + mw - size.width as i32 - 8);
    y = y.clamp(my + 8, my + mh - size.height as i32 - 8);
    let _ = window.set_position(PhysicalPosition::new(x, y));
}

fn toggle_panel(app: &AppHandle, at: Option<PhysicalPosition<f64>>) {
    let Some(window) = app.get_webview_window(PANEL) else { return };
    if window.is_visible().unwrap_or(false) {
        log("toggle: hiding");
        let _ = window.hide();
        return;
    }
    place_panel(app, at);
    *app.state::<AppState>().shown_at.lock().expect("shown lock") = Some(Instant::now());
    log("toggle: showing");
    let _ = window.show();
    let _ = window.set_focus();
    let _ = app.emit("panel-opened", ());
}

fn open_screen(app: &AppHandle, screen: &str) {
    let Some(window) = app.get_webview_window(PANEL) else { return };
    place_panel(app, None);
    *app.state::<AppState>().shown_at.lock().expect("shown lock") = Some(Instant::now());
    log(&format!("open screen: {screen}"));
    let _ = window.show();
    let _ = window.set_focus();
    let _ = app.emit("navigate", screen);
}

fn seconds_to_next_midnight(app: &AppHandle) -> u64 {
    let settings = app.state::<AppState>().snapshot();
    let seconds_today = if settings.nepal_date {
        nepal_now().num_seconds_from_midnight() as u64
    } else {
        local_now().num_seconds_from_midnight() as u64
    };
    (86_400 - seconds_today).max(1)
}

fn spawn_day_watcher(app: AppHandle) {
    std::thread::spawn(move || {
        let mut last = {
            let state = app.state::<AppState>();
            npt_core::ad_to_bs(current_ad(&state.snapshot())).ok()
        };
        loop {
            let wait = seconds_to_next_midnight(&app).clamp(5, 300);
            std::thread::sleep(Duration::from_secs(wait));
            let now = {
                let state = app.state::<AppState>();
                npt_core::ad_to_bs(current_ad(&state.snapshot())).ok()
            };
            if now != last {
                last = now;
                log(&format!("day changed: {now:?}"));
                let _ = refresh_tray(&app);
                let _ = app.emit("day-changed", ());
            }
        }
    });
}

// ---------------------------------------------------------------- commands

#[tauri::command]
fn get_state(app: AppHandle, state: State<'_, AppState>) -> Result<UiState, String> {
    build_state(&app, state.inner())
}

#[tauri::command]
fn set_language(app: AppHandle, state: State<'_, AppState>, lang: String) -> Result<UiState, String> {
    let settings = update_settings(state.inner(), |s| {
        s.lang = if lang == "en" { "en".into() } else { "np".into() };
    })?;
    if let Some(tray) = app.tray_by_id(TRAY_ID) {
        if let Ok(menu) = menu_for(&app, lang_of(&settings)) {
            let _ = tray.set_menu(Some(menu));
        }
    }
    refresh_tray(&app)?;
    build_state(&app, state.inner())
}

#[tauri::command]
fn set_nepal_date(
    app: AppHandle,
    state: State<'_, AppState>,
    enabled: bool,
) -> Result<UiState, String> {
    update_settings(state.inner(), |s| s.nepal_date = enabled)?;
    refresh_tray(&app)?;
    let _ = app.emit("day-changed", ());
    build_state(&app, state.inner())
}

#[tauri::command]
fn set_autostart(
    app: AppHandle,
    state: State<'_, AppState>,
    enabled: bool,
) -> Result<UiState, String> {
    if enabled {
        app.autolaunch().enable().map_err(|e| e.to_string())?;
    } else {
        app.autolaunch().disable().map_err(|e| e.to_string())?;
    }
    update_settings(state.inner(), |s| s.autostart = enabled)?;
    build_state(&app, state.inner())
}

#[tauri::command]
fn set_channel(
    app: AppHandle,
    state: State<'_, AppState>,
    channel: String,
) -> Result<UiState, String> {
    update_settings(state.inner(), |s| s.channel = normalize_channel(&channel).into())?;
    build_state(&app, state.inner())
}

#[tauri::command]
fn set_module(
    app: AppHandle,
    state: State<'_, AppState>,
    module: String,
    enabled: bool,
) -> Result<UiState, String> {
    update_settings(state.inner(), |s| match module.as_str() {
        "today" => s.modules.today = enabled,
        "converter" => s.modules.converter = enabled,
        "text" => s.modules.text = enabled,
        _ => {}
    })?;
    build_state(&app, state.inner())
}

#[tauri::command]
fn convert_ad(y: i32, m: u8, d: u8) -> Result<ConvView, String> {
    let ad = AdDate { year: y, month: m, day: d };
    let bs = npt_core::ad_to_bs(ad).map_err(error_code)?;
    Ok(conv_view(bs, ad))
}

#[tauri::command]
fn convert_bs(y: i32, m: u8, d: u8) -> Result<ConvView, String> {
    let bs = BsDate { year: y, month: m, day: d };
    let ad = npt_core::bs_to_ad(bs).map_err(error_code)?;
    Ok(conv_view(bs, ad))
}

#[tauri::command]
fn copy_text(app: AppHandle, text: String) -> Result<(), String> {
    app.clipboard().write_text(text).map_err(|e| e.to_string())
}

#[tauri::command]
fn hide_panel(app: AppHandle) {
    if let Some(window) = app.get_webview_window(PANEL) {
        let _ = window.hide();
    }
}

#[tauri::command]
fn quit_app(app: AppHandle) {
    app.exit(0);
}

// ---------------------------------------------------------------- text

#[derive(Serialize, Clone)]
struct TextResult {
    text: String,
    direction: String,
    unconverted: Vec<char>,
}

/// Converts one text buffer. `direction` may be `auto`, `to_preeti` or
/// `to_unicode`; auto follows the script of the input.
#[tauri::command]
fn convert_text(text: String, direction: String) -> TextResult {
    let (converted, direction) = match direction.as_str() {
        "to_preeti" => (npt_core::preeti::unicode_to_preeti(&text), "to_preeti"),
        "to_unicode" => (npt_core::preeti::preeti_to_unicode(&text), "to_unicode"),
        _ => match npt_core::preeti::detect(&text) {
            npt_core::preeti::Direction::ToPreeti => {
                (npt_core::preeti::unicode_to_preeti(&text), "to_preeti")
            }
            npt_core::preeti::Direction::ToUnicode => {
                (npt_core::preeti::preeti_to_unicode(&text), "to_unicode")
            }
        },
    };
    TextResult {
        text: converted.text,
        direction: direction.into(),
        unconverted: converted.unconverted,
    }
}

#[tauri::command]
fn set_shortcut_mode(
    app: AppHandle,
    state: State<'_, AppState>,
    mode: String,
) -> Result<UiState, String> {
    let mode = if mode == "split" { "split" } else { "auto" };
    update_settings(state.inner(), |s| s.shortcut_mode = mode.into())?;
    sync_text_shortcuts(&app);
    build_state(&app, state.inner())
}

#[derive(Serialize, Clone)]
struct TextShortcutReport {
    code: String,
    direction: String,
    chars: usize,
    unconverted: Vec<char>,
}

/// Registers the in-place shortcuts for the current mode. The panel hotkey is
/// registered separately at startup; this only owns Ctrl+Alt+P and Ctrl+Alt+U.
fn sync_text_shortcuts(app: &AppHandle) {
    let mode = app.state::<AppState>().snapshot().shortcut_mode;
    let shortcuts = app.global_shortcut();
    let ctrl_alt = Modifiers::CONTROL | Modifiers::ALT;
    let to_preeti = Shortcut::new(Some(ctrl_alt), Code::KeyP);
    let to_unicode = Shortcut::new(Some(ctrl_alt), Code::KeyU);
    let mut errors = Vec::new();

    if !shortcuts.is_registered(to_preeti) {
        if let Err(error) = shortcuts.register(to_preeti) {
            errors.push(format!("Ctrl+Alt+P: {error}"));
        }
    }
    if mode == "split" {
        if !shortcuts.is_registered(to_unicode) {
            if let Err(error) = shortcuts.register(to_unicode) {
                errors.push(format!("Ctrl+Alt+U: {error}"));
            }
        }
    } else if shortcuts.is_registered(to_unicode) {
        let _ = shortcuts.unregister(to_unicode);
    }

    let state = app.state::<AppState>();
    let mut guard = state.text_shortcut_error.lock().expect("shortcut lock");
    if errors.is_empty() {
        log(&format!("text shortcuts registered: mode={mode}"));
        *guard = None;
    } else {
        let message = errors.join("; ");
        log(&format!("text shortcut failed: {message}"));
        *guard = Some(message);
    }
}

fn report_shortcut(app: &AppHandle, code: &str, direction: &str, chars: usize, unconverted: Vec<char>) {
    let report = TextShortcutReport {
        code: code.into(),
        direction: direction.into(),
        chars,
        unconverted,
    };
    let _ = app.emit("text-shortcut", report);
    // Anything other than a clean conversion needs a human to look at it.
    if code != "converted" {
        open_screen(app, "text");
    }
}

/// Copies the selection, converts it and pastes it back, then restores the
/// clipboard. Only the text part of the clipboard is preserved.
fn convert_selection(app: &AppHandle, forced: Option<npt_core::preeti::Direction>) {
    use npt_core::preeti::Direction;

    if let Some(window) = app.get_webview_window(PANEL) {
        if window.is_focused().unwrap_or(false) {
            log("text: skipped, panel has focus");
            report_shortcut(app, "panel_focused", "", 0, Vec::new());
            return;
        }
    }

    let clipboard = app.clipboard();
    // The hotkey fires while Ctrl and Alt are still physically down; injecting
    // Ctrl+C on top of that produces Alt+C, which is not a copy.
    wait_for_modifiers();
    let previous = clipboard.read_text().ok().filter(|text| !text.is_empty());

    if !send_ctrl(0x43) {
        log("text: could not send copy");
        report_shortcut(app, "failed", "", 0, Vec::new());
        return;
    }
    let Some(selection) = wait_for_selection(&clipboard, previous.as_deref()) else {
        log("text: no selection");
        restore_clipboard(&clipboard, previous.as_deref());
        report_shortcut(app, "no_selection", "", 0, Vec::new());
        return;
    };

    let direction = forced.unwrap_or_else(|| npt_core::preeti::detect(&selection));
    let (converted, label) = match direction {
        Direction::ToPreeti => (npt_core::preeti::unicode_to_preeti(&selection), "to_preeti"),
        Direction::ToUnicode => (npt_core::preeti::preeti_to_unicode(&selection), "to_unicode"),
    };

    if converted.text == selection {
        log("text: nothing to convert");
        restore_clipboard(&clipboard, previous.as_deref());
        report_shortcut(app, "unchanged", label, 0, Vec::new());
        return;
    }

    if clipboard.write_text(&converted.text).is_err() || !send_ctrl(0x56) {
        log("text: could not paste");
        restore_clipboard(&clipboard, previous.as_deref());
        report_shortcut(app, "failed", label, 0, Vec::new());
        return;
    }
    std::thread::sleep(Duration::from_millis(220));
    restore_clipboard(&clipboard, previous.as_deref());

    let chars = converted.text.chars().count();
    log(&format!(
        "text: {label} {} chars -> {chars} chars, {} unconverted",
        selection.chars().count(),
        converted.unconverted.len()
    ));
    let code = if converted.unconverted.is_empty() { "converted" } else { "partial" };
    report_shortcut(app, code, label, chars, converted.unconverted);
}

/// Polls the clipboard until the copy lands. A copy that produces the same text
/// that was already there is indistinguishable from no copy at all, so it is
/// treated as no selection rather than pasting stale content.
fn wait_for_selection(
    clipboard: &tauri_plugin_clipboard_manager::Clipboard<tauri::Wry>,
    previous: Option<&str>,
) -> Option<String> {
    let deadline = Instant::now() + Duration::from_millis(900);
    while Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(40));
        if let Ok(text) = clipboard.read_text() {
            if !text.is_empty() && previous != Some(text.as_str()) {
                return Some(text);
            }
        }
    }
    None
}

fn restore_clipboard(
    clipboard: &tauri_plugin_clipboard_manager::Clipboard<tauri::Wry>,
    previous: Option<&str>,
) {
    match previous {
        Some(text) => {
            let _ = clipboard.write_text(text);
        }
        None => {
            let _ = clipboard.clear();
        }
    }
}

/// Sends Ctrl plus one virtual key, one event at a time. Some editors ignore a
/// single zero-delay batch, so the presses are spaced out.
#[cfg(windows)]
fn send_ctrl(key: u16) -> bool {
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VK_CONTROL,
    };

    fn send(virtual_key: u16, release: bool) -> bool {
        let mut input: INPUT = unsafe { std::mem::zeroed() };
        input.r#type = INPUT_KEYBOARD;
        input.Anonymous.ki = KEYBDINPUT {
            wVk: virtual_key,
            wScan: 0,
            dwFlags: if release { KEYEVENTF_KEYUP } else { 0 },
            time: 0,
            dwExtraInfo: 0,
        };
        unsafe {
            SendInput(1, &input, std::mem::size_of::<INPUT>() as i32) == 1
        }
    }

    let mut ok = true;
    for (virtual_key, release) in [(VK_CONTROL, false), (key, false), (key, true), (VK_CONTROL, true)] {
        ok &= send(virtual_key, release);
        std::thread::sleep(Duration::from_millis(12));
    }
    ok
}

#[cfg(not(windows))]
fn send_ctrl(_key: u16) -> bool {
    false
}

/// Blocks until the modifier keys from the hotkey chord are released, so the
/// next injected chord is read as Ctrl plus a letter.
#[cfg(windows)]
fn wait_for_modifiers() {
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        GetAsyncKeyState, VK_CONTROL, VK_MENU, VK_SHIFT,
    };
    let deadline = Instant::now() + Duration::from_millis(1200);
    while Instant::now() < deadline {
        let held = unsafe {
            [VK_CONTROL, VK_MENU, VK_SHIFT]
                .into_iter()
                .any(|key| GetAsyncKeyState(key as i32) < 0)
        };
        if !held {
            return;
        }
        std::thread::sleep(Duration::from_millis(15));
    }
}

#[cfg(not(windows))]
fn wait_for_modifiers() {}

// ---------------------------------------------------------------- updates

#[derive(Serialize, Clone)]
struct UpdateInfo {
    version: String,
    notes: String,
    date: Option<String>,
}

#[derive(Serialize, Clone)]
struct UpdateCheck {
    update: Option<UpdateInfo>,
    /// True when the channel has no published manifest yet, which is not the
    /// same as being up to date.
    empty: bool,
}

#[derive(Serialize, Clone)]
struct UpdateProgress {
    downloaded: usize,
    total: Option<u64>,
}

/// Every channel is a static manifest on the site. Keeping the endpoint out of
/// `tauri.conf.json` lets the setting pick the channel at runtime, and the
/// configured endpoint stays as the fallback for builds that predate channels.
fn channel_endpoint(channel: &str) -> Result<tauri::Url, String> {
    let channel = normalize_channel(channel);
    let url = format!("https://nepalitools.pages.dev/updates/{channel}.json");
    url.parse().map_err(|error| format!("bad update url: {error}"))
}

fn updater_for(app: &AppHandle, channel: &str) -> Result<tauri_plugin_updater::Updater, String> {
    app.updater_builder()
        .endpoints(vec![channel_endpoint(channel)?])
        .map_err(|error| error.to_string())?
        .build()
        .map_err(|error| error.to_string())
}

/// Called only when the user presses Check for updates in Settings. The app
/// makes no other network requests and never checks on a timer or at startup.
#[tauri::command]
async fn check_update(app: AppHandle, channel: String) -> Result<UpdateCheck, String> {
    let channel = normalize_channel(&channel);
    log(&format!("update: checking {channel}"));
    let updater = updater_for(&app, channel)?;
    let update = match updater.check().await {
        Ok(update) => update,
        Err(tauri_plugin_updater::Error::ReleaseNotFound) => {
            log(&format!("update: no manifest on channel {channel}"));
            return Ok(UpdateCheck { update: None, empty: true });
        }
        Err(error) => {
            log(&format!("update: check failed: {error}"));
            return Err(error.to_string());
        }
    };
    match update {
        Some(update) => {
            log(&format!("update: {} available on {channel}", update.version));
            Ok(UpdateCheck {
                update: Some(UpdateInfo {
                    version: update.version,
                    notes: update.body.unwrap_or_default(),
                    date: update.date.map(|date| date.to_string()),
                }),
                empty: false,
            })
        }
        None => {
            log(&format!("update: up to date on {channel}"));
            Ok(UpdateCheck { update: None, empty: false })
        }
    }
}

/// Downloads the signed installer and hands it to the NSIS updater in quiet
/// mode (`/S /R`): no installer window, and the new build is started again.
#[tauri::command]
async fn install_update(app: AppHandle, channel: String) -> Result<(), String> {
    let channel = normalize_channel(&channel);
    let updater = updater_for(&app, channel)?;
    let Some(update) = updater
        .check()
        .await
        .map_err(|error| error.to_string())?
    else {
        return Err("no update".into());
    };
    log(&format!("update: installing {} from {channel}", update.version));
    let handle = app.clone();
    let mut downloaded = 0usize;
    update
        .download_and_install(
            move |chunk, total| {
                downloaded += chunk;
                let _ = handle.emit(
                    "update-progress",
                    UpdateProgress {
                        downloaded,
                        total,
                    },
                );
            },
            || {},
        )
        .await
        .map_err(|error| {
            log(&format!("update: install failed: {error}"));
            error.to_string()
        })?;
    Ok(())
}

// ---------------------------------------------------------------- main

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            log("second instance: showing panel");
            open_screen(app, "today");
        }))
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    if event.state() != ShortcutState::Pressed {
                        return;
                    }
                    let ctrl_alt = Modifiers::CONTROL | Modifiers::ALT;
                    let panel =
                        Shortcut::new(Some(ctrl_alt), Code::KeyN);
                    if *shortcut == panel {
                        toggle_panel(app, None);
                        return;
                    }
                    let to_preeti = Shortcut::new(Some(ctrl_alt), Code::KeyP);
                    let to_unicode = Shortcut::new(Some(ctrl_alt), Code::KeyU);
                    let forced = if *shortcut == to_preeti {
                        let mode = app.state::<AppState>().snapshot().shortcut_mode;
                        if mode == "split" {
                            Some(npt_core::preeti::Direction::ToPreeti)
                        } else {
                            None
                        }
                    } else if *shortcut == to_unicode {
                        Some(npt_core::preeti::Direction::ToUnicode)
                    } else {
                        return;
                    };
                    let handle = app.clone();
                    std::thread::spawn(move || convert_selection(&handle, forced));
                })
                .build(),
        )
        .setup(|app| {
            let state = AppState::load(app.handle());
            let settings = state.snapshot();
            if !state.path.exists() {
                let _ = save_settings(&state.path, &settings);
            }
            app.manage(state);
            log(&format!(
                "start: lang={} nepal_date={} autostart={}",
                settings.lang, settings.nepal_date, settings.autostart
            ));

            if let Ok(fake) = std::env::var("NPT_TODAY") {
                log(&format!("date override: {fake}"));
            }
            if clock_shift() != 0 {
                log(&format!("clock offset: {}s", clock_shift()));
            }

            if let Some(window) = app.get_webview_window(PANEL) {
                let handle = window.clone();
                let tray_app = app.handle().clone();
                window.on_window_event(move |event| match event {
                    WindowEvent::Focused(false) => {
                        let recent = {
                            let state = handle.state::<AppState>();
                            let guard = state.shown_at.lock().expect("shown lock");
                            (*guard)
                                .map(|at| at.elapsed() < Duration::from_millis(450))
                                .unwrap_or(false)
                        };
                        log(&format!("focus lost (recent={recent})"));
                        if !recent {
                            let _ = handle.hide();
                        }
                    }
                    WindowEvent::CloseRequested { api, .. } => {
                        api.prevent_close();
                        let _ = handle.hide();
                    }
                    WindowEvent::ScaleFactorChanged { .. } => {
                        log("scale changed: refreshing tray icon");
                        let _ = refresh_tray(&tray_app);
                    }
                    _ => {}
                });
            }

            if settings.autostart {
                match app.autolaunch().enable() {
                    Ok(()) => log("autostart enabled"),
                    Err(error) => log(&format!("autostart enable failed: {error}")),
                }
            } else {
                let _ = app.autolaunch().disable();
            }

            let shortcut =
                Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyN);
            match app.global_shortcut().register(shortcut) {
                Ok(()) => log("hotkey registered"),
                Err(error) => {
                    log(&format!("hotkey failed: {error}"));
                    *app.state::<AppState>()
                        .hotkey_error
                        .lock()
                        .expect("hotkey lock") = Some(error.to_string());
                }
            }
            sync_text_shortcuts(app.handle());

            let ad = current_ad(&settings);
            let day = npt_core::describe(ad).expect("today is inside the BS range");
            let tooltip =
                npt_core::tooltip(ad, lang_of(&settings)).expect("today has a tooltip");
            log(&format!("tooltip: {tooltip}"));
            let menu = menu_for(app.handle(), lang_of(&settings))?;

            TrayIconBuilder::with_id(TRAY_ID)
                .icon(badges::image(day.bs.day, ui_scale(app.handle())))
                .tooltip(tooltip)
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        position,
                        ..
                    } = event
                    {
                        toggle_panel(tray.app_handle(), Some(position));
                    }
                })
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "modules" => open_screen(app, "modules"),
                    "settings" => open_screen(app, "settings"),
                    "quit" => app.exit(0),
                    _ => {}
                })
                .build(app)?;

            log("tray built");
            let _ = refresh_tray(app.handle());
            if std::env::args().any(|arg| arg == "--show") {
                let handle = app.handle().clone();
                std::thread::spawn(move || {
                    std::thread::sleep(Duration::from_millis(900));
                    open_screen(&handle, "today");
                });
            }
            spawn_day_watcher(app.handle().clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_state,
            set_language,
            set_nepal_date,
            set_autostart,
            set_module,
            convert_ad,
            convert_bs,
            copy_text,
            hide_panel,
            quit_app,
            set_channel,
            convert_text,
            set_shortcut_mode,
            check_update,
            install_update
        ])
        .run(tauri::generate_context!())
        .expect("NepaliToolkit failed to start");
}
