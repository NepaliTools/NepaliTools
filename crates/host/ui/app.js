const tauri = window.__TAURI__;
const invoke = tauri && tauri.core ? tauri.core.invoke : null;
const listen = tauri && tauri.event ? tauri.event.listen : null;

const STR = {
  np: {
    headerToday: "आज",
    tabToday: "आज",
    tabConvert: "परिवर्तन",
    adLabel: "ईस्वी मिति",
    year: "वर्ष",
    month: "महिना",
    day: "गते",
    copy: "प्रतिलिपि",
    copied: "सकियो",
    todayBtn: "आज",
    errInvalid: "यो मिति मिलेन।",
    errRange: "समर्थित दायरा बाहिरको मिति हो।",
    projected: "२०९१ र त्यसपछिको मिति प्रक्षेपित हो।",
    modules: "मोड्युल",
    settings: "सेटिङ",
    quit: "बन्द",
    modulesTitle: "मोड्युल",
    modTodayTitle: "आज",
    modTodaySub: "आजको विक्रम सम्वत् मिति",
    modConvTitle: "परिवर्तन",
    modConvSub: "ईस्वी र विक्रम सम्वत् दुईतर्फी",
    settingsTitle: "सेटिङ",
    langTitle: "भाषा",
    nepalTitle: "नेपालको मिति",
    autostartTitle: "सुरुमा चलाउने",
    autostartSub: "Windows सँगै खोल्ने",
    hotkeyTitle: "सर्टकट",
    hotkeyMissing: "उपलब्ध छैन",
    versionTitle: "संस्करण",
    back: "फर्कनु",
    empty: "कुनै मोड्युल सक्रिय छैन।",
    openModules: "मोड्युल खोल्नु",
    updateTitle: "अपडेट",
    updateSub: "GitHub Releases बाट, थिचेपछि मात्र जाँचिन्छ।",
    channelTitle: "च्यानल",
    channelSub: "स्थिर, बिटा वा अल्फा रिलिज।",
    channelEmpty: "{channel} च्यानलमा अझै कुनै रिलिज छैन।",
    updateCheck: "जाँच्नु",
    updateChecking: "जाँच हुँदै...",
    updateCurrent: "हालको संस्करण",
    updateLatest: "तपाईंसँग नयाँ संस्करण छ।",
    updateAvailable: "नयाँ संस्करण उपलब्ध छ",
    updateInstall: "स्थापना गर्नु",
    updateInstalling: "स्थापना हुँदै...",
    updateDownloading: "डाउनलोड हुँदै...",
    updateFailed: "जाँच्न सकिएन। इन्टरनेट जाँच्नुहोस्।",
    updatePortable: "पोर्टेबल संस्करण। नयाँ जिप रिलिज पृष्ठबाट लिनुहोस्।",
    portable: "पोर्टेबल",
    tabText: "पाठ",
    modTextTitle: "पाठ",
    modTextSub: "Preeti र Unicode बीच दुईतर्फी",
    dirToUnicode: "Preeti → Unicode",
    dirToPreeti: "Unicode → Preeti",
    textInPreeti: "Preeti पाठ",
    textInUnicode: "Unicode पाठ",
    textOutPreeti: "Preeti पाठ",
    textOutUnicode: "Unicode पाठ",
    textSwap: "साट्नु",
    textUnconverted: "Preeti मा लेख्न नमिल्ने अक्षर: {chars}",
    textModeTitle: "सर्टकट",
    textModeSub: "छानिएको पाठ जहाँसुकै परिवर्तन गर्ने",
    textShortcutAuto: "Ctrl+Alt+P: चयन गरेको पाठ स्वतः दिशा पहिचान गरी परिवर्तन हुन्छ।",
    textShortcutSplit: "Ctrl+Alt+P: Unicode बाट Preeti। Ctrl+Alt+U: Preeti बाट Unicode।",
    textShortcutMissing: "सर्टकट उपलब्ध छैन: {error}",
    textConverted: "{chars} अक्षर परिवर्तन भयो।",
    textPanelFocused: "प्यानलमा सर्टकट चल्दैन। माथिको बाकसमा पाठ हाल्नुहोस्।",
    textNoSelection: "पहिले पाठ छान्नुहोस्, अनि सर्टकट थिच्नुहोस्।",
    textUnchanged: "परिवर्तन गर्नुपर्ने केही भेटिएन।",
    textFailed: "चयन पढ्न सकिएन।",
  },
  en: {
    headerToday: "Today",
    tabToday: "Today",
    tabConvert: "Convert",
    adLabel: "Gregorian date",
    year: "Year",
    month: "Month",
    day: "Day",
    copy: "Copy",
    copied: "Copied",
    todayBtn: "Today",
    errInvalid: "That date does not exist.",
    errRange: "That date is outside the supported range.",
    projected: "Dates from 2091 on are projected.",
    modules: "Modules",
    settings: "Settings",
    quit: "Quit",
    modulesTitle: "Modules",
    modTodayTitle: "Today",
    modTodaySub: "Today's Bikram Sambat date",
    modConvTitle: "Converter",
    modConvSub: "Both directions, BS and AD",
    settingsTitle: "Settings",
    langTitle: "Language",
    nepalTitle: "Nepal date",
    autostartTitle: "Start with Windows",
    autostartSub: "Launch when Windows starts",
    hotkeyTitle: "Shortcut",
    hotkeyMissing: "unavailable",
    versionTitle: "Version",
    back: "Back",
    empty: "No modules are enabled.",
    openModules: "Open modules",
    updateTitle: "Updates",
    updateSub: "Checks GitHub Releases only when you press the button.",
    channelTitle: "Channel",
    channelSub: "Stable, beta or alpha releases.",
    channelEmpty: "No {channel} release yet.",
    updateCheck: "Check",
    updateChecking: "Checking...",
    updateCurrent: "Current version",
    updateLatest: "You are on the latest release.",
    updateAvailable: "Update available",
    updateInstall: "Install and restart",
    updateInstalling: "Installing...",
    updateDownloading: "Downloading...",
    updateFailed: "Could not check. You may be offline.",
    updatePortable: "Portable build. Download the new zip from the release page.",
    portable: "portable",
    tabText: "Text",
    modTextTitle: "Text",
    modTextSub: "Preeti and Unicode, both directions",
    dirToUnicode: "Preeti → Unicode",
    dirToPreeti: "Unicode → Preeti",
    textInPreeti: "Preeti text",
    textInUnicode: "Unicode text",
    textOutPreeti: "Preeti text",
    textOutUnicode: "Unicode text",
    textSwap: "Swap",
    textUnconverted: "Characters Preeti cannot write: {chars}",
    textModeTitle: "Shortcut",
    textModeSub: "Convert the selection anywhere",
    textShortcutAuto: "Ctrl+Alt+P converts the selection and picks the direction.",
    textShortcutSplit: "Ctrl+Alt+P: Unicode to Preeti. Ctrl+Alt+U: Preeti to Unicode.",
    textShortcutMissing: "Shortcut unavailable: {error}",
    textConverted: "Converted {chars} characters.",
    textPanelFocused: "The shortcut does not run inside the panel. Use the box above.",
    textNoSelection: "Select text first, then press the shortcut.",
    textUnchanged: "Nothing to convert.",
    textFailed: "Could not read the selection.",
  },
};

