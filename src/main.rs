use komitto::{build_prompt, get_ai_commit_message};
use clap::Parser;
use std::process::Command;
use std::{env, time::Duration};
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = "openai/gpt-4.1")]
    model: String,
    #[arg(long)]
    staged: bool,
}

fn main() {
    let args = Cli::parse();
    let (diff_args, info_msg) = if args.staged {
        (vec!["diff", "--staged"], "没有暂存的修改（未执行 git add）。")
    } else {
        (vec!["diff"], "没有工作区的修改。")
    };

    let diff = Command::new("git")
        .args(&diff_args)
        .output()
        .expect("git diff 失败");
    let diff_text = String::from_utf8_lossy(&diff.stdout);

    if diff_text.trim().is_empty() {
        eprintln!("{}", info_msg);
        return;
    }

    let prompt = build_prompt(&diff_text);

    let api_key = env::var("OPENROUTER_API_KEY")
        .expect("请在环境变量设置 OPENROUTER_API_KEY");

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.cyan} {msg}")
        .unwrap());
    spinner.set_message("Waiting for OpenRouter AI response...");
    spinner.enable_steady_tick(Duration::from_millis(120));

    match get_ai_commit_message(&api_key, &args.model, &prompt) {
        Ok(msg) => {
            spinner.finish_and_clear();
            println!("\n建议 commit message：\n{}", msg);
            println!("\n你可以使用：\n  git commit -m '{}'", msg.replace('\'', "\\'"));
        }
        Err(err) => {
            spinner.finish_and_clear();
            eprintln!("错误: {}", err);
        }
    }
}
