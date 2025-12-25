#!/bin/bash

# 读取项目信息
PACKAGE_NAME=$(grep -m1 "^name" Cargo.toml | sed 's/name = "\(.*\)"/\1/' | tr -d ' ')
PACKAGE_VERSION=$(grep -m1 "^version" Cargo.toml | sed 's/version = "\(.*\)"/\1/' | tr -d ' ')

PROFILE=${1:-release}
TARGET_DIR="target/${PROFILE}"
PACKAGE_DIR="target/package"

# 确定可执行文件名
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    EXE_NAME="pdf2other.exe"
else
    EXE_NAME="pdf2other"
fi

EXE_PATH="${TARGET_DIR}/${EXE_NAME}"

if [ ! -f "$EXE_PATH" ]; then
    echo "错误: 可执行文件不存在: $EXE_PATH"
    exit 1
fi

# 创建打包目录
mkdir -p "$PACKAGE_DIR"

# 复制可执行文件
cp "$EXE_PATH" "$PACKAGE_DIR/"
echo "✓ 已复制可执行文件: $PACKAGE_DIR/$EXE_NAME"

# 复制dep目录
if [ -d "dep" ]; then
    cp -r dep/* "$PACKAGE_DIR/"
    echo "✓ 已复制dep目录下的文件"
else
    echo "警告: dep目录不存在，跳过复制依赖文件"
fi

# 确定目标架构和操作系统
TARGET_ARCH=$(uname -m)
if [ "$TARGET_ARCH" = "x86_64" ]; then
    TARGET_ARCH="x64"
elif [ "$TARGET_ARCH" = "aarch64" ]; then
    TARGET_ARCH="arm64"
fi

TARGET_OS=$(uname -s | tr '[:upper:]' '[:lower:]')
if [ "$TARGET_OS" = "darwin" ]; then
    TARGET_OS="macos"
fi

# 创建 tar.gz 文件
ARCHIVE_NAME="${PACKAGE_NAME}-${PACKAGE_VERSION}-${TARGET_OS}-${TARGET_ARCH}.tar.gz"
ARCHIVE_PATH="${TARGET_DIR}/${ARCHIVE_NAME}"

# 切换到项目根目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$PROJECT_ROOT" || exit 1

# 创建 tar.gz 文件
cd "$PACKAGE_DIR" || exit 1
tar -czf "$PROJECT_ROOT/$ARCHIVE_PATH" .
cd "$PROJECT_ROOT" || exit 1

echo "✓ 打包完成，文件位于: $PACKAGE_DIR"
echo "✓ 压缩包已创建: $ARCHIVE_PATH"

# 显示文件大小
if [ -f "$ARCHIVE_PATH" ]; then
    SIZE=$(du -h "$ARCHIVE_PATH" | cut -f1)
    echo "✓ 压缩包大小: $SIZE"
fi

