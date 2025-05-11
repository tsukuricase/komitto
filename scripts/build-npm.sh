#!/bin/bash
set -e

# 版本号
VERSION=$(grep '^version' Cargo.toml | sed 's/.*"\(.*\)"/\1/')
echo "Building komitto version $VERSION"

# 获取项目根目录的绝对路径
PROJECT_ROOT=$(pwd)

# 创建发布目录
mkdir -p "$PROJECT_ROOT/releases"

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  PLATFORM="x86_64-unknown-linux-gnu"
  TARGET="linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
  if [[ "$(uname -m)" == "arm64" ]]; then
    PLATFORM="aarch64-apple-darwin"
    TARGET="macosarm64"
  else
    PLATFORM="x86_64-apple-darwin"
    TARGET="macos"
  fi
elif [[ "$OSTYPE" == "msys"* || "$OSTYPE" == "win"* || "$OSTYPE" == "cygwin"* ]]; then
  PLATFORM="x86_64-pc-windows-msvc"
  TARGET="win64"
else
  echo "Unsupported platform: $OSTYPE"
  exit 1
fi

echo "Building for $PLATFORM"

# 清理旧构建
rm -rf "$PROJECT_ROOT/releases/"*

# 确保 npm 目录存在
mkdir -p "$PROJECT_ROOT/npm/bin"

# 直接使用原生编译，不进行交叉编译
cargo build --release

# 创建发布包
if [[ $PLATFORM == *"windows"* ]]; then
  BINARY="$PROJECT_ROOT/target/release/komitto.exe"
  PACKAGE="$PROJECT_ROOT/releases/komitto-v$VERSION-$TARGET.tar.gz"
  
  mkdir -p "$PROJECT_ROOT/target/package-$TARGET"
  cp "$BINARY" "$PROJECT_ROOT/target/package-$TARGET/komitto.exe"
  
  # 创建 tar 包 - 使用绝对路径和 -C 选项
  tar -C "$PROJECT_ROOT/target/package-$TARGET" -czf "$PACKAGE" "komitto.exe"
else
  BINARY="$PROJECT_ROOT/target/release/komitto"
  PACKAGE="$PROJECT_ROOT/releases/komitto-v$VERSION-$TARGET.tar.gz"
  
  mkdir -p "$PROJECT_ROOT/target/package-$TARGET"
  cp "$BINARY" "$PROJECT_ROOT/target/package-$TARGET/komitto"
  
  # 创建 tar 包 - 使用绝对路径和 -C 选项
  tar -C "$PROJECT_ROOT/target/package-$TARGET" -czf "$PACKAGE" "komitto"
fi

echo "Created $PACKAGE"
echo "Build completed!"