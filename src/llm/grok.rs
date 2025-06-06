use super::{LLMComplete, LLMError, Message, Role};
use async_trait::async_trait;
use reqwest::StatusCode;
use serde_json::{json, Value};

#[derive(Clone)]
pub struct GroqConfig {
    api_key: String,
    model: String,
    api_base_url: String,
}

impl GroqConfig {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        Self {
            api_key,
            model: model.unwrap_or_else(|| "mixtral-8x7b-32768".to_string()),
            api_base_url: "https://api.groq.com/openai/v1/chat/completions".to_string(),
        }
    }
}
#[derive(Clone)]
pub struct GroqProvider {
    client: reqwest::Client,
    config: GroqConfig,
}

impl GroqProvider {
    pub fn new(client: reqwest::Client, config: GroqConfig) -> Self {
        Self { client, config }
    }

    async fn complete(&self, messages: &[Message]) -> Result<String, LLMError> {
        let payload = json!({
            "model": self.config.model,
            "messages": messages.iter().map(|message| {
                json!({
                    "role": match message.role {
                        Role::System => "system",
                        Role::User => "user",
                    },
                    "content": message.content,
                })
            }).collect::<Vec<Value>>(),
        });

        let response = self
            .client
            .post(&self.config.api_base_url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&payload)
            .send()
            .await?;

        let status = response.status();
        match status {
            StatusCode::OK => {
                let response_json: Value = response.json().await?;
                let content = response_json
                    .get("choices")
                    .and_then(|choices| choices.get(0))
                    .and_then(|choice| choice.get("message"))
                    .and_then(|message| message.get("content"))
                    .and_then(|content| content.as_str())
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
impl LLMComplete for GroqProvider {
    async fn complete(&self, messages: &[Message]) -> Result<String, LLMError> {
        self.complete(&messages).await
    }
}
