use async_trait::async_trait;
use reqwest::StatusCode;
use serde_json::{json, Value};

use super::{LLMComplete, LLMError, Message, Role};

pub struct AnthropicConfig {
    api_key: String,
    model: String,
    api_base_url: String,
}

impl AnthropicConfig {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        AnthropicConfig {
            api_key,
            model: model.unwrap_or("gpt-4o-mini".to_string()),
            api_base_url: "https://api.openai.com/v1/chat/completions".to_string(),
        }
    }
}

pub struct AnthropicProvider {
    config: AnthropicConfig,
    client: reqwest::Client,
}

impl AnthropicProvider {
    pub fn new(client: reqwest::Client, config: AnthropicConfig) -> Self {
        AnthropicProvider { client, config }
    }

    pub async fn complete(&self, messages: &[Message]) -> Result<String, LLMError> {
        let system_prompt = messages
            .iter()
            .find(|message| message.role == Role::System)
            .map(|message| message.content.clone())
            .unwrap_or("".to_string());

        let user_mesage = messages.get(1).ok_or(LLMError::SomeError)?;

        let payload = json!({
            "model": self.config.model,
            "max_tokens": 4096,
            "system": system_prompt,
            "messages": [
                {
                    "role": "user",
                    "content": user_mesage.content
                }
            ]
        });

        let response = self
            .client
            .post(&self.config.api_base_url)
            .header("x-api-key", &self.config.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        let status = response.status();

        match status {
            StatusCode::OK => {
                let response_json: Value = response.json().await?;

                let content = response_json
                    .get("content")
                    .and_then(|content| content.get(0))
                    .and_then(|choice| choice.get("message"))
                    .and_then(|message| message.get("text"))
                    .and_then(|text| text.as_str())
                    .ok_or(LLMError::NoCompletionChoice)?;

                Ok(content.to_string())
            }
            _ => {
                let error_json: Value = response.json().await?;

                let error_message = error_json
                    .get("error")
                    .and_then(|error| error.get("message"))
                    .and_then(|msg| msg.as_str())
                    .ok_or(LLMError::UnexpectedResponse)?
                    .into();

                Err(LLMError::APIError(status, error_message))
            }
        }
    }
}

#[async_trait]
impl LLMComplete for AnthropicProvider {
    async fn complete(&self, messages: &[Message]) -> Result<String, LLMError> {
        self.complete(&messages).await
    }
}
