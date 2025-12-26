#!/bin/bash

# Read project info
PACKAGE_NAME=$(grep -m1 "^name" Cargo.toml | sed 's/name = "\(.*\)"/\1/' | tr -d ' ')
PACKAGE_VERSION=$(grep -m1 "^version" Cargo.toml | sed 's/version = "\(.*\)"/\1/' | tr -d ' ')

PROFILE=${1:-release}
TARGET_DIR="target/${PROFILE}"
PACKAGE_DIR="target/package"

# Determine executable name
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    EXE_NAME="pdf2other.exe"
else
    EXE_NAME="pdf2other"
fi

EXE_PATH="${TARGET_DIR}/${EXE_NAME}"

if [ ! -f "$EXE_PATH" ]; then
    echo "Error: executable not found: $EXE_PATH"
    exit 1
fi

# Create package directory
mkdir -p "$PACKAGE_DIR"

# Copy executable
cp "$EXE_PATH" "$PACKAGE_DIR/"
echo "✓ Executable copied: $PACKAGE_DIR/$EXE_NAME"

# Copy files from dep directory
if [ -d "dep" ]; then
    cp -r dep/* "$PACKAGE_DIR/"
    echo "✓ Copied files from dep directory"
else
    echo "Warning: dep directory not found; skipping dependency copy"
fi

# Determine target architecture and OS
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

# Create tar.gz archive
ARCHIVE_NAME="${PACKAGE_NAME}-${PACKAGE_VERSION}-${TARGET_OS}-${TARGET_ARCH}.tar.gz"
ARCHIVE_PATH="${TARGET_DIR}/${ARCHIVE_NAME}"

# Switch to project root directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$PROJECT_ROOT" || exit 1

# Create tar.gz archive
cd "$PACKAGE_DIR" || exit 1
tar -czf "$PROJECT_ROOT/$ARCHIVE_PATH" .
cd "$PROJECT_ROOT" || exit 1

echo "✓ Packaging complete; files are in: $PACKAGE_DIR"
echo "✓ Archive created: $ARCHIVE_PATH"

# Show file size
if [ -f "$ARCHIVE_PATH" ]; then
    SIZE=$(du -h "$ARCHIVE_PATH" | cut -f1)
    echo "✓ Archive size: $SIZE"
fi

