use komitto::{build_prompt, get_ai_commit_message, Config, load_config, save_config, get_config_path};
use clap::{Parser, Subcommand};
use std::process::Command;
use std::{env, time::Duration, io};
use std::io::Write;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// 指定使用的 AI 模型
    #[arg(long)]
    model: Option<String>,
    
    /// 使用暂存区的变更而不是工作区
    #[arg(long)]
    staged: bool,
    
    /// 指定 API 密钥（不指定则使用环境变量或配置文件）
    #[arg(long)]
    api_key: Option<String>,
    
    /// 指定 API 服务地址
    #[arg(long)]
    api_url: Option<String>,
    
    /// 保存当前参数到配置文件
    #[arg(long)]
    save_config: bool,
    
    /// 显示当前配置
    #[arg(long)]
    show_config: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// 初始化配置向导
    Init,
}

fn main() {
    let args = Cli::parse();
    
    if let Some(command) = args.command {
        match command {
            Commands::Init => {
                init_command();
                return;
            }
        }
    }
    
    let mut config = load_config();
    
    // 显示当前配置并退出
    if args.show_config {
        println!("当前配置:");
        println!("提供商: {}", config.provider.unwrap_or_else(|| "未指定".to_string()));
        println!("模型: {}", config.model);
        println!("API URL: {}", config.api_url);
        println!("配置文件路径: {:?}", get_config_path());
        return;
    }
    
    // 命令行参数覆盖配置文件
    if let Some(model) = args.model {
        config.model = model;
    }
    
    if let Some(api_url) = args.api_url {
        config.api_url = api_url;
    }
    
    // 保存配置
    if args.save_config {
        if let Err(e) = save_config(&config) {
            eprintln!("保存配置失败: {}", e);
        } else {
            println!("配置已保存到 {:?}", get_config_path());
        }
        return;
    }
    
    // 获取差异
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

    // 优先使用命令行参数的 API 密钥，其次使用配置文件，最后使用环境变量
    let api_key = args.api_key
        .or(config.api_key)
        .unwrap_or_else(|| env::var("OPENROUTER_API_KEY")
            .expect("请在环境变量设置 OPENROUTER_API_KEY 或使用 --api_key 参数指定"));

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.cyan} {msg}")
        .unwrap());
    let provider_name = config.provider.as_deref().unwrap_or("AI");
    spinner.set_message(format!("正在使用 {} ({}) 生成提交信息...", config.model, provider_name));
    spinner.enable_steady_tick(Duration::from_millis(120));

    match get_ai_commit_message(&api_key, &config.model, &prompt, &config.api_url) {
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

fn init_command() {
    println!("欢迎使用 komitto 配置向导！");
    println!("我们将引导您设置 AI 提供商和 API 密钥。");
    println!("目前支持的大模型厂商: openrouter, deepseek");
    
    let mut provider = String::new();
    let mut api_key = String::new();
    let mut model = String::new();
    
    // 选择提供商
    loop {
        println!("\n请选择大模型厂商:");
        println!("1. OpenRouter (https://openrouter.ai)");
        println!("2. DeepSeek (https://deepseek.com)");
        print!("请输入选项 [1/2]: ");
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("读取输入失败");
        
        match choice.trim() {
            "1" => {
                provider = "openrouter".to_string();
                break;
            }
            "2" => {
                provider = "deepseek".to_string();
                break;
            }
            _ => println!("无效选项，请重新输入"),
        }
    }
    
    // 获取 API 密钥
    println!("\n请输入您的 {} API 密钥:", provider);
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut api_key).expect("读取输入失败");
    api_key = api_key.trim().to_string();
    
    // 获取默认模型
    if provider == "openrouter" {
        println!("\n请输入您想使用的默认模型 (默认: openai/gpt-4.1):");
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut model).expect("读取输入失败");
        model = model.trim().to_string();
        if model.is_empty() {
            model = "openai/gpt-4.1".to_string();
        }
    } else {
        println!("\n请输入您想使用的默认模型 (默认: deepseek-chat):");
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut model).expect("读取输入失败");
        model = model.trim().to_string();
        if model.is_empty() {
            model = "deepseek-chat".to_string();
        }
    }
    
    // 创建配置
    let api_url = match provider.as_str() {
        "openrouter" => "https://openrouter.ai/api/v1/chat/completions".to_string(),
        "deepseek" => "https://api.deepseek.com/chat/completions".to_string(),
        _ => panic!("不支持的提供商"),
    };
    
    let config = Config {
        model,
        api_key: Some(api_key),
        api_url,
        provider: Some(provider),
    };
    
    // 保存配置
    match save_config(&config) {
        Ok(_) => {
            println!("\n配置已成功保存到 {:?}", get_config_path());
            println!("现在您可以使用 'komitto' 命令生成提交信息了！");
        }
        Err(e) => {
            eprintln!("\n保存配置失败: {}", e);
            eprintln!("请检查权限或手动创建配置文件。");
        }
    }
}
