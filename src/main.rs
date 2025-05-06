use std::process::Command;
use std::env;
use reqwest::blocking::Client;
use serde_json::json;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = "openai/gpt-4.1")]
    model: String,
}

fn main() {
    let args = Cli::parse();

    let diff = Command::new("git")
        .args(["diff", "--staged"])
        .output()
        .expect("git diff 失败");
    let diff_text = String::from_utf8_lossy(&diff.stdout);

    if diff_text.trim().is_empty() {
        eprintln!("没有暂存的修改（未执行 git add）。");
        return;
    }

    let prompt = format!(
        "Based on the following git diff, generate a concise, professional English git commit message following the Conventional Commits style (such as feat, fix, chore, etc). Output only the commit message, no explanations.\n\n{}\n", 
        diff_text
    );

    let api_key = env::var("OPENROUTER_API_KEY")
        .expect("请在环境变量设置 OPENROUTER_API_KEY");

    let client = Client::new();
    let body = json!({
        "model": args.model,
        "messages": [
            {"role": "user", "content": prompt}
        ]
    });

    let res = client.post("https://openrouter.ai/api/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .expect("请求 openrouter 失败");

    let resp_json: serde_json::Value = res.json().expect("解析 json 失败");

    if let Some(message) = resp_json["choices"][0]["message"]["content"].as_str() {
        println!("\n建议 commit message：\n{}", message.trim());
        println!("\n你可以使用：\n  git commit -m '{}'", message.trim().replace('\'', "\\'"));
    } else {
        eprintln!("AI 没有返回有效建议。\n完整响应: {resp_json:?}");
    }
}
