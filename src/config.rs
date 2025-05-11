use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::io::ErrorKind;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub model: String,
    pub api_key: Option<String>,
    pub api_url: String,
    pub provider: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            model: "openai/gpt-4.1".to_string(),
            api_key: None,
            api_url: "https://openrouter.ai/api/v1/chat/completions".to_string(),
            provider: Some("openrouter".to_string()),
        }
    }
}

pub fn get_config_path() -> PathBuf {
    let home = dirs::home_dir().expect("无法获取用户主目录");
    home.join(".komittorc").join("config.json")
}

pub fn load_config() -> Config {
    let config_path = get_config_path();
    
    if !config_path.exists() {
        return Config::default();
    }
    
    match fs::read_to_string(&config_path) {
        Ok(content) => match serde_json::from_str::<Config>(&content) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("配置文件格式错误: {}", e);
                Config::default()
            }
        },
        Err(e) if e.kind() == ErrorKind::NotFound => Config::default(),
        Err(e) => {
            eprintln!("读取配置文件失败: {}", e);
            Config::default()
        }
    }
}

pub fn save_config(config: &Config) -> Result<(), String> {
    let config_path = get_config_path();
    
    // 确保目录存在
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("创建配置目录失败: {}", e))?;
    }
    
    let content = serde_json::to_string_pretty(config).map_err(|e| format!("序列化配置失败: {}", e))?;
    fs::write(&config_path, content).map_err(|e| format!("写入配置文件失败: {}", e))?;
    
    Ok(())
} 