param(
    [string]$Setup = "$PSScriptRoot\..\target\release\bundle\nsis\NepaliToolkit_0.1.0_x64-setup.exe",
    [string]$Evidence = "$PSScriptRoot\..\docs\evidence"
)

$ErrorActionPreference = 'Stop'

Add-Type @"
using System;
using System.Runtime.InteropServices;
public class InstallerWin {
  [DllImport("user32.dll")] public static extern bool SetProcessDpiAwarenessContext(IntPtr value);
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
  [DllImport("user32.dll")] public static extern void mouse_event(uint flags, uint dx, uint dy, uint data, UIntPtr extra);
  public struct RECT { public int Left; public int Top; public int Right; public int Bottom; }
}
"@
[void][InstallerWin]::SetProcessDpiAwarenessContext([IntPtr](-4))

Add-Type -AssemblyName UIAutomationClient, UIAutomationTypes, System.Drawing, System.Windows.Forms

function Get-InstallerWindow([int]$ProcessId) {
    $root = [System.Windows.Automation.AutomationElement]::RootElement
    $cond = New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::ProcessIdProperty, $ProcessId)
    $win = $root.FindFirst([System.Windows.Automation.TreeScope]::Children, $cond)
    if ($win) { return $win }
    foreach ($p in Get-Process) {
        if ($p.MainWindowTitle -match 'NepaliToolkit') {
            $cond = New-Object System.Windows.Automation.PropertyCondition(
                [System.Windows.Automation.AutomationElement]::ProcessIdProperty, $p.Id)
            $win = $root.FindFirst([System.Windows.Automation.TreeScope]::Children, $cond)
            if ($win) { return $win }
        }
    }
    return $null
}

function Save-Window($Win, [string]$Path) {
    $rect = New-Object InstallerWin+RECT
    [void][InstallerWin]::GetWindowRect($Win.Current.NativeWindowHandle, [ref]$rect)
    $w = $rect.Right - $rect.Left
    $h = $rect.Bottom - $rect.Top
    $bmp = New-Object System.Drawing.Bitmap $w, $h
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.CopyFromScreen($rect.Left, $rect.Top, 0, 0, $bmp.Size)
    $g.Dispose()
    $bmp.Save($Path, [System.Drawing.Imaging.ImageFormat]::Png)
    $bmp.Dispose()
    "saved $Path (${w}x${h})"
}

function Get-ByName($Win, [string]$Name) {
    $cond = New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::NameProperty, $Name)
    return $Win.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $cond)
}

function Get-Buttons($Win) {
    $cond = New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::ControlTypeProperty,
        [System.Windows.Automation.ControlType]::Button)
    return $Win.FindAll([System.Windows.Automation.TreeScope]::Descendants, $cond)
}

function Click-Next($Win) {
    foreach ($name in @('Next >', 'Next', '&Next >')) {
        $el = Get-ByName $Win $name
        if ($el -and $el.Current.IsEnabled) {
            $el.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern).Invoke()
            return $name
        }
    }
    $buttons = Get-Buttons $Win
    foreach ($el in $buttons) {
        if ($el.Current.Name -match 'Next' -and $el.Current.IsEnabled) {
            $el.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern).Invoke()
            return $el.Current.Name
        }
    }
    throw 'Next button not found'
}

function Click-Button($Win, [string]$Pattern) {
    $buttons = Get-Buttons $Win
    foreach ($el in $buttons) {
        if ($el.Current.Name -match $Pattern -and $el.Current.IsEnabled) {
            $el.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern).Invoke()
            return $el.Current.Name
        }
    }
    throw "Button matching $Pattern not found"
}

function Click-ButtonMouse($Win, [string]$Pattern) {
    $buttons = Get-Buttons $Win
    foreach ($el in $buttons) {
        if ($el.Current.Name -match $Pattern -and $el.Current.IsEnabled) {
            $rect = $el.Current.BoundingRectangle
            $x = [int](($rect.Left + $rect.Right) / 2)
            $y = [int](($rect.Top + $rect.Bottom) / 2)
            [void][InstallerWin]::SetCursorPos($x, $y)
            Start-Sleep -Milliseconds 120
            [InstallerWin]::mouse_event(0x0002, 0, 0, 0, [UIntPtr]::Zero)
            [InstallerWin]::mouse_event(0x0004, 0, 0, 0, [UIntPtr]::Zero)
            return $el.Current.Name
        }
    }
    throw "Button matching $Pattern not found"
}