let S = null;
let lang = "np";
let screen = "today";
let converterReady = false;
let mode = "ad";
let current = null;
let updateStatus = "idle";
let update = null;
let updateDownloaded = 0;
let updateTotal = 0;
let textDirection = "to_unicode";
let textUnconverted = [];
let textStatus = "";
let textTimer = 0;

const $ = (id) => document.getElementById(id);
const copyOf = () => STR[lang];

function iso(date) {
  return `${String(date.y).padStart(4, "0")}-${String(date.m).padStart(2, "0")}-${String(date.d).padStart(2, "0")}`;
}

function pad(value) {
  return String(value).padStart(2, "0");
}

function daysIn(year, month) {
  const index = S.bs_years.indexOf(year);
  if (index < 0) return 30;
  return S.bs_calendar[index][month - 1];
}

function yearLabel(year) {
  const index = S.bs_years.indexOf(year);
  return index >= 0 ? S.years_np[index] : String(year);
}

function dayLabel(day) {
  return S.days_np[day - 1] ?? String(day);
}

function rebuildYears(selected) {
  const current = Number($("bs-year").value) || S.today.bs.y;
  fillSelect(
    $("bs-year"),
    S.bs_years.map((year) => ({ value: year, label: yearLabel(year) })),
    selected ?? current
  );
}

function fillSelect(select, items, selected) {  select.replaceChildren();
  for (const item of items) {
    const option = document.createElement("option");
    option.value = String(item.value);
    option.textContent = item.label;
    select.append(option);
  }
  if (selected !== undefined) select.value = String(selected);
}

