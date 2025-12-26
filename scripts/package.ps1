param(
    [string]$Profile = "release"
)

# Read project info
$cargoToml = Get-Content "Cargo.toml" -Raw
$packageName = if ($cargoToml -match 'name\s*=\s*"([^"]+)"') { $matches[1] } else { "pdf2other" }
$packageVersion = if ($cargoToml -match 'version\s*=\s*"([^"]+)"') { $matches[1] } else { "0.1.0" }

$targetDir = "target\$Profile"
$packageDir = "target\package"
$exeName = "pdf2other.exe"
$exePath = Join-Path $targetDir $exeName

if (-not (Test-Path $exePath)) {
    Write-Host "Error: executable not found: $exePath" -ForegroundColor Red
    exit 1
}

# Create package directory
if (-not (Test-Path $packageDir)) {
    New-Item -ItemType Directory -Path $packageDir -Force | Out-Null
}

# Copy executable into package directory
$destExe = Join-Path $packageDir $exeName
Copy-Item -Path $exePath -Destination $destExe -Force
Write-Host "✓ Executable copied: $destExe" -ForegroundColor Green

# Copy all files from dep directory into package directory
$depDir = "dep"
if (Test-Path $depDir) {
    Copy-Item -Path "$depDir\*" -Destination $packageDir -Recurse -Force
    Write-Host "✓ Copied files from dep directory" -ForegroundColor Green
} else {
    Write-Host "Warning: dep directory not found; skipping dependency copy" -ForegroundColor Yellow
}

# Determine target architecture
$targetArch = if ($env:PROCESSOR_ARCHITECTURE -eq "AMD64" -or $env:PROCESSOR_ARCHITECTURE -eq "x86_64") {
    "x64"
} elseif ($env:PROCESSOR_ARCHITECTURE -eq "ARM64") {
    "arm64"
} else {
    "x86"
}

# Create ZIP archive
$archiveName = "$packageName-$packageVersion-windows-$targetArch.zip"
$archivePath = Join-Path $targetDir $archiveName

# Remove existing ZIP archive
if (Test-Path $archivePath) {
    Remove-Item $archivePath -Force
}

# Create ZIP archive
Compress-Archive -Path "$packageDir\*" -DestinationPath $archivePath -Force

Write-Host "✓ Packaging complete; files are in: $packageDir" -ForegroundColor Green
Write-Host "✓ ZIP archive created: $archivePath" -ForegroundColor Green

# Show file size
if (Test-Path $archivePath) {
    $size = (Get-Item $archivePath).Length / 1MB
    Write-Host "✓ ZIP archive size: $([math]::Round($size, 2)) MB" -ForegroundColor Green
}

