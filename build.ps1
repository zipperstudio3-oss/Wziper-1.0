$ErrorActionPreference = "Stop"
Write-Host "Building Wziper for this Windows machine..."
cargo build --release
if ($LASTEXITCODE -ne 0) { throw "Cargo build failed." }
New-Item -ItemType Directory -Force -Path "dist" | Out-Null
Copy-Item "target\release\wziper.exe" "dist\Wziper.exe" -Force
Write-Host "Build complete: dist\Wziper.exe"
Write-Host "Next: compile installer\Wziper.iss with Inno Setup 6."
