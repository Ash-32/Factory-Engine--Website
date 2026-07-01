# Go live on factory-engine.com
# Run from the web folder:  .\scripts\go-live.ps1

$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot\..

Write-Host "`n=== Factory Engine — Cloudflare deploy ===" -ForegroundColor Cyan

if (-not (Test-Path ".env")) {
  Write-Host "Create web/.env from .env.example with CLOUDFLARE_API_TOKEN and CLOUDFLARE_ACCOUNT_ID" -ForegroundColor Yellow
  Write-Host "Token: https://dash.cloudflare.com/profile/api-tokens (Edit Cloudflare Workers template)`n"
  exit 1
}

Write-Host "Deploying to factory-engine.com ...`n" -ForegroundColor Cyan
npm run deploy:domain
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "`nLive at https://factory-engine.com and https://www.factory-engine.com`n" -ForegroundColor Green