function rebuildMonths(selected) {
  const values = S.months_np;
  const current = Number($("bs-month").value) || 1;
  fillSelect(
    $("bs-month"),
    values.map((label, index) => ({ value: index + 1, label })),
    selected ?? current
  );
}

function rebuildDays(selected) {
  const year = Number($("bs-year").value);
  const month = Number($("bs-month").value);
  const count = daysIn(year, month);
  const keep = selected ?? Number($("bs-day").value) ?? 1;
  fillSelect(
    $("bs-day"),
    Array.from({ length: count }, (_, index) => ({
      value: index + 1,
      label: dayLabel(index + 1),
    })),
    Math.min(Math.max(keep, 1), count)
  );
}

function setBs(y, m, d) {
  $("bs-year").value = String(y);
  rebuildMonths(m);
  rebuildDays(d);
}

function render() {
  if (!S) return;
  lang = S.lang;
  const s = copyOf();
  document.documentElement.lang = lang === "np" ? "ne" : "en";

  $("header-label").textContent = s.headerToday;
  $("header-bs").textContent = S.today.bs_np;
  $("header-sub").textContent = `${S.today[`weekday_${lang}`]} · ${S.today[`ad_${lang}`]}`;

  $("seg-today").textContent = s.tabToday;
  $("seg-converter").textContent = s.tabConvert;
  $("seg-text").textContent = s.tabText;
  $("seg-today").hidden = !S.modules.today;
  $("seg-converter").hidden = !S.modules.converter;
  $("seg-text").hidden = !S.modules.text;
  const enabled = [S.modules.today, S.modules.converter, S.modules.text].filter(Boolean).length;
  $("segmented").hidden = enabled < 2;

  $("lbl-ad").textContent = s.adLabel;
  $("lbl-year").textContent = s.year;
  $("lbl-month").textContent = s.month;
  $("lbl-day").textContent = s.day;
  $("copy-today").textContent = s.copy;
  $("copy-bs").textContent = s.copy;
  $("copy-ad").textContent = s.copy;
  $("conv-today").textContent = s.todayBtn;

  $("today-bs").textContent = S.today.bs_np;
  $("today-sub").textContent = `${S.today[`weekday_${lang}`]} · ${S.today[`ad_${lang}`]}`;

  $("modules-title").textContent = s.modulesTitle;
  $("mod-today-title").textContent = s.modTodayTitle;
  $("mod-today-sub").textContent = s.modTodaySub;
  $("mod-conv-title").textContent = s.modConvTitle;
  $("mod-conv-sub").textContent = s.modConvSub;
  $("mod-text-title").textContent = s.modTextTitle;
  $("mod-text-sub").textContent = s.modTextSub;
  $("modules-back").textContent = s.back;

  $("settings-title").textContent = s.settingsTitle;
  $("set-lang-title").textContent = s.langTitle;
  $("set-nepal-title").textContent = s.nepalTitle;
  $("set-autostart-title").textContent = s.autostartTitle;
  $("set-autostart-sub").textContent = s.autostartSub;
  $("set-hotkey-title").textContent = s.hotkeyTitle;
  $("set-hotkey-sub").textContent = S.hotkey_ok
    ? S.hotkey
    : `${S.hotkey} — ${s.hotkeyMissing}`;
  $("set-hotkey-sub").title = S.hotkey_error || "";
  $("set-version-title").textContent = s.versionTitle;
  $("set-version").textContent = S.version;
  $("settings-back").textContent = s.back;

  $("nav-modules").textContent = s.modules;
  $("nav-settings").textContent = s.settings;
  $("quit").textContent = s.quit;
  $("empty-text").textContent = s.empty;
  $("empty-modules").textContent = s.openModules;

  $("lang-select").value = S.lang;
  $("set-nepal").checked = S.nepal_date;
  $("set-autostart").checked = S.autostart;
  $("mod-today").checked = S.modules.today;
  $("mod-converter").checked = S.modules.converter;
  $("mod-text").checked = S.modules.text;
  $("ad-input").min = S.ad_min;
  $("ad-input").max = S.ad_max;

  rebuildYears();
  rebuildMonths();
  rebuildDays();
  renderUpdates();
  renderText();
  show(screen);
}

function channelLabel(channel) {
  return channel ? channel.charAt(0).toUpperCase() + channel.slice(1) : "";
}

