# Build, sign, publish and deploy a NepaliToolkit alpha release.
#
# The private updater key is read from $HOME\.tauri\nepalitoolkit.key. It never
# goes into the repository, and this script never prints it. See docs/RELEASING.md.
param(
    [Parameter(Mandatory = $true)][string]$Version,
    [ValidateSet('alpha', 'beta', 'stable')][string]$Channel = 'alpha',
    [string]$NotesFile,
    [string]$KeyPath = (Join-Path $HOME '.tauri\nepalitoolkit.key'),
    [string]$Site,
    [switch]$SkipPublish,
    [switch]$SkipSiteDeploy
)

$ErrorActionPreference = 'Stop'
$root = Split-Path $PSScriptRoot -Parent
Set-Location $root

$tag = if ($Channel -eq 'stable') { "v$Version" } else { "v$Version-$Channel" }
if (-not $NotesFile) { $NotesFile = "docs\release-notes-$Version-$Channel.md" }
if (-not $Site) { $Site = Join-Path $root '..\nepalitools-site' }
if (-not (Test-Path $KeyPath)) { throw "Updater key not found at $KeyPath" }
if (-not (Test-Path $NotesFile)) { throw "Release notes not found at $NotesFile" }
if (-not (Test-Path $Site)) { throw "Site checkout not found at $Site" }

$configVersion = (Get-Content 'crates\host\tauri.conf.json' -Raw | ConvertFrom-Json).version
$cargoVersion = (Select-String -Path 'Cargo.toml' -Pattern '^version = "(.+)"$').Matches[0].Groups[1].Value
if ($configVersion -ne $Version -or $cargoVersion -ne $Version) {
    throw "Version mismatch: requested $Version, tauri.conf.json $configVersion, Cargo.toml $cargoVersion"
}

Write-Host "Building $Version"
$env:TAURI_SIGNING_PRIVATE_KEY = $KeyPath
$env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD = ''
npx --yes @tauri-apps/cli@^2 build --bundles nsis
if ($LASTEXITCODE -ne 0) { throw "Tauri build failed" }

$setup = "target\release\bundle\nsis\NepaliToolkit_${Version}_x64-setup.exe"
$sig = "$setup.sig"
if (-not (Test-Path $setup) -or -not (Test-Path $sig)) { throw "Signed installer or signature missing" }

$portableExe = 'target\release\nepalitoolkit.exe'
if (-not (Test-Path $portableExe)) { throw "Portable executable missing" }

$out = 'dist\release'
New-Item -ItemType Directory -Force -Path $out | Out-Null
$setupAssetName = "NepaliToolkit-$Version-alpha-win64-setup.exe"
$portableAssetName = "NepaliToolkit-$Version-alpha-win64-portable.zip"
$setupAsset = Join-Path $out $setupAssetName
$portableZip = Join-Path $out $portableAssetName
$portableDir = Join-Path $out "NepaliToolkit-$Version-alpha-win64-portable"

if (Test-Path $portableDir) { Remove-Item $portableDir -Recurse -Force }
New-Item -ItemType Directory -Force -Path $portableDir | Out-Null
Copy-Item $setup $setupAsset -Force
Copy-Item $sig "$setupAsset.sig" -Force
Copy-Item $portableExe (Join-Path $portableDir 'NepaliToolkit.exe') -Force
# Compress-Archive stalls on this machine with a 5 MB folder; ZipFile returns.
if (Test-Path $portableZip) { Remove-Item $portableZip -Force }
Add-Type -AssemblyName System.IO.Compression.FileSystem
[IO.Compression.ZipFile]::CreateFromDirectory($portableDir, $portableZip)

