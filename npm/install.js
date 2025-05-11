#!/usr/bin/env node

const { Binary } = require('binary-install');
const os = require('os');
const { join } = require('path');

const PACKAGE_NAME = 'komitto';
const VERSION = '0.1.6';

function getPlatform() {
  const type = os.type();
  const arch = os.arch();

  if (type === 'Windows_NT') {
    return arch === 'x64' ? 'win64' : 'win32';
  }
  
  if (type === 'Linux') {
    return arch === 'x64' ? 'linux' : arch === 'arm64' ? 'linuxarm64' : null;
  }
  
  if (type === 'Darwin') {
    return arch === 'x64' ? 'macos' : arch === 'arm64' ? 'macosarm64' : null;
  }

  throw new Error(`Unsupported platform: ${type} ${arch}`);
}

function getBinary() {
  const platform = getPlatform();
  if (!platform) {
    throw new Error(`Unsupported platform: ${os.type()} ${os.arch()}`);
  }
  
  const url = `https://github.com/tsukuricase/komitto/releases/download/v${VERSION}/komitto-v${VERSION}-${platform}.tar.gz`;
  console.log(`Downloading from: ${url}`);
  
  return new Binary(PACKAGE_NAME, url);
}

function install() {
  const binary = getBinary();
  binary.install();
}

if (require.main === module) {
  install();
}

module.exports = getBinary; 