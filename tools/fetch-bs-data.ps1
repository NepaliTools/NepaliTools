# Build-time BS calendar snapshot.
#
# Downloads four independent Bikram Sambat month-length tables, verifies pinned
# SHA256 hashes, normalises them to data/sources/<name>.csv, resolves the range
# 1975..2100 by majority vote, and writes data/bs-months.csv plus the audit trail
# in data/overrides.csv and data/SOURCES.md.
#
# Run:  powershell -ExecutionPolicy Bypass -File tools\fetch-bs-data.ps1
# Network is used here and nowhere else. The app itself never fetches anything.

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
Set-Location $root
$up = Join-Path $root 'data/upstream'
$srcDir = Join-Path $root 'data/sources'
New-Item -ItemType Directory -Force -Path $up, $srcDir | Out-Null

$sources = @(
    @{ Name = 'nepali-date-converter'; Version = '3.4.2';
       Url = 'https://registry.npmjs.org/nepali-date-converter/-/nepali-date-converter-3.4.2.tgz';
       Hash = 'C48B25414084C11F8114774FB978909675F6E13180D45844336DFBBBFA1AA12B';
       File = 'nepali-date-converter-3.4.2.tgz'; License = 'MIT' },
    @{ Name = 'bikram-sambat'; Version = '1.8.1';
       Url = 'https://registry.npmjs.org/bikram-sambat/-/bikram-sambat-1.8.1.tgz';
       Hash = '619AD3EA4A2132AE7D8E453C79499FE1E1188C5946E17EEA8AC1D947333CC744';
       File = 'bikram-sambat-1.8.1.tgz'; License = 'MIT' },
    @{ Name = 'nepali-datetime'; Version = '1.0.8.5';
       Url = 'https://files.pythonhosted.org/packages/d0/41/6ef290b5ab1671a0d421d105ee59d3d02b5c96004ba77f2ccb0b99a55e8f/nepali_datetime-1.0.8.5-py3-none-any.whl';
       Hash = 'ADEC683E06333482D31543D0D2BDF195CEF12505615EAF747BCAE0B82E5F6AD2';
       File = 'nepali_datetime.whl'; License = 'MIT' },
    @{ Name = 'nepdate'; Version = '1.0.0-rc.1';
       Url = 'https://crates.io/api/v1/crates/nepdate/1.0.0-rc.1/download';
       Hash = 'FA490330B7F250A9C58FFBD5BEE28E8EABE3565225E192F5DC568DC623EB59BD';
       File = 'nepdate.crate'; License = 'MIT (data: RajuPrasai/NepDate)' }
)

foreach ($s in $sources) {
    $path = Join-Path $up $s.File
    $need = $true
    if (Test-Path $path) { $need = ((Get-FileHash $path -Algorithm SHA256).Hash -ne $s.Hash) }
    if ($need) { curl.exe -sL $s.Url -o $path }
    $actual = (Get-FileHash $path -Algorithm SHA256).Hash
    if ($actual -ne $s.Hash) { throw "hash mismatch for $($s.Name): $actual" }
    $s.ActualHash = $actual
    $dir = Join-Path $up $s.Name
    if (-not (Test-Path $dir)) {
        New-Item -ItemType Directory -Force -Path $dir | Out-Null
        if ($s.File -like '*.tgz' -or $s.File -like '*.crate') { tar.exe -xzf $path -C $dir }
        elseif ($s.File -like '*.whl') {
            Copy-Item $path (Join-Path $up "$($s.Name).zip") -Force
            Expand-Archive (Join-Path $up "$($s.Name).zip") -DestinationPath $dir -Force
        }
    }
}

$months = @('Baisakh','Jestha','Ashadh','Shrawan','Bhadra','Ashwin','Kartik','Mangsir','Poush','Magh','Falgun','Chaitra')

# --- source A: nepali-date-converter, 2000..2090
$txt = Get-Content (Join-Path $up 'nepali-date-converter/package/dist/lib/date-config.js') -Raw
$ndc = @{}
foreach ($m in ([regex]"'(\d{4})':\s*\{([^}]*)\}").Matches($txt)) {
    $ndc[[int]$m.Groups[1].Value] = @([regex]::Matches($m.Groups[2].Value, '(\w+):\s*(\d+)') | ForEach-Object { [int]$_.Groups[2].Value })
}

# --- source B: bikram-sambat, 1970..2090 (2 bits per month, 29 + value)
$txt = Get-Content (Join-Path $up 'bikram-sambat/package/src/index.js') -Raw
$m = [regex]::Match($txt, 'ENCODED_MONTH_LENGTHS = \[([^\]]+)\]')
if (-not $m.Success) { throw 'bikram-sambat: ENCODED_MONTH_LENGTHS not found' }
$vals = $m.Groups[1].Value -split ',' | ForEach-Object { [int64]$_.Trim() }
$bik = @{}
for ($i = 0; $i -lt $vals.Count; $i++) {
    $bik[1970 + $i] = @(0..11 | ForEach-Object { 29 + (($vals[$i] -shr (2 * $_)) -band 3) })
}

# --- source C: nepali-datetime, 1975..2100 (CSV)
$py = @{}
Get-Content (Join-Path $up 'nepali-datetime/nepali_datetime/data/calendar_bs.csv') | Select-Object -Skip 1 | ForEach-Object {
    $p = $_ -split ','
    if ($p.Count -ne 13) { throw "nepali-datetime: bad row $_" }
    $py[[int]$p[0]] = @($p[1..12] | ForEach-Object { [int]$_ })
}

# --- source D: nepdate, 1901..2199 (packed 0x........, // YYYY: comment)
$nep = @{}
foreach ($line in Get-Content (Join-Path $up 'nepdate/nepdate-1.0.0-rc.1/src/generated/calendar.rs')) {
    if ($line -match '^\s*(0x[0-9A-Fa-f_]+),\s*//\s*(\d{4}):') {
        $v = [Convert]::ToUInt32(($matches[1] -replace '_', ''), 16)
        $nep[[int]$matches[2]] = @(0..11 | ForEach-Object { 29 + (($v -shr (2 * $_)) -band 3) })
    }
}

$all = @{ 'ndc' = $ndc; 'bik' = $bik; 'py' = $py; 'nep' = $nep }
foreach ($k in $all.Keys) { if ($all[$k].Count -eq 0) { throw "source $k parsed to zero years" } }

# --- normalised per-source snapshots (committed; tests read these)
foreach ($k in $all.Keys) {
    $file = Join-Path $srcDir "$(@{ ndc='nepali-date-converter'; bik='bikram-sambat'; py='nepali-datetime'; nep='nepdate' }[$k]).csv"
    $lines = @(($months | ForEach-Object { $_.ToLower() }) -join ',')
    $lines = @("bs_year,$($months -join ',')")
    foreach ($y in ($all[$k].Keys | Sort-Object)) {
        $lines += "$y,$($all[$k][$y] -join ',')"
    }
    Set-Content -Path $file -Value $lines -Encoding ASCII
}

# --- epoch cross-check: every lineage must place 1975-01-01 BS on 1918-04-13 AD
function NewYear([hashtable]$t, [int]$fromYear, [datetime]$fromAd, [int]$toYear) {
    $d = $fromAd
    for ($y = $fromYear; $y -lt $toYear; $y++) { $d = $d.AddDays(($t[$y] | Measure-Object -Sum).Sum) }
    $d
}
$e1 = NewYear $bik 1970 ([datetime]'1913-04-13') 1975
$e2 = NewYear $nep 1901 ([datetime]'1844-04-11') 1975
if ($e1 -ne [datetime]'1918-04-13' -or $e2 -ne [datetime]'1918-04-13') {
    throw "epoch mismatch: bik=$e1 nep=$e2 expected 1918-04-13"
}

# --- resolve 1975..2100
$minYear = 1975; $maxYear = 2100
$primary = [ordered]@{}
$over = New-Object System.Collections.Generic.List[string]
$over.Add('bs_year,month,chosen,rule,values,note')

for ($y = $minYear; $y -le $maxYear; $y++) {
    $row = @()
    for ($mi = 1; $mi -le 12; $mi++) {
        $v = [ordered]@{}
        foreach ($k in @('py', 'bik', 'ndc', 'nep')) {
            if ($all[$k].ContainsKey($y)) { $v[$k] = $all[$k][$y][$mi - 1] }
        }
        if ($v.Count -eq 0) { throw "no source covers $y" }
        $valsText = (($v.GetEnumerator() | ForEach-Object { "$($_.Key)=$($_.Value)" }) -join ';')
        $groups = @($v.Values | Group-Object | Sort-Object Count -Descending)
        if ($y -ge 2091) {
            $chosen = $v['nep']; $rule = 'single-source'
            $note = 'No gazette or second table for this year; nepdate is the only coherent source (nepali-datetime has an invalid 364-day 2096).'
        } elseif ($groups.Count -eq 1) {
            $chosen = [int]$groups[0].Name; $rule = 'unanimous'; $note = ''
        } elseif ($groups[0].Count -gt $groups[1].Count) {
            $chosen = [int]$groups[0].Name; $rule = 'majority'
            $note = 'Strict majority of the independent tables agrees; the dissent is recorded in values.'
        } else {
            $chosen = [int]$v['bik']; $rule = 'tie-bik'
            $note = 'Two independent lineages disagree 2-2; tie broken to the bikram-sambat/nepali-date-converter lineage, alternatives recorded.'
        }
        $row += $chosen
        if ($rule -ne 'unanimous') {
            $over.Add("$y,$mi,$chosen,$rule,$valsText,$note")
        }
    }
    if ((($row | Measure-Object -Sum).Sum) -notin 365, 366) { throw "year $y sums to $(($row | Measure-Object -Sum).Sum) days" }
    foreach ($d in $row) { if ($d -lt 29 -or $d -gt 32) { throw "year $y has month length $d" } }
    $primary["$y"] = $row
}

# --- fixture anchors, checked before writing anything
$check = @(
    @{ BS = 2000; AD = '1943-04-14' },
    @{ BS = 2082; AD = '2025-04-14' },
    @{ BS = 2083; AD = '2026-04-14' }
)
foreach ($c in $check) {
    $d = [datetime]'1918-04-13'
    for ($y = 1975; $y -lt $c.BS; $y++) { $d = $d.AddDays(($primary["$y"] | Measure-Object -Sum).Sum) }
    if ($d -ne [datetime]$c.AD) { throw "fixture failed: BS $($c.BS)-01-01 = $($d.ToString('yyyy-MM-dd')), expected $($c.AD)" }
}

# --- write primary
$lines = @("bs_year,$($months -join ',')")
foreach ($y in $primary.Keys) { $lines += "$y," + ($primary[$y] -join ',') }
Set-Content -Path (Join-Path $root 'data/bs-months.csv') -Value $lines -Encoding ASCII
Set-Content -Path (Join-Path $root 'data/overrides.csv') -Value $over -Encoding ASCII

# --- SOURCES.md
$retrieved = (Get-Date).ToUniversalTime().ToString('yyyy-MM-dd')
$rows = foreach ($s in $sources) {
    $k = @{ 'nepali-date-converter'='ndc'; 'bikram-sambat'='bik'; 'nepali-datetime'='py'; 'nepdate'='nep' }[$s.Name]
    $ys = $all[$k].Keys | Sort-Object
    "| $($s.Name) | $($s.Version) | $($ys[0])-$($ys[-1]) | ``$($s.ActualHash)`` | $($s.License) | <$($s.Url)> |"
}
$md = @"
# BS month-length sources

Snapshot generated by ``tools/fetch-bs-data.ps1`` on $retrieved (UTC). The application never
touches the network; it ships ``data/bs-months.csv`` via ``include_str!``.

| Source | Version | Range (BS) | SHA256 | License | URL |
|---|---|---|---|---|---|
$($rows -join "`n")

## How data/bs-months.csv is chosen

- Every month 1975-2100 is resolved from the four tables above.
- Unanimous agreement is taken as-is.
- Where a strict majority agrees, the majority wins.
- Where two lineages tie 2-2, the ``bikram-sambat``/``nepali-date-converter`` lineage wins;
  the alternative is recorded in ``data/overrides.csv``.
- 2091-2100 has no gazette and no second coherent table (``nepali-datetime`` contains an
  invalid 364-day year at 2096), so ``nepdate`` is authoritative there and those rows are
  marked ``single-source``. The UI labels years after 2090 as projected.

## Epoch cross-check

Each lineage's own epoch converts to the same anchor without sharing code:

- ``bikram-sambat``: 1970-01-01 BS = 1913-04-13 AD then 1975-01-01 = 1918-04-13
- ``nepdate``: 1901-01-01 BS = 1844-04-11 AD then 1975-01-01 = 1918-04-13
- ``nepali-datetime``: ``REFERENCE_DATE_AD`` = 1918-04-13

## Verified anchors

- BS 2000-01-01 = AD 1943-04-14
- BS 2082-01-01 = AD 2025-04-14
- BS 2083-01-01 = AD 2026-04-14

## Refresh

``````powershell
powershell -ExecutionPolicy Bypass -File tools\fetch-bs-data.ps1
``````

The script fails on any hash mismatch, epoch drift, impossible month length, year without
365-366 days, or fixture mismatch.
"@
Set-Content -Path (Join-Path $root 'data/SOURCES.md') -Value $md -Encoding UTF8

$disputed = ($over | Select-Object -Skip 1 | Where-Object { $_ -notmatch ',single-source,' }).Count
"wrote data/bs-months.csv ($($primary.Count) years), data/overrides.csv ($($over.Count - 1) rows; $disputed disputed), data/SOURCES.md"
"fixtures: BS 2000=1943-04-14, 2082=2025-04-14, 2083=2026-04-14 OK"
