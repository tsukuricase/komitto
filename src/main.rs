use std::process::Command;
use std::env;
use reqwest::blocking::Client;
use serde_json::json;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = "openai/gpt-4.1")]
    model: String,
    #[arg(long)]
    staged: bool,
}

fn main() {
    let args = Cli::parse();

    // 根据参数决定 git diff 类型
    let (diff_args, info_msg) = if args.staged {
        (vec!["diff", "--staged"], "没有暂存的修改（未执行 git add）。")
    } else {
        (vec!["diff"], "没有工作区的修改。")
    };

    // 获取 diff
    let diff = Command::new("git")
        .args(&diff_args)
        .output()
        .expect("git diff 失败");
    let diff_text = String::from_utf8_lossy(&diff.stdout);

    if diff_text.trim().is_empty() {
        eprintln!("{}", info_msg);
        return;
    }

    let prompt = format!(
        "Based on the following git diff, generate a concise, professional English git commit message following the Conventional Commits style (such as feat, fix, chore, etc). Output only the commit message, no explanations.\n\n{}\n", 
        diff_text
    );

    let api_key = env::var("OPENROUTER_API_KEY")
        .expect("请在环境变量设置 OPENROUTER_API_KEY");

    // Spinner
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.cyan} {msg}")
        .unwrap());
    spinner.set_message("Waiting for OpenRouter AI response...");
    spinner.enable_steady_tick(Duration::from_millis(120));

    let client = Client::new();
    let body = json!({
        "model": args.model,
        "messages": [
            {"role": "user", "content": prompt}
        ]
    });

    // AI 请求
    let res = client.post("https://openrouter.ai/api/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .expect("请求 openrouter 失败");

    spinner.finish_and_clear();

    let resp_json: serde_json::Value = res.json().expect("解析 json 失败");

    if let Some(message) = resp_json["choices"][0]["message"]["content"].as_str() {
        println!("\n建议 commit message：\n{}", message.trim());
        println!("\n你可以使用：\n  git commit -m '{}'", message.trim().replace('\'', "\\'"));
    } else {
        eprintln!("AI 没有返回有效建议。\n完整响应: {resp_json:?}");
    }
}