$signature = (Get-Content $sig -Raw).Trim()
$notes = Get-Content $NotesFile -Raw
$url = "https://github.com/NepaliTools/NepaliTools/releases/download/$tag/$setupAssetName"
$pubDate = (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ssZ')

# ConvertTo-Json stalls indefinitely on this machine once the notes string is in
# the object, so the manifest is encoded by hand. It is four fields wide.
function ConvertTo-JsonString([string]$value) {
    $sb = New-Object System.Text.StringBuilder
    [void]$sb.Append('"')
    foreach ($ch in $value.ToCharArray()) {
        switch ($ch) {
            '"' { [void]$sb.Append('\"'); break }
            '\' { [void]$sb.Append('\\'); break }
            "`b" { [void]$sb.Append('\b'); break }
            "`f" { [void]$sb.Append('\f'); break }
            "`n" { [void]$sb.Append('\n'); break }
            "`r" { [void]$sb.Append('\r'); break }
            "`t" { [void]$sb.Append('\t'); break }
            default {
                if ([int]$ch -lt 32) { [void]$sb.Append(('\u{0:x4}' -f [int]$ch)) } else { [void]$sb.Append($ch) }
            }
        }
    }
    [void]$sb.Append('"')
    $sb.ToString()
}

$json = @"
{
  "version": "$Version",
  "notes": $(ConvertTo-JsonString $notes),
  "pub_date": "$pubDate",
  "platforms": {
    "windows-x86_64": {
      "signature": $(ConvertTo-JsonString $signature),
      "url": $(ConvertTo-JsonString $url)
    }
  }
}
"@
$null = $json | ConvertFrom-Json
$latest = Join-Path $out 'latest.json'
[IO.File]::WriteAllText($latest, $json, (New-Object System.Text.UTF8Encoding($false)))

$siteUpdates = Join-Path $Site 'updates'
New-Item -ItemType Directory -Force -Path $siteUpdates | Out-Null
$channelFile = Join-Path $siteUpdates "$Channel.json"
[IO.File]::WriteAllText($channelFile, $json, (New-Object System.Text.UTF8Encoding($false)))
if ($Channel -eq 'alpha') {
    # 0.1.2 predates channels and still reads this one file.
    [IO.File]::WriteAllText((Join-Path $siteUpdates 'latest.json'), $json, (New-Object System.Text.UTF8Encoding($false)))
}

$checksumFiles = @($setupAsset, $portableZip, $latest, "$setupAsset.sig")
$checksums = foreach ($file in $checksumFiles) {
    $text = certutil -hashfile $file SHA256
    $hash = ($text | Where-Object { $_ -match '^[0-9a-fA-F ]{64,}$' } | Select-Object -First 1) -replace ' ', ''
    "$hash  $(Split-Path $file -Leaf)"
}
[IO.File]::WriteAllText((Join-Path $out 'SHA256SUMS.txt'), ($checksums -join "`r`n") + "`r`n", (New-Object System.Text.UTF8Encoding($false)))

Write-Host "Artifacts in $out"
Get-ChildItem $out -File | Select-Object Name, Length

if (-not $SkipPublish) {
    $assets = @($setupAsset, $portableZip, $latest, "$setupAsset.sig", (Join-Path $out 'SHA256SUMS.txt'))
    $releaseArgs = @('release', 'create', $tag, '--repo', 'NepaliTools/NepaliTools', '--title', "NepaliToolkit $Version $Channel", '--notes-file', $NotesFile)
    if ($Channel -ne 'stable') { $releaseArgs += '--prerelease' }
    $releaseArgs += $assets
    gh @releaseArgs
    if ($LASTEXITCODE -ne 0) { throw "gh release create failed" }
    Write-Host "Release: https://github.com/NepaliTools/NepaliTools/releases/tag/$tag"
}

if (-not $SkipSiteDeploy) {
    Push-Location $Site
    try {
        powershell -ExecutionPolicy Bypass -File .\deploy.ps1
        if ($LASTEXITCODE -ne 0) { throw "Site deploy failed" }
    }
    finally {
        Pop-Location
    }
    Write-Host "Update manifest: https://nepalitools.pages.dev/updates/latest.json"
}