function renderUpdates() {
  if (!S) return;
  const s = copyOf();
  $("update-title").textContent = s.updateTitle;
  $("update-current").textContent = `${s.updateCurrent} ${S.version}${S.portable ? ` · ${s.portable}` : ""}`;
  $("update-channel-title").textContent = s.channelTitle;
  $("update-channel-sub").textContent = s.channelSub;
  $("update-channel").value = S.channel;
  $("update-channel").disabled = updateStatus === "checking" || updateStatus === "installing";
  $("update-check").textContent = updateStatus === "checking" ? s.updateChecking : s.updateCheck;
  $("update-check").disabled = updateStatus === "checking" || updateStatus === "installing";
  const available = updateStatus === "available" && update;
  $("update-available").hidden = !available;
  if (available) {
    $("update-available-title").textContent = `${s.updateAvailable} · ${update.version}`;
    $("update-notes").textContent = update.notes || "";
    $("update-install").hidden = S.portable;
    $("update-install").textContent = s.updateInstall;
  }
  const progress = $("update-progress");
  progress.hidden = updateStatus !== "installing";
  progress.value = updateTotal ? Math.min(100, Math.round((updateDownloaded / updateTotal) * 100)) : 0;
  const messages = {
    idle: s.updateSub,
    checking: s.updateChecking,
    current: s.updateLatest,
    empty: s.channelEmpty.replace("{channel}", channelLabel(S.channel)),
    available: s.updateAvailable,
    failed: s.updateFailed,
    installing: updateTotal ? `${s.updateDownloading} ${progress.value}%` : s.updateInstalling,
  };
  $("update-status").textContent =
    updateStatus === "available" && S.portable ? s.updatePortable : messages[updateStatus] || "";
}

async function checkForUpdate() {
  if (updateStatus === "checking" || updateStatus === "installing") return;
  updateStatus = "checking";
  update = null;
  updateDownloaded = 0;
  updateTotal = 0;
  renderUpdates();
  try {
    const result = await invoke("check_update", { channel: S.channel });
    update = result.update;
    updateStatus = update ? "available" : result.empty ? "empty" : "current";
  } catch (error) {
    console.error(error);
    updateStatus = "failed";
  }
  renderUpdates();
}

async function installUpdate() {
  if (updateStatus !== "available" || S.portable) return;
  updateStatus = "installing";
  updateDownloaded = 0;
  updateTotal = 0;
  renderUpdates();
  try {
    await invoke("install_update", { channel: S.channel });
  } catch (error) {
    console.error(error);
    updateStatus = "failed";
    renderUpdates();
  }
}

function renderText() {
  if (!S) return;
  const s = copyOf();
  const toUnicode = textDirection === "to_unicode";
  $("dir-to-unicode").textContent = s.dirToUnicode;
  $("dir-to-preeti").textContent = s.dirToPreeti;
  $("dir-to-unicode").classList.toggle("is-active", toUnicode);
  $("dir-to-preeti").classList.toggle("is-active", !toUnicode);
  $("text-in-label").textContent = toUnicode ? s.textInPreeti : s.textInUnicode;
  $("text-out-label").textContent = toUnicode ? s.textOutUnicode : s.textOutPreeti;
  $("text-in").placeholder = toUnicode ? "g]kfn" : "नेपाल";
  $("text-copy").textContent = s.copy;
  $("text-swap").textContent = s.textSwap;
  $("text-mode-title").textContent = s.textModeTitle;
  $("text-mode-sub").textContent = s.textModeSub;

  const mode = $("text-mode");
  mode.value = S.shortcut_mode;
  const note = S.shortcut_mode === "split" ? s.textShortcutSplit : s.textShortcutAuto;
  $("text-shortcut-note").textContent = S.text_shortcut_ok
    ? note
    : s.textShortcutMissing.replace("{error}", S.text_shortcut_error || "");

  $("text-unconverted").hidden = textUnconverted.length === 0;
  $("text-unconverted").textContent = textUnconverted.length
    ? s.textUnconverted.replace("{chars}", textUnconverted.join(" "))
    : "";
  $("text-status").hidden = !textStatus;
  $("text-status").textContent = textStatus;
}

async function convertText() {
  const value = $("text-in").value;
  if (!value) {
    $("text-out").value = "";
    textUnconverted = [];
    renderText();
    return;
  }
  try {
    const result = await invoke("convert_text", { text: value, direction: textDirection });
    $("text-out").value = result.text;
    textUnconverted = result.unconverted || [];
  } catch (error) {
    console.error(error);
    textUnconverted = [];
  }
  renderText();
}

