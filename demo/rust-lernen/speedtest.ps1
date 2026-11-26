# Baut Rust und Go optimiert und lässt alle drei Speedtests nacheinander laufen.
# Starten: .\speedtest.ps1
$ErrorActionPreference = 'Stop'
$env:Path = "$env:USERPROFILE\.cargo\bin;C:\Program Files\Go\bin;$env:Path"
$root = $PSScriptRoot

Write-Host "Baue Rust (release) ..." -ForegroundColor DarkGray
cargo build --release --quiet --manifest-path "$root\speedtest\rust\Cargo.toml"
if ($LASTEXITCODE -ne 0) { throw "Rust-Build fehlgeschlagen" }

Write-Host "Baue Go ..." -ForegroundColor DarkGray
go build -o "$root\speedtest\go\speedtest.exe" "$root\speedtest\go\speedtest.go"
if ($LASTEXITCODE -ne 0) { throw "Go-Build fehlgeschlagen" }

Write-Host ""
& "$root\speedtest\rust\target\release\speedtest.exe"
Write-Host ""
& "$root\speedtest\go\speedtest.exe"
Write-Host ""
python "$root\speedtest\python\speedtest.py"
