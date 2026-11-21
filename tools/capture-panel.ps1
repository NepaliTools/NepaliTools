# Brings the panel up with the global hotkey and captures it, so the capture
# process itself does not steal focus before the shot.
$ErrorActionPreference = 'Stop'
Add-Type @"
using System;
using System.Text;
using System.Runtime.InteropServices;
public class Cap {
  public delegate bool EnumProc(IntPtr h, IntPtr l);
  [DllImport("user32.dll")] public static extern bool SetProcessDpiAwarenessContext(IntPtr value);
  [DllImport("user32.dll")] public static extern bool EnumWindows(EnumProc cb, IntPtr l);
  [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
  [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
  [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetClassNameW(IntPtr h, StringBuilder s, int n);
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern void keybd_event(byte b, byte s, uint f, UIntPtr e);
  public struct RECT { public int L; public int T; public int R; public int B; }
  public static IntPtr Panel(uint target, out RECT rect) {
    IntPtr found = IntPtr.Zero;
    RECT box = new RECT();
    EnumWindows((h, l) => {
      uint pid; GetWindowThreadProcessId(h, out pid);
      if (pid == target) {
        var c = new StringBuilder(256); GetClassNameW(h, c, 256);
        if (c.ToString() == "Tauri Window") { found = h; GetWindowRect(h, out box); }
      }
      return true;
    }, IntPtr.Zero);
    rect = box;
    return found;
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
}
"@
[void][Cap]::SetProcessDpiAwarenessContext([IntPtr](-4))
Add-Type -AssemblyName System.Windows.Forms, System.Drawing

$proc = Get-Process nepalitoolkit -ErrorAction SilentlyContinue | Select-Object -First 1
if (-not $proc) { throw "NepaliToolkit is not running" }
$rect = New-Object Cap+RECT
$hwnd = [Cap]::Panel([uint32]$proc.Id, [ref]$rect)
if ($hwnd -eq [IntPtr]::Zero) { throw "panel window not found" }
if (-not [Cap]::IsWindowVisible($hwnd)) {
    [Cap]::Hotkey()
    Start-Sleep -Milliseconds 1200
}

$bounds = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds
$full = New-Object System.Drawing.Bitmap $bounds.Width, $bounds.Height
$g = [System.Drawing.Graphics]::FromImage($full)
$g.CopyFromScreen($bounds.X, $bounds.Y, 0, 0, $full.Size)
$g.Dispose()
$full.Save((Join-Path (Get-Location) 'tmp\screen-panel.png'), [System.Drawing.Imaging.ImageFormat]::Png)

$w = $rect.R - $rect.L
$h = $rect.B - $rect.T
$pad = 12
$x = [Math]::Max(0, $rect.L - $pad)
$y = [Math]::Max(0, $rect.T - $pad)
$cw = [Math]::Min($bounds.Width - $x, $w + 2 * $pad)
$ch = [Math]::Min($bounds.Height - $y, $h + 2 * $pad)
$crop = New-Object System.Drawing.Bitmap $cw, $ch
$g2 = [System.Drawing.Graphics]::FromImage($crop)
$g2.DrawImage($full, (New-Object System.Drawing.Rectangle(0, 0, $cw, $ch)), (New-Object System.Drawing.Rectangle($x, $y, $cw, $ch)), [System.Drawing.GraphicsUnit]::Pixel)
$g2.Dispose()
$crop.Save((Join-Path (Get-Location) 'tmp\panel-live.png'), [System.Drawing.Imaging.ImageFormat]::Png)
"visible=$([Cap]::IsWindowVisible($hwnd)) rect=$($rect.L),$($rect.T),$($rect.R),$($rect.B) crop=${cw}x${ch}"
$full.Dispose(); $crop.Dispose()
