use crate::{cli::LLMProviderType, error::GitAIError};
use anthropic::AnthropicProvider;
use async_trait::async_trait;
use openai::OpenAIProvider;
use phind::PhindProvider;
use grok::GroqProvider;
use thiserror::Error;

pub mod anthropic;
pub mod openai;
pub mod phind;
pub mod grok;

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
pub struct AIPromptError(pub String);

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
pub trait LLMComplete: Sync + Send + Clone {
    async fn complete(&self, messages: &[Message]) -> Result<String, LLMError>;
}

#[derive(Clone)]
pub enum LLMProvider {
    Openai(OpenAIProvider),
    Anthropic(AnthropicProvider),
    Phind(PhindProvider),
    Grok(GroqProvider),
}

#[async_trait]
impl LLMComplete for LLMProvider {
    async fn complete(&self, messages: &[Message]) -> Result<String, LLMError> {
        match self {
            LLMProvider::Openai(provider) => provider.complete(messages).await,
            LLMProvider::Anthropic(provider) => provider.complete(messages).await,
            LLMProvider::Phind(provider) => provider.complete(messages).await,
            LLMProvider::Grok(provider) => provider.complete(messages).await,
        }
    }
}

pub fn get_llm(
    provider: LLMProviderType,
    model: Option<String>,
    api_key: Option<String>,
) -> Result<LLMProvider, GitAIError> {
    match provider {
        LLMProviderType::Openai => {
            let api_key = api_key.ok_or(GitAIError::MissingApiKey("OpenAI".to_string()))?;
            let config = openai::OpenAIConfig::new(api_key, model);
            let client = reqwest::Client::new();
            Ok(LLMProvider::Openai(OpenAIProvider::new(client, config)))
        }
        LLMProviderType::Anthropic => {
            let api_key = api_key.ok_or(GitAIError::MissingApiKey("Anthropic".to_string()))?;
            let config = anthropic::AnthropicConfig::new(api_key, model);
            let client = reqwest::Client::new();
            Ok(LLMProvider::Anthropic(AnthropicProvider::new(
                client, config,
            )))
        }
        LLMProviderType::Phind => {
            let config = phind::PhindConfig::new(model);
            let client = reqwest::Client::new();
            Ok(LLMProvider::Phind(PhindProvider::new(client, config)))
        }
        LLMProviderType::Grok => {
            let api_key = api_key.ok_or(GitAIError::MissingApiKey
            ("Grok".to_string()))?;
            let config = grok::GroqConfig::new(api_key, model);
            let client = reqwest::Client::new();
            Ok(LLMProvider::Grok(GroqProvider::new(client, config)))
        }
    }
}
