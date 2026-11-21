# DPI-aware screen capture. The process must become per-monitor aware before .NET
# initialises the screen bounds, otherwise a 125% display reports 1536x864 and the
# capture silently shows only the top-left of the 1920x1080 screen.
param([string]$Out = "tmp\screen-full.png")
$ErrorActionPreference = 'Stop'
Add-Type @"
using System;
using System.Runtime.InteropServices;
public class DpiShim {
  [DllImport("user32.dll")] public static extern bool SetProcessDpiAwarenessContext(IntPtr value);
}
"@
[void][DpiShim]::SetProcessDpiAwarenessContext([IntPtr](-4))
Add-Type -AssemblyName System.Windows.Forms, System.Drawing
$bounds = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds
$bmp = New-Object System.Drawing.Bitmap $bounds.Width, $bounds.Height
$g = [System.Drawing.Graphics]::FromImage($bmp)
$g.CopyFromScreen($bounds.X, $bounds.Y, 0, 0, $bmp.Size)
$path = Join-Path (Get-Location) $Out
$bmp.Save($path, [System.Drawing.Imaging.ImageFormat]::Png)
$g.Dispose(); $bmp.Dispose()
"$($bounds.Width)x$($bounds.Height) -> $path"
