param(
  [string]$Out = "target/pramaan-minimum-lovable"
)

$ErrorActionPreference = "Stop"

if (Test-Path $Out) {
  Remove-Item -Recurse -Force $Out
}

$base = "examples/vulnerable-python-pr/base"
$head = "examples/vulnerable-python-pr/weakened-test"

Write-Host "== Pramaan minimum lovable verifier loop =="
Write-Host "Demo: weakened assertion, ordinary CI green, Pramaan red"
Write-Host ""

Write-Host "1. Ordinary CI on AI-style PR branch"
python -m unittest discover -s $head -p "test_*.py"

Write-Host ""
Write-Host "2. Pramaan oracle evidence"
cargo run -p pramaan-cli -- oracle --base-repo $base --head-repo $head --out $Out

Write-Host ""
Write-Host "3. Confidence explanation"
cargo run -p pramaan-cli -- confidence explain $Out

Write-Host ""
Write-Host "4. Bundle integrity verification"
cargo run -p pramaan-cli -- bundle verify $Out

Write-Host ""
Write-Host "5. Policy explanation"
$policyOutput = cargo run -p pramaan-cli -- policy explain $Out --profile private-preview
$policyOutput | Tee-Object -FilePath (Join-Path $Out "policy-explain.txt")

$receiptPath = Join-Path $Out "receipts/oracle-integrity.receipt.json"
$receipt = Get-Content $receiptPath -Raw | ConvertFrom-Json
$manifest = Get-Content (Join-Path $Out "bundle.manifest.json") -Raw | ConvertFrom-Json
$confidencePath = Join-Path $Out "confidence.md"
$confidenceNote = if (Test-Path $confidencePath) { "confidence.md" } else { "not generated" }
$reportPath = Join-Path $Out "minimum-lovable-report.md"

$blockers = @()
if ($receipt.status -eq "failed") {
  $blockers += "- Oracle integrity failed: $($receipt.summary.details)"
}
if ($receipt.residual_risks.Count -gt 0) {
  $blockers += "- Residual oracle risks: $($receipt.residual_risks -join ', ')"
}
if ($blockers.Count -eq 0) {
  $blockers += "- No blocking oracle findings in this demo run."
}

$report = @"
# Pramaan Minimum Lovable Report

## Decision

Bundle status: **$($manifest.final_status)**

## Blockers First

$($blockers -join "`n")

## What Ran

- Ordinary Python unit tests on the AI-style branch passed.
- Pramaan oracle integrity compared base and head tests.
- Confidence explanation: $confidenceNote
- Bundle manifest: bundle.manifest.json
- Policy output: policy-explain.txt

## Reviewer Read

The production code is still wrong in the AI-style branch, but the test changed
from checking the exact discounted total to checking only a broad positive
value. That is why green CI is not enough.

Inspect:

- receipts/oracle-integrity.receipt.json
- oracle-diff.json
- confidence.md
- bundle.manifest.json

## Replay / Inspection Commands

~~~powershell
cargo run -p pramaan-cli -- bundle verify $Out
cargo run -p pramaan-cli -- policy explain $Out --profile private-preview
cargo run -p pramaan-cli -- confidence explain $Out
~~~

## Honesty Boundary

This report proves the bundle and receipts were generated for the demo. It does
not prove the code is correct, and it does not turn skipped or missing tools
into pass evidence.
"@

Set-Content -Path $reportPath -Value $report -Encoding UTF8

Write-Host ""
Write-Host "Report: $reportPath"
Write-Host "Bundle: $Out"
