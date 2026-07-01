@echo off
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
cd /d %~dp0..
cargo build --release --bin EngineVault
if errorlevel 1 exit /b 1
dir target\release\EngineVault.exe
start "" target\release\EngineVault.exe
