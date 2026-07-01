#Requires -RunAsAdministrator
# EngineVault — install + scan drive C (optional)
param(
    [char]$Drive = 'C'
)

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
& (Join-Path $ScriptDir "install.ps1")

$Exe = Join-Path $env:ProgramFiles "EngineVault\EngineVault.exe"
Write-Host "For a live scan, run EngineVault as Administrator and click Scan Drive $Drive`:" -ForegroundColor Cyan