function scheduleText() {
  textStatus = "";
  window.clearTimeout(textTimer);
  textTimer = window.setTimeout(convertText, 60);
}

function setDirection(direction) {
  if (textDirection === direction) return;
  textDirection = direction;
  textStatus = "";
  renderText();
  convertText();
}

function swapText() {
  const input = $("text-in").value;
  $("text-in").value = $("text-out").value;
  $("text-out").value = input;
  textDirection = textDirection === "to_unicode" ? "to_preeti" : "to_unicode";
  textStatus = "";
  renderText();
  convertText();
}

function showTextShortcut(report) {
  const s = copyOf();
  if (report.code === "converted") {
    textStatus = s.textConverted.replace("{chars}", String(report.chars));
    textUnconverted = [];
  } else if (report.code === "partial") {
    textUnconverted = report.unconverted || [];
    textStatus = s.textConverted.replace("{chars}", String(report.chars));
  } else {
    const messages = {
      no_selection: s.textNoSelection,
      panel_focused: s.textPanelFocused,
      unchanged: s.textUnchanged,
      failed: s.textFailed,
    };
    textStatus = messages[report.code] || "";
  }
  show("text");
  renderText();
}

function firstScreen() {
  if (S.modules.today) return "today";
  if (S.modules.converter) return "converter";
  if (S.modules.text) return "text";
  return "empty";
}

function show(next) {
  if (!S) return;
  if (next === "today" && !S.modules.today) next = S.modules.converter ? "converter" : "empty";
  if (next === "converter" && !S.modules.converter) next = S.modules.today ? "today" : "empty";
  if (next === "text" && !S.modules.text) next = firstScreen();
  if (next === "empty" && firstScreen() !== "empty") next = firstScreen();
  screen = next;
  const scroll = document.getElementById("main");
  if (scroll) scroll.scrollTop = 0;
  for (const element of document.querySelectorAll(".screen")) {
    element.classList.toggle("is-active", element.id === `screen-${next}`);
  }
  for (const element of document.querySelectorAll(".seg")) {
    element.classList.toggle("is-active", element.dataset.screen === next);
  }
  if (next === "converter" && !converterReady) initConverter();
}

function initConverter() {
  converterReady = true;
  setBs(S.today.bs.y, S.today.bs.m, S.today.bs.d);
  $("ad-input").value = iso(S.today.ad);
  applyResult(S.today);
}

function applyResult(result) {
  current = result;
  $("res-bs").textContent = result.bs_np;
  $("res-bs-sub").textContent = result[`weekday_${lang}`];
  $("res-ad").textContent = result[`ad_${lang}`];
  $("res-ad-sub").textContent = "";
  $("conv-error").hidden = true;
  $("projected-note").hidden = !result.projected;
  $("projected-note").textContent = copyOf().projected;
  if (mode === "bs") {
    $("ad-input").value = iso(result.ad);
  } else {
    setBs(result.bs.y, result.bs.m, result.bs.d);
  }
}

function showError(code) {
  const s = copyOf();
  const message =
    code === "out_of_range"
      ? `${s.errRange} ${S.ad_min} – ${S.ad_max}`
      : s.errInvalid;
  current = null;
  $("res-bs").textContent = "—";
  $("res-bs-sub").textContent = "";
  $("res-ad").textContent = "—";
  $("res-ad-sub").textContent = "";
  $("conv-error").textContent = message;
  $("conv-error").hidden = false;
  $("conv-error").scrollIntoView({ block: "nearest" });
}

async function convertFromAd() {
  const value = $("ad-input").value;
  if (!value) return;
  const [y, m, d] = value.split("-").map(Number);
  mode = "ad";
  try {
    applyResult(await invoke("convert_ad", { y, m, d }));
  } catch (error) {
    showError(String(error));
  }
}

async function convertFromBs() {
  const y = Number($("bs-year").value);
  const m = Number($("bs-month").value);
  const d = Number($("bs-day").value);
  mode = "bs";
  try {
    applyResult(await invoke("convert_bs", { y, m, d }));
  } catch (error) {
    showError(String(error));
  }
}

async function copyText(text, button) {
  if (!text) return;
  try {
    await invoke("copy_text", { text });
    button.textContent = copyOf().copied;
    window.setTimeout(() => {
      button.textContent = copyOf().copy;
    }, 1200);
  } catch (error) {
    console.error(error);
  }
}

