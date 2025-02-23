use crate::{cli::LLMProviderType, error::GitAIError};
use anthropic::AnthropicProvider;
use async_trait::async_trait;
use openai::OpenAIProvider;
use thiserror::Error;

pub mod anthropic;
pub mod openai;

#[derive(Debug, PartialEq)]
pub enum Role {
    System,
    User,
}

pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Error, Debug)]
#[error("{0}")]
pub struct AIPromptError(String);

#[derive(Error, Debug)]
pub enum LLMError {
    #[error("API request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("No completion choice available")]
    NoCompletionChoice,

    #[error(transparent)]
    AIPromptError(#[from] AIPromptError),

    #[error("API request failed with status code {0}: {1}")]
    APIError(reqwest::StatusCode, String),

    #[error("Unexpected response")]
    UnexpectedResponse,

    #[error("Some error occurred")]
    SomeError,
}

impl From<GitAIError> for LLMError {
    fn from(error: GitAIError) -> Self {
        LLMError::AIPromptError(AIPromptError(error.to_string()))
    }
}

#[async_trait]
pub trait LLMComplete {
    async fn complete(&self, messages: &[Message]) -> Result<String, LLMError>;
}

pub fn get_llm(
    provider: LLMProviderType,
    model: Option<String>,
    api_key: Option<String>,
) -> Result<Box<dyn LLMComplete>, GitAIError> {
    match provider {
        LLMProviderType::Openai => {
            let config = openai::OpenAIConfig::new(api_key.unwrap(), model);
            let client = reqwest::Client::new();
            Ok(Box::new(OpenAIProvider::new(client, config)))
        }
        LLMProviderType::Anthropic => {
            let config = anthropic::AnthropicConfig::new(api_key.unwrap(), model);
            let client = reqwest::Client::new();
            Ok(Box::new(AnthropicProvider::new(client, config)))
        }
    }
}
