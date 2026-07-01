# EngineVault — build release binary
# Requires: Rust (rustup) + Visual Studio 2022 Build Tools (C++)

$ErrorActionPreference = "Stop"
$ProjectRoot = Split-Path -Parent $PSScriptRoot
Set-Location $ProjectRoot

Write-Host "Building EngineVault (release)..." -ForegroundColor Cyan
cargo build --release --bin EngineVault

$Exe = Join-Path $ProjectRoot "target\release\EngineVault.exe"
if (-not (Test-Path $Exe)) {
    Write-Error "Build failed — EngineVault.exe not found."
}

Write-Host ""
Write-Host "Built: $Exe" -ForegroundColor Green
Write-Host "Run installer:  .\install\install.ps1" -ForegroundColor Yellow
