#!/bin/bash
set -e

# 确保已经构建
if [ ! -d "./releases" ]; then
  echo "No releases found. Run build-npm.sh first."
  exit 1
fi

# 确保 npm 包目录设置正确
if [ ! -f "./npm/package.json" ]; then
  echo "npm/package.json not found. Check your setup."
  exit 1
fi

# 设置权限
chmod +x npm/bin/komitto.js
chmod +x npm/install.js

# 进入 npm 目录
cd npm

# 发布包
echo "Publishing to npm..."
npm publish

echo "Successfully published to npm!" 