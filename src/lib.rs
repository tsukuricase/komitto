use reqwest::blocking::Client;
use serde_json::json;

/// 构造 OpenRouter Prompt
pub fn build_prompt(diff: &str) -> String {
    format!(
        "Based on the following git diff, generate a concise, professional English git commit message following the Conventional Commits style (such as feat, fix, chore, etc). Output only the commit message, no explanations.\n\n{}\n", 
        diff
    )
}

/// 发送 API 并获取 message
pub fn get_ai_commit_message(api_key: &str, model: &str, prompt: &str) -> Result<String, String> {
    let client = Client::new();
    let body = json!({
        "model": model,
        "messages": [
            {"role": "user", "content": prompt}
        ]
    });

    let res = client.post("https://openrouter.ai/api/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .map_err(|e| format!("请求 openrouter 失败: {e:?}"))?;

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
