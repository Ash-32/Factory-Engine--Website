# EngineVault Windows Installer
# Run from an elevated PowerShell if installing to Program Files:
#   Set-ExecutionPolicy Bypass -Scope Process; .\install\install.ps1

$ErrorActionPreference = "Stop"

$ProductName = "EngineVault"
$InstallDir  = Join-Path $env:ProgramFiles $ProductName
$ProjectRoot = Split-Path -Parent $PSScriptRoot
$SourceExe   = Join-Path $ProjectRoot "target\release\EngineVault.exe"
$RulesSrc    = Join-Path $ProjectRoot "rules"

if (-not (Test-Path $SourceExe)) {
    Write-Host "EngineVault.exe not found — building first..." -ForegroundColor Yellow
    & (Join-Path $ProjectRoot "install\build.ps1")
}

Write-Host "Installing $ProductName to $InstallDir ..." -ForegroundColor Cyan

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $InstallDir "rules") | Out-Null

Copy-Item -Force $SourceExe (Join-Path $InstallDir "EngineVault.exe")
Copy-Item -Force (Join-Path $RulesSrc "classification.toml") (Join-Path $InstallDir "rules\classification.toml")

# Start Menu shortcut
$WshShell = New-Object -ComObject WScript.Shell
$ShortcutDir = [Environment]::GetFolderPath("Programs")
$Shortcut = $WshShell.CreateShortcut((Join-Path $ShortcutDir "$ProductName.lnk"))
$Shortcut.TargetPath = Join-Path $InstallDir "EngineVault.exe"
$Shortcut.WorkingDirectory = $InstallDir
$Shortcut.Description = "EngineVault — Engineering File Intelligence"
$Shortcut.Save()

# Uninstall script
@'
$InstallDir = Join-Path $env:ProgramFiles "EngineVault"
Remove-Item -Recurse -Force $InstallDir -ErrorAction SilentlyContinue
$lnk = Join-Path ([Environment]::GetFolderPath("Programs")) "EngineVault.lnk"
Remove-Item -Force $lnk -ErrorAction SilentlyContinue
Write-Host "EngineVault uninstalled."
'@ | Set-Content (Join-Path $InstallDir "uninstall.ps1")

Write-Host ""
Write-Host "Installed successfully!" -ForegroundColor Green
Write-Host "  Executable: $(Join-Path $InstallDir 'EngineVault.exe')"
Write-Host "  Start Menu: $ProductName"
Write-Host ""
Write-Host "Launch EngineVault and click 'Load Demo' or 'Scan Drive' (Admin)." -ForegroundColor Yellow

# Offer to launch
$launch = Read-Host "Launch EngineVault now? (Y/n)"
if ($launch -ne "n" -and $launch -ne "N") {
    Start-Process (Join-Path $InstallDir "EngineVault.exe")
}
