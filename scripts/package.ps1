param(
    [string]$Profile = "release"
)

# 读取项目信息
$cargoToml = Get-Content "Cargo.toml" -Raw
$packageName = if ($cargoToml -match 'name\s*=\s*"([^"]+)"') { $matches[1] } else { "pdf2other" }
$packageVersion = if ($cargoToml -match 'version\s*=\s*"([^"]+)"') { $matches[1] } else { "0.1.0" }

$targetDir = "target\$Profile"
$packageDir = "target\package"
$exeName = "pdf2other.exe"
$exePath = Join-Path $targetDir $exeName

if (-not (Test-Path $exePath)) {
    Write-Host "错误: 可执行文件不存在: $exePath" -ForegroundColor Red
    exit 1
}

# 创建打包目录
if (-not (Test-Path $packageDir)) {
    New-Item -ItemType Directory -Path $packageDir -Force | Out-Null
}

# 复制可执行文件到打包目录
$destExe = Join-Path $packageDir $exeName
Copy-Item -Path $exePath -Destination $destExe -Force
Write-Host "✓ 已复制可执行文件: $destExe" -ForegroundColor Green

# 复制dep目录下的所有文件到打包目录
$depDir = "dep"
if (Test-Path $depDir) {
    Copy-Item -Path "$depDir\*" -Destination $packageDir -Recurse -Force
    Write-Host "✓ 已复制dep目录下的文件" -ForegroundColor Green
} else {
    Write-Host "警告: dep目录不存在，跳过复制依赖文件" -ForegroundColor Yellow
}

# 确定目标架构
$targetArch = if ($env:PROCESSOR_ARCHITECTURE -eq "AMD64" -or $env:PROCESSOR_ARCHITECTURE -eq "x86_64") {
    "x64"
} elseif ($env:PROCESSOR_ARCHITECTURE -eq "ARM64") {
    "arm64"
} else {
    "x86"
}

# 创建 ZIP 文件
$archiveName = "$packageName-$packageVersion-windows-$targetArch.zip"
$archivePath = Join-Path $targetDir $archiveName

# 删除已存在的 ZIP 文件
if (Test-Path $archivePath) {
    Remove-Item $archivePath -Force
}

# 创建 ZIP 文件
Compress-Archive -Path "$packageDir\*" -DestinationPath $archivePath -Force

Write-Host "✓ 打包完成，文件位于: $packageDir" -ForegroundColor Green
Write-Host "✓ ZIP 文件已创建: $archivePath" -ForegroundColor Green

# 显示文件大小
if (Test-Path $archivePath) {
    $size = (Get-Item $archivePath).Length / 1MB
    Write-Host "✓ ZIP 文件大小: $([math]::Round($size, 2)) MB" -ForegroundColor Green
}

