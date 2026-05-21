param(
    [string]$PilotRoot = "target/pramaan-phase26-pilots",
    [switch]$SkipClone
)

$ErrorActionPreference = "Stop"

$Workspace = (Resolve-Path ".").Path
$PramaanExe = Join-Path $Workspace "target/debug/pramaan.exe"
if (!(Test-Path $PramaanExe)) {
    cargo build -p pramaan-cli
}

$PilotRoot = Join-Path $Workspace $PilotRoot
$CloneRoot = Join-Path $PilotRoot "repos"
$BundleRoot = Join-Path $PilotRoot "bundles"
New-Item -ItemType Directory -Force -Path $CloneRoot, $BundleRoot | Out-Null

$Repos = @(
    @{ slug = "python-packaging"; language = "Python"; url = "https://github.com/pypa/packaging.git" },
    @{ slug = "typescript-is"; language = "TypeScript"; url = "https://github.com/sindresorhus/is.git" },
    @{ slug = "rust-itoa"; language = "Rust"; url = "https://github.com/dtolnay/itoa.git" }
)

$Rows = @()
foreach ($RepoInfo in $Repos) {
    $Slug = $RepoInfo.slug
    $RepoPath = Join-Path $CloneRoot $Slug
    $BundlePath = Join-Path $BundleRoot $Slug
    $BaseWorktree = Join-Path $PilotRoot "$Slug-base"
    $HeadWorktree = Join-Path $PilotRoot "$Slug-head"

    if (!(Test-Path $RepoPath)) {
        if ($SkipClone) {
            throw "Missing $RepoPath and -SkipClone was set."
        }
        git clone --depth 20 $RepoInfo.url $RepoPath
    }

    Push-Location $RepoPath
    try {
        git fetch --depth 20 origin | Out-Null
        $Head = (git rev-parse HEAD).Trim()
        $Base = (git rev-parse HEAD~1).Trim()
        $ChangedFiles = @(git diff --name-only $Base $Head | Where-Object { $_ })
        $Changed = ($ChangedFiles -join ",")

        foreach ($Worktree in @($BaseWorktree, $HeadWorktree)) {
            if (Test-Path $Worktree) {
                git worktree remove --force $Worktree | Out-Null
            }
        }
        git worktree add --detach $BaseWorktree $Base | Out-Null
        git worktree add --detach $HeadWorktree $Head | Out-Null

        $ChangedArgs = @()
        foreach ($File in $ChangedFiles) {
            $ChangedArgs += @("--changed-file", $File)
        }

        $Stages = @(
            @{ name = "verify"; args = @("verify", "--base", $Base, "--head", $Head, "--out", (Join-Path $BundlePath "verify")) },
            @{ name = "static"; args = @("static-checks", "--repo", $HeadWorktree, "--out", (Join-Path $BundlePath "static")) },
            @{ name = "oracle"; args = @("oracle", "--base-repo", $BaseWorktree, "--head-repo", $HeadWorktree, "--out", (Join-Path $BundlePath "oracle")) },
            @{ name = "fuzz"; args = @("fuzz", "--base-repo", $BaseWorktree, "--head-repo", $HeadWorktree, "--out", (Join-Path $BundlePath "fuzz")) },
            @{ name = "mutation"; args = @("mutation", "--repo", $HeadWorktree, "--out", (Join-Path $BundlePath "mutation"), "--timeout-ms", "1000") + $ChangedArgs }
        )

        $Result = [ordered]@{
            slug = $Slug
            language = $RepoInfo.language
            url = $RepoInfo.url
            base = $Base
            head = $Head
            changed = $Changed
            bundle = $BundlePath
        }

        foreach ($Stage in $Stages) {
            $Timer = [Diagnostics.Stopwatch]::StartNew()
            & $PramaanExe @($Stage.args) | Out-Null
            $ExitCode = $LASTEXITCODE
            $Timer.Stop()
            $Result["$($Stage.name)_exit"] = $ExitCode
            $Result["$($Stage.name)_ms"] = $Timer.ElapsedMilliseconds
        }

        $Rows += [pscustomobject]$Result
    }
    finally {
        Pop-Location
    }
}

$ResultsPath = Join-Path $PilotRoot "pilot-results.json"
$Rows | ConvertTo-Json -Depth 5 | Set-Content -Path $ResultsPath
$Rows | Format-Table -AutoSize
Write-Host "Wrote $ResultsPath"