function Accept-License($Win) {
    $agree = Get-ByName $Win 'I Agree'
    if ($agree -and $agree.Current.IsEnabled) {
        $agree.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern).Invoke()
        return 'advanced'
    }
    $types = @(
        @{ Name = 'RadioButton'; Type = [System.Windows.Automation.ControlType]::RadioButton; Pattern = [System.Windows.Automation.SelectionItemPattern]::Pattern },
        @{ Name = 'CheckBox'; Type = [System.Windows.Automation.ControlType]::CheckBox; Pattern = [System.Windows.Automation.TogglePattern]::Pattern }
    )
    foreach ($entry in $types) {
        $condition = New-Object System.Windows.Automation.PropertyCondition(
            [System.Windows.Automation.AutomationElement]::ControlTypeProperty, $entry.Type)
        $els = $Win.FindAll([System.Windows.Automation.TreeScope]::Descendants, $condition)
        foreach ($el in $els) {
            if ($el.Current.Name -match 'accept' -and $el.Current.Name -notmatch 'do not') {
                if ($entry.Name -eq 'RadioButton') {
                    $el.GetCurrentPattern($entry.Pattern).Select()
                } else {
                    $el.GetCurrentPattern($entry.Pattern).Toggle()
                }
                return 'enabled'
            }
        }
    }
    throw 'Accept control not found'
}

$setupPath = (Resolve-Path $Setup).Path
$proc = Start-Process -FilePath $setupPath -PassThru
Start-Sleep -Seconds 2
$win = Get-InstallerWindow $proc.Id
if (-not $win) { throw 'installer window not found' }
"window: $($win.Current.Name)"

$buttons = Get-Buttons $win
"buttons: " + (($buttons | ForEach-Object { $_.Current.Name }) -join ' | ')

Save-Window $win (Join-Path $Evidence 'installer-welcome.png')
"next: " + (Click-Next $win)
Start-Sleep -Milliseconds 800
Save-Window $win (Join-Path $Evidence 'installer-license.png')
$license = Accept-License $win
"license: $license"
Start-Sleep -Milliseconds 800
if ($license -eq 'enabled') {
    "next: " + (Click-Next $win)
    Start-Sleep -Milliseconds 800
}
Save-Window $win (Join-Path $Evidence 'installer-directory.png')
"next: " + (Click-Next $win)
Start-Sleep -Milliseconds 800
Save-Window $win (Join-Path $Evidence 'installer-start-menu.png')
"install: " + (Click-ButtonMouse $win '^Install$')
Start-Sleep -Seconds 2
$win = Get-InstallerWindow $proc.Id
if (-not $win) { $win = Get-InstallerWindow 0 }
if (-not $win) { throw 'installer window lost after Install' }

$finish = $null
for ($i = 0; $i -lt 90; $i++) {
    Start-Sleep -Milliseconds 500
    $finish = Get-ByName $win 'Finish'
    if ($finish -and $finish.Current.IsEnabled) { break }
}
if (-not $finish) { throw 'Finish not found; install may have failed' }
Start-Sleep -Milliseconds 500
Save-Window $win (Join-Path $Evidence 'installer-finish.png')
"finish: " + $finish.Current.Name
$finish.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern).Invoke()
Start-Sleep -Seconds 3

$installed = Join-Path $env:LOCALAPPDATA 'NepaliToolkit\NepaliToolkit.exe'
$uninstall = Join-Path $env:LOCALAPPDATA 'NepaliToolkit\uninstall.exe'
$shortcut = Join-Path $env:APPDATA 'Microsoft\Windows\Start Menu\Programs\NepaliTools\NepaliToolkit.lnk'
$uninstallKey = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\NepaliToolkit'
$runKey = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Run'
"installed exe: $([bool](Test-Path $installed))"
"uninstaller: $([bool](Test-Path $uninstall))"
"start menu: $([bool](Test-Path $shortcut))"
if (Test-Path $uninstallKey) {
    $u = Get-ItemProperty $uninstallKey
    "uninstall entry: $($u.DisplayName) $($u.DisplayVersion) / $($u.Publisher)"
}
$run = Get-ItemProperty $runKey -ErrorAction SilentlyContinue
"NepaliToolkit Run: $($run.NepaliToolkit)"
$app = Get-Process nepalitoolkit -ErrorAction SilentlyContinue | Select-Object -First 1
if ($app) { "app process: $($app.Id) $($app.Path)" } else { 'app process: not running' }
