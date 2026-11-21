# Hidden UI driver: shows the panel, optionally clicks and presses keys, captures it.
# Run it with -WindowStyle Hidden so the console never takes focus from the panel.
param(
    [int]$X = -1,
    [int]$Y = -1,
    [string]$Keys = "",
    [string]$Out = "tmp\ui-step.png",
    [switch]$Clip,
    [string]$Expect = "",
    [string]$Text = "",
    [string]$Digits = ""
)
$ErrorActionPreference = 'Stop'
Add-Type @"
using System;
using System.Text;
using System.Runtime.InteropServices;
public class Drv {
  public delegate bool EnumProc(IntPtr h, IntPtr l);
  [DllImport("user32.dll")] public static extern bool SetProcessDpiAwarenessContext(IntPtr value);
  [DllImport("user32.dll")] public static extern bool EnumWindows(EnumProc cb, IntPtr l);
  [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
  [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
  [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetClassNameW(IntPtr h, StringBuilder s, int n);
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
  [DllImport("user32.dll")] public static extern void mouse_event(uint f, uint dx, uint dy, uint d, UIntPtr e);
  [DllImport("user32.dll")] public static extern void keybd_event(byte b, byte s, uint f, UIntPtr e);
  public struct RECT { public int L; public int T; public int R; public int B; }
  public static IntPtr Panel(uint target, out RECT rect) {
    IntPtr found = IntPtr.Zero; RECT box = new RECT();
    EnumWindows((h, l) => {
      uint pid; GetWindowThreadProcessId(h, out pid);
      if (pid == target) {
        var c = new StringBuilder(256); GetClassNameW(h, c, 256);
        if (c.ToString() == "Tauri Window") { found = h; GetWindowRect(h, out box); }
      }
      return true;
    }, IntPtr.Zero);
    rect = box; return found;
  }
  public static void Hotkey() {
    keybd_event(0x11, 0, 0, UIntPtr.Zero);
    keybd_event(0x12, 0, 0, UIntPtr.Zero);
    keybd_event(0x4E, 0, 0, UIntPtr.Zero);
    System.Threading.Thread.Sleep(60);
    keybd_event(0x4E, 0, 2, UIntPtr.Zero);
    keybd_event(0x12, 0, 2, UIntPtr.Zero);
    keybd_event(0x11, 0, 2, UIntPtr.Zero);
  }
  public static void Click(int x, int y) {
    SetCursorPos(x, y);
    System.Threading.Thread.Sleep(120);
    mouse_event(0x0002, 0, 0, 0, UIntPtr.Zero);
    mouse_event(0x0004, 0, 0, 0, UIntPtr.Zero);
  }
  public static void Key(byte vk) {
    keybd_event(vk, 0, 0, UIntPtr.Zero);
    System.Threading.Thread.Sleep(40);
    keybd_event(vk, 0, 2, UIntPtr.Zero);
  }
}
"@
[void][Drv]::SetProcessDpiAwarenessContext([IntPtr](-4))
Add-Type -AssemblyName System.Windows.Forms, System.Drawing

$proc = Get-Process nepalitoolkit -ErrorAction SilentlyContinue | Select-Object -First 1
if (-not $proc) { throw "NepaliToolkit is not running" }
$rect = New-Object Drv+RECT
$hwnd = [Drv]::Panel([uint32]$proc.Id, [ref]$rect)
if (-not [Drv]::IsWindowVisible($hwnd)) {
    [Drv]::Hotkey()
    Start-Sleep -Milliseconds 900
}

if ($X -ge 0 -and $Y -ge 0) {
    [Drv]::Click($X, $Y)
    Start-Sleep -Milliseconds 500
}

foreach ($name in ($Keys -split ',' | Where-Object { $_ })) {
    $vk = switch ($name.ToLower()) {
        'down' { 0x28 }
        'up' { 0x26 }
        'enter' { 0x0D }
        'tab' { 0x09 }
        'escape' { 0x1B }
        default { 0x28 }
    }
    [Drv]::Key([byte]$vk)
    Start-Sleep -Milliseconds 250
}
if ($Text) {
    [System.Windows.Forms.SendKeys]::SendWait($Text)
    Start-Sleep -Milliseconds 300
}
foreach ($ch in ($Digits.ToCharArray())) {
    [Drv]::Key([byte](0x30 + [int][string]$ch))
    Start-Sleep -Milliseconds 60
}
Start-Sleep -Milliseconds 400

$bounds = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds
$full = New-Object System.Drawing.Bitmap $bounds.Width, $bounds.Height
$g = [System.Drawing.Graphics]::FromImage($full)
$g.CopyFromScreen($bounds.X, $bounds.Y, 0, 0, $full.Size)
$g.Dispose()
$full.Save((Join-Path (Get-Location) 'tmp\ui-last-full.png'), [System.Drawing.Imaging.ImageFormat]::Png)

$rect = New-Object Drv+RECT
$hwnd = [Drv]::Panel([uint32]$proc.Id, [ref]$rect)
$pad = 12
$cx = [Math]::Max(0, $rect.L - $pad)
$cy = [Math]::Max(0, $rect.T - $pad)
$cw = [Math]::Min($bounds.Width - $cx, ($rect.R - $rect.L) + 2 * $pad)
$ch = [Math]::Min($bounds.Height - $cy, ($rect.B - $rect.T) + 2 * $pad)
$crop = New-Object System.Drawing.Bitmap $cw, $ch
$g2 = [System.Drawing.Graphics]::FromImage($crop)
$g2.DrawImage($full, (New-Object System.Drawing.Rectangle(0, 0, $cw, $ch)), (New-Object System.Drawing.Rectangle($cx, $cy, $cw, $ch)), [System.Drawing.GraphicsUnit]::Pixel)
$g2.Dispose()
$crop.Save((Join-Path (Get-Location) $Out), [System.Drawing.Imaging.ImageFormat]::Png)
$full.Dispose(); $crop.Dispose()

$report = @{
    visible = [Drv]::IsWindowVisible($hwnd)
    rect = "$($rect.L),$($rect.T),$($rect.R),$($rect.B)"
    clip = if ($Clip) { $c = Get-Clipboard -Raw; if ($c -eq $Expect) { "match" } else { "mismatch len=$($c.Length)" } } else { $null }
}
$report | ConvertTo-Json | Set-Content (Join-Path (Get-Location) 'tmp\ui-report.json') -Encoding UTF8
$report | ConvertTo-Json
