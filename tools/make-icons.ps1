# Generates the application icon (the tray day badges live in make-badges.ps1).
# ASCII-only: Windows PowerShell 5.1 reads BOM-less scripts as ANSI, and the
# glyph is assembled from code points so no encoding can mangle it.
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing
$root = Split-Path -Parent $PSScriptRoot
Set-Location $root
New-Item -ItemType Directory -Force -Path crates\host\icons | Out-Null

$fontName = 'Nirmala UI'
foreach ($candidate in @('Nirmala UI', 'Nirmala', 'Mangal', 'Segoe UI')) {
    $test = [System.Drawing.Font]::new($candidate, 12)
    if ($test.Name -eq $candidate) { $fontName = $candidate; $test.Dispose(); break }
    $test.Dispose()
}
# "ने" = U+0928 (na) + U+0947 (e matra)
$glyph = [string][char]0x0928 + [string][char]0x0947

function New-RoundedPath([single]$x, [single]$y, [single]$w, [single]$h, [single]$r) {
    $p = [System.Drawing.Drawing2D.GraphicsPath]::new()
    $d = $r * 2
    $p.AddArc($x, $y, $d, $d, 180, 90)
    $p.AddArc($x + $w - $d, $y, $d, $d, 270, 90)
    $p.AddArc($x + $w - $d, $y + $h - $d, $d, $d, 0, 90)
    $p.AddArc($x, $y + $h - $d, $d, $d, 90, 90)
    $p.CloseFigure()
    $p
}

$app = [System.Drawing.Bitmap]::new(512, 512, [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
$g = [System.Drawing.Graphics]::FromImage($app)
$g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
$g.TextRenderingHint = [System.Drawing.Text.TextRenderingHint]::AntiAliasGridFit
$g.Clear([System.Drawing.Color]::Transparent)
$path = New-RoundedPath 24 24 464 464 104
$brush = [System.Drawing.SolidBrush]::new([System.Drawing.Color]::FromArgb(255, 185, 20, 32))
$g.FillPath($brush, $path)

$probe = [System.Drawing.Font]::new($fontName, 300, [System.Drawing.FontStyle]::Bold, [System.Drawing.GraphicsUnit]::Pixel)
$measured = $g.MeasureString($glyph, $probe)
$fitScale = [Math]::Min(400.0 / $measured.Width, 400.0 / $measured.Height)
$probe.Dispose()
$font = [System.Drawing.Font]::new($fontName, (300 * $fitScale), [System.Drawing.FontStyle]::Bold, [System.Drawing.GraphicsUnit]::Pixel)
$fmt = [System.Drawing.StringFormat]::new()
$fmt.Alignment = [System.Drawing.StringAlignment]::Center
$fmt.LineAlignment = [System.Drawing.StringAlignment]::Center
$g.DrawString($glyph, $font, [System.Drawing.Brushes]::White, [System.Drawing.RectangleF]::new(0, 0, 512, 512), $fmt)
$path.Dispose(); $brush.Dispose(); $font.Dispose(); $fmt.Dispose(); $g.Dispose()

$app.Save('crates\host\icons\icon.png', [System.Drawing.Imaging.ImageFormat]::Png)
$app.Save('crates\host\icons\128x128@2x.png', [System.Drawing.Imaging.ImageFormat]::Png)
$app.Dispose()

function Resize([string]$src, [int]$size, [string]$out) {
    $img = [System.Drawing.Image]::FromFile((Resolve-Path $src))
    $bmp = [System.Drawing.Bitmap]::new($size, $size)
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
    $g.DrawImage($img, 0, 0, $size, $size)
    $g.Dispose(); $img.Dispose()
    $bmp.Save($out, [System.Drawing.Imaging.ImageFormat]::Png)
    $bmp.Dispose()
}
Resize 'crates\host\icons\icon.png' 128 'crates\host\icons\128x128.png'
Resize 'crates\host\icons\icon.png' 32 'crates\host\icons\32x32.png'

# PNG-compressed ICO (single 256 px entry), accepted by Windows Vista and later.
$png = [IO.File]::ReadAllBytes((Resolve-Path 'crates\host\icons\128x128@2x.png'))
$ms = [IO.MemoryStream]::new()
$bw = [IO.BinaryWriter]::new($ms)
$bw.Write([UInt16]0); $bw.Write([UInt16]1); $bw.Write([UInt16]1)
$bw.Write([Byte]0); $bw.Write([Byte]0); $bw.Write([Byte]0); $bw.Write([Byte]0)
$bw.Write([UInt16]1); $bw.Write([UInt16]32)
$bw.Write([UInt32]$png.Length); $bw.Write([UInt32]22)
$bw.Write($png); $bw.Flush()
[IO.File]::WriteAllBytes((Join-Path $root 'crates\host\icons\icon.ico'), $ms.ToArray())
$bw.Dispose(); $ms.Dispose()

"icons: $((Get-ChildItem crates\host\icons\* | Measure-Object).Count) files"