async function refresh() {
  try {
    S = await invoke("get_state");
  } catch (error) {
    console.error(error);
    return;
  }
  render();
}

function wire() {
  $("seg-today").addEventListener("click", () => show("today"));
  $("seg-converter").addEventListener("click", () => show("converter"));
  $("seg-text").addEventListener("click", () => show("text"));
  $("nav-modules").addEventListener("click", () => show("modules"));
  $("nav-settings").addEventListener("click", () => show("settings"));
  $("update-check").addEventListener("click", checkForUpdate);
  $("update-install").addEventListener("click", installUpdate);
  $("update-channel").addEventListener("change", async (event) => {
    S = await invoke("set_channel", { channel: event.target.value });
    update = null;
    updateStatus = "idle";
    renderUpdates();
  });
  $("empty-modules").addEventListener("click", () => show("modules"));
  $("modules-back").addEventListener("click", () => show(firstScreen()));
  $("settings-back").addEventListener("click", () => show(firstScreen()));
  $("quit").addEventListener("click", () => invoke("quit_app"));

  $("copy-today").addEventListener("click", () =>
    copyText(S.today.bs_np, $("copy-today"))
  );
  $("copy-bs").addEventListener("click", () =>
    copyText(current ? current.bs_np : "", $("copy-bs"))
  );
  $("copy-ad").addEventListener("click", () =>
    copyText(current ? current[`ad_${lang}`] : "", $("copy-ad"))
  );

  $("ad-input").addEventListener("change", convertFromAd);
  $("bs-year").addEventListener("change", () => {
    rebuildDays();
    convertFromBs();
  });
  $("bs-month").addEventListener("change", () => {
    rebuildDays();
    convertFromBs();
  });
  $("bs-day").addEventListener("change", convertFromBs);
  $("conv-today").addEventListener("click", () => {
    mode = "ad";
    setBs(S.today.bs.y, S.today.bs.m, S.today.bs.d);
    $("ad-input").value = iso(S.today.ad);
    applyResult(S.today);
  });

  $("lang-select").addEventListener("change", async (event) => {
    S = await invoke("set_language", { lang: event.target.value });
    render();
  });
  $("set-nepal").addEventListener("change", async (event) => {
    S = await invoke("set_nepal_date", { enabled: event.target.checked });
    render();
  });
  $("set-autostart").addEventListener("change", async (event) => {
    try {
      S = await invoke("set_autostart", { enabled: event.target.checked });
      render();
    } catch (error) {
      console.error(error);
      event.target.checked = !event.target.checked;
    }
  });
  $("mod-today").addEventListener("change", async (event) => {
    S = await invoke("set_module", { module: "today", enabled: event.target.checked });
    render();
  });
  $("mod-converter").addEventListener("change", async (event) => {
    S = await invoke("set_module", { module: "converter", enabled: event.target.checked });
    render();
  });
  $("mod-text").addEventListener("change", async (event) => {
    S = await invoke("set_module", { module: "text", enabled: event.target.checked });
    render();
  });

  $("dir-to-unicode").addEventListener("click", () => setDirection("to_unicode"));
  $("dir-to-preeti").addEventListener("click", () => setDirection("to_preeti"));
  $("text-in").addEventListener("input", scheduleText);
  $("text-swap").addEventListener("click", swapText);
  $("text-copy").addEventListener("click", () => copyText($("text-out").value, $("text-copy")));
  $("text-mode").addEventListener("change", async (event) => {
    S = await invoke("set_shortcut_mode", { mode: event.target.value });
    textStatus = "";
    render();
  });

  window.addEventListener("keydown", (event) => {
    if (event.key === "Escape") invoke("hide_panel");
  });
  document.addEventListener("contextmenu", (event) => event.preventDefault());
}

async function start() {
  if (!invoke) {
    document.body.textContent = "Tauri bridge unavailable";
    return;
  }
  wire();
  await refresh();
  if (listen) {
    await listen("panel-opened", refresh);
    await listen("day-changed", refresh);
    await listen("update-progress", (event) => {
      const { downloaded, total } = event.payload;
      updateDownloaded = downloaded;
      updateTotal = total || 0;
      if (updateStatus === "installing") renderUpdates();
    });
    await listen("text-shortcut", (event) => showTextShortcut(event.payload));
    await listen("navigate", (event) => {
      show(event.payload);
    });
  }
}

start();
