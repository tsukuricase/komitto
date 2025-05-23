# komitto

> 🚀 **AI-powered Conventional Commit Message Generator for Git.**  
> Automatically generate professional, well-structured Git commit messages using large language models (LLMs) via OpenRouter.

---

## ✨ Features

- **One-command automation:** Summarizes your `git diff --staged` and generates a commit message with a single command.
- **Conventional Commits:** Output messages conform to [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) style (e.g., feat, fix, chore).
- **Supports OpenRouter (OpenAI-compatible):** Only OpenRouter API Key is supported currently.
- **Multilingual:** Outputs commit messages in both English and Chinese (auto-detect by LLM).
- **Model Flexibility:** Choose any OpenRouter-supported model (e.g., `gpt-4`, `gpt-3.5`, Llama, etc).
- **Cross-platform:** Lightning-fast Rust CLI utility (macOS, Linux, Windows supported).
- **Security:** API keys are handled via environment variables.

---

## �� Installation

### 通过 npm 安装 (推荐)

如果您已经安装了 Node.js，可以通过 npm 轻松安装:

```bash
npm install -g komitto
```

### 通过源码安装

1. Prerequisites

- [Rust toolchain](https://www.rust-lang.org/tools/install) installed
- [OpenRouter](https://openrouter.ai/) account and API Key

### 2. Build from source

```bash
git clone https://github.com/tsukuricase/komitto.git
cd komitto
cargo build --release
```

After build, the binary is at: `target/release/komitto`

### 3. Set up your OpenRouter API Key

Add your API Key to your environment, e.g.:

**Linux/Mac:**

```bash
export OPENROUTER_API_KEY=your-openrouter-api-key
```

*(Recommended: add to your `~/.bashrc` or `~/.zshrc` for convenience)*

**Windows CMD:**

```cmd
set OPENROUTER_API_KEY=your-openrouter-api-key
```

---

## 🚦 Usage

### 1. Stage your changes

```bash
git add .
```

### 2. Generate commit message

```bash
target/release/komitto
```

> The tool will automatically extract your staged changes, send to OpenRouter, and propose a commit message.

### 3. Use the suggested commit message

```bash
git commit -m "your AI-generated commit message"
```

---

### ⚙️ CLI options

| Argument          | Description                                                  | Example                         |
|-------------------|--------------------------------------------------------------|--------------------------------|
| `--model`         | Specify OpenRouter model (default: `openai/gpt-4.1`)         | `--model gpt-3.5`              |
| `--staged`        | Use staged changes only (`git diff --staged`, default: false)| `--staged`                     |
| `--api_key`       | Specify API key (overrides env and config)                   | `--api_key your-api-key`       |
| `--api_url`       | Specify custom API URL                                       | `--api_url https://custom.api` |
| `--save_config`   | Save current parameters to config file                       | `--save_config`                |
| `--show_config`   | Display current configuration                                | `--show_config`                |
| `--help`          | Show help message                                            | `--help`                       |

Try:

```bash
komitto --help
```

### 📝 Configuration File

你可以通过交互式配置向导快速设置：

```bash
# 运行配置向导
komitto init
```

或者手动使用 JSON 配置文件，位于 `~/.komittorc/config.json`：

```json
{
  "model": "openai/gpt-4.1",
  "api_key": "your-api-key",
  "api_url": "https://openrouter.ai/api/v1/chat/completions"
}
```

要手动创建或更新配置文件：

```bash
# 设置并保存配置
komitto --model anthropic/claude-3-opus-20240229 --api_url https://your-api-endpoint.com --save_config

# 查看当前配置
komitto --show_config
```

> 优先级顺序：命令行参数 > 配置文件 > 环境变量

---

## 🌐 Internationalization

- The prompt is optimized for English Conventional Commits, but models may return content in English or Chinese based on your code/comments/context.
- Contributors interested in further i18n support may open an Issue or PR!

---

## 🔐 OpenRouter Only

> ⚠️ **Notice:**  
> Currently, komitto **only supports [OpenRouter](https://openrouter.ai/) API Key** (`OPENROUTER_API_KEY` environment variable).  
> OpenAI direct keys are not yet supported.

For OpenRouter API docs and supported models, see: [OpenRouter Developers](https://openrouter.ai/docs)

---

## 🧪 Testing

- Run tests locally before publishing:
  ```bash
  cargo test
  ```
- Our GitHub Actions CI will also automatically build & test for every push before publishing to crates.io.

---

## 🤝 Contributing

1. Fork this repo, create your feature branch (`git checkout -b feat/my-feature`)
2. Write code & tests (`cargo test`)
3. Make sure all checks pass and open a Pull Request

Questions? Suggestions? [Open an issue!](https://github.com/tsukuricase/komitto/issues)

---

## 📄 License

MIT License © 2024 tsukuricase & contributors

---

## 🙏 Acknowledgements

- Powered by [OpenRouter](https://openrouter.ai/)
- Built with [Rust](https://www.rust-lang.org/)
- Inspired by [Conventional Commits](https://www.conventionalcommits.org/)