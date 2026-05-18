param(
  [int]$StartPhase = 0,
  [int]$EndPhase = 0
)

$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
$phasesDir = Join-Path $repoRoot ".planning\phases"

if (-not (Test-Path -LiteralPath $phasesDir)) {
  throw "Missing .planning\phases directory. Run from a Pramaan checkout."
}

$phaseNumbers = Get-ChildItem -LiteralPath $phasesDir -Directory |
  Where-Object { $_.Name -match '^(\d+)-' } |
  ForEach-Object { [int]$Matches[1] } |
  Sort-Object

if (-not $phaseNumbers) {
  throw "No numbered GSD phases found."
}

if ($EndPhase -eq 0) {
  $EndPhase = ($phaseNumbers | Select-Object -Last 1)
}

if ($StartPhase -eq 0) {
  $aggregate = Join-Path $repoRoot ".planning\PHASE_AGGREGATE.md"
  $state = Join-Path $repoRoot ".planning\STATE.md"
  if (Test-Path -LiteralPath $state) {
    $stateText = Get-Content -LiteralPath $state -Raw
    if ($stateText -match 'Current Phase\s*\r?\n\s*\r?\nPhase\s+(\d+)') {
      $StartPhase = [int]$Matches[1]
    }
  }
  if (Test-Path -LiteralPath $aggregate) {
    $done = Select-String -LiteralPath $aggregate -Pattern '^\|\s*(\d+)\s*\|\s*(PASS|PASS_WITH_RISKS)\s*\|' |
      ForEach-Object { [int]$_.Matches[0].Groups[1].Value }
    $nextIncomplete = ($phaseNumbers | Where-Object { $_ -notin $done } | Select-Object -First 1)
    if ($nextIncomplete) {
      $StartPhase = $nextIncomplete
    }
  }
  if ($StartPhase -eq 0) {
    $StartPhase = ($phaseNumbers | Select-Object -First 1)
  }
}

Write-Output "Run .planning/AUTONOMOUS_RECURSIVE_GSD_COMMAND.md with START_PHASE=$StartPhase and END_PHASE=$EndPhase."
