#!/usr/bin/env node

const getBinary = require('./install');

// 导出 getBinary 函数便于直接调用
module.exports = {
  getBinary
}; 