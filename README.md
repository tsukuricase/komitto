# komitto

> 使用大模型（OpenRouter，兼容 OpenAI）自动生成专业、规范的 Git Commit Message，提高团队协作效率！

---

## ✨ 项目简介

**komitto** 是一个 Rust 编写的命令行工具，可自动获取 Git 暂存区变更，联动大语言模型（如 ChatGPT，通过 OpenRouter)，一键帮你生成符合 [Conventional Commits](https://www.conventionalcommits.org/zh-hans/v1.0.0/) 规范的 commit message，助力代码审查与版本管理更高效、专业。

---

## 🚀 主要特性

- ✨ 自动提取 `git diff --staged`，智能生成提交说明（支持中文/英文）
- 🤖 支持指定多种模型（如 gpt-4, gpt-3.5, Llama 等，只要 OpenRouter 支持）
- 🔒 API 密钥通过环境变量安全管理
- 🛠️ 一行命令轻松集成你的开发流程
- 📜 输出规范的 `git commit -m "..."`
- ⚡ 跨平台，编译产物小巧，命令行体验极佳

---

## 🛠️ 安装指南

### 1. 准备环境

- 已安装 [Rust 开发工具链](https://www.rust-lang.org/zh-CN/tools/install)
- 已注册 [OpenRouter](https://openrouter.ai/) 并获取 API Key

### 2. 克隆项目 & 编译

```bash
git clone https://github.com/你的用户名/komitto.git
cd komitto

# 构建 release 版（建议）
cargo build --release

# 若需测试直接运行
cargo run
```
编译后可执行文件路径为: `target/release/komitto`

### 3. 配置 OpenRouter API Key

将 API 密钥写入终端环境（建议加入 .zshrc 或 .bashrc 等文件）：

```bash
export OPENROUTER_API_KEY=你的apikey
```

---

## 🚦 使用方法

1. **小步提交，先添加变更：**
   ```bash
   git add .
   ```
2. **运行 komitto 自动生成提交消息：**
   - 调试时用 cargo 执行：
     ```bash
     cargo run
     ```
   - 或者直接用 release 可执行文件（推荐）：
     ```bash
     ./target/release/komitto
     ```
3. **查看推荐的 commit message，按需提交：**
   ```bash
   git commit -m "fix(api): 修复登录异常导致的会话丢失"
   ```

---

## ⚙️ 可选参数

| 参数           | 说明                                 | 示例                    |
|----------------|-------------------------------------|------------------------|
| `--model`      | 指定大模型（如 `gpt-4`, `gpt-3.5`） | `--model gpt-4`        |
| `--lang`       | 输出语言（如 `zh` 中文或 `en` 英文） | `--lang zh`            |
| `--yes`        | 自动用建议直接提交（谨慎使用）       | `--yes`                |
| `--help`       | 查看命令帮助信息                     | `--help`               |

> **提示** 具体参数以你的实现为准，支持 `komitto --help` 查看即时文档。

---

## ⛔ 常见问题

- **没有检测到暂存区 diff？**  
  请先 `git add` 需要提交的文件。
- **API Key Invalid？**  
  检查环境变量设置和 Key 是否过期。
- **响应过慢？**  
  网络质量或服务商限流问题，稍后重试。

---

## 📦 依赖&兼容性

- Rust 2021+
- 依赖 crates: `clap`、`reqwest`、`serde_json` 等
- 兼容任意支持 OpenRouter 的 LLM 模型

---

## 🤝 贡献指南

欢迎社区开发者一起共建！

1. fork 并新建分支
2. 开发并自测（建议加单测）
3. 提交 PR 说明修改点

---

## 📄 协议

MIT license ©️ 2024 

---

## 🙌 鸣谢

特别感谢 [OpenRouter](https://openrouter.ai/)、[Rust 社区](https://rust-lang.org/) 与所有贡献者！