use reqwest::blocking::Client;
use serde_json::json;

// 添加配置模块
pub mod config;

// 重新导出配置模块
pub use crate::config::{Config, load_config, save_config, get_config_path};

pub fn build_prompt(diff: &str) -> String {
    format!(
        "Given the following git diff, generate a single-line, professional commit message in English that follows the Conventional Commits style (e.g., feat, fix, chore).  
        - Focus on the **core functional/structural changes**.  
        - **Summarize exactly _what_ and _why_ was changed**, not _how_.  
        - Use precise nouns and verbs directly reflecting the affected files/functions/logic.  
        - DO NOT include explanations, descriptions, bullet points, or extra lines—respond ONLY with the commit message title, strictly on a single line.
        
        GIT DIFF:
        {diff}
        "
            )
}

/// 发送 API 并获取 message
pub fn get_ai_commit_message(api_key: &str, model: &str, prompt: &str, api_url: &str) -> Result<String, String> {
    let client = Client::new();
    let body = json!({
        "model": model,
        "messages": [
            {"role": "user", "content": prompt}
        ]
    });

    let res = client.post(api_url)
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .map_err(|e| format!("API 请求失败: {e:?}"))?;

    let resp_json: serde_json::Value = res.json().map_err(|e| format!("解析 json 失败: {e:?}"))?;

    if let Some(message) = resp_json["choices"][0]["message"]["content"].as_str() {
        Ok(message.trim().to_string())
    } else {
        Err(format!("AI 没有返回有效建议。\n完整响应: {resp_json:?}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_prompt_basic() {
        let diff = "diff --git a/foo.txt b/foo.txt";
        let prompt = build_prompt(diff);
        assert!(prompt.contains(diff));
        assert!(prompt.contains("generate a concise"));
    }
}
