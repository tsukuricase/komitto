#!/bin/bash
# 更新版本号的脚本
NEW_VERSION="0.1.6"

# 更新 Cargo.toml
sed -i '' "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml

# 更新 npm/package.json
sed -i '' "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" npm/package.json

# 更新 npm/install.js
sed -i '' "s/const VERSION = '.*';/const VERSION = '$NEW_VERSION';/" npm/install.js

echo "Version updated to $NEW_VERSION"