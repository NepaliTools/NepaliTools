# Move a published release to a slower channel without rebuilding it.
#
# The installer and its signature stay identical; only the manifest a channel
# serves changes, so `stable` never offers bytes that were not first published
# on `alpha`. See docs/RELEASING.md.
param(
    [ValidateSet('alpha', 'beta', 'stable')][string]$From = 'alpha',
    [ValidateSet('alpha', 'beta', 'stable')][string]$To = 'beta',
    [string]$Site,
    [switch]$SkipSiteDeploy
)

$ErrorActionPreference = 'Stop'
if (-not $Site) {
    $Site = Join-Path (Split-Path $PSScriptRoot -Parent) '..\nepalitools-site'
}
if ($From -eq $To) { throw "From and To are both $From" }

$updates = Join-Path $Site 'updates'
$source = Join-Path $updates "$From.json"
if (-not (Test-Path $source)) { throw "No $From manifest at $source" }

$manifest = Get-Content $source -Raw | ConvertFrom-Json
if (-not $manifest.version) { throw "The $From manifest has no version" }

$json = Get-Content $source -Raw
[IO.File]::WriteAllText((Join-Path $updates "$To.json"), $json, (New-Object System.Text.UTF8Encoding($false)))
if ($To -eq 'alpha') {
    [IO.File]::WriteAllText((Join-Path $updates 'latest.json'), $json, (New-Object System.Text.UTF8Encoding($false)))
}
Write-Host "Promoted $($manifest.version) from $From to $To"

if (-not $SkipSiteDeploy) {
    Push-Location $Site
    try {
        powershell -ExecutionPolicy Bypass -File .\deploy.ps1
        if ($LASTEXITCODE -ne 0) { throw 'Site deploy failed' }
    }
    finally {
        Pop-Location
    }
    Write-Host "Live: https://nepalitools.pages.dev/updates/$To.json"
}
