use crate::cli::LLMProviderType;
use crate::error::GitAIError;
use crate::Cli;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct GitAIConfig {
    #[serde(
        default = "default_provider",
        deserialize_with = "deserialize_llm_provider"
    )]
    pub provider: LLMProviderType,

    #[serde(default = "default_model")]
    pub model: Option<String>,

    #[serde(default = "default_api_key")]
    pub api_key: Option<String>,
}

fn default_model() -> Option<String> {
    std::env::var("GITAI_MODEL").ok()
}

fn default_provider() -> LLMProviderType {
    std::env::var("GITAI_PROVIDER")
        .unwrap_or_else(|_| "phind".to_string())
        .parse()
        .unwrap_or(LLMProviderType::Openai)
}

fn default_api_key() -> Option<String> {
    std::env::var("GITAI_API_KEY").ok()
}

fn deserialize_llm_provider<'de, D>(deserializer: D) -> Result<LLMProviderType, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}

impl GitAIConfig {
    pub fn build(cli: &Cli) -> Result<Self, GitAIError> {
        let default = GitAIConfig::default();

        let provider = cli.provider.as_ref().cloned().unwrap_or(default.provider);
        let model = cli.model.clone().or(default.model);
        let api_key = cli.api_key.clone().or(default.api_key);

        Ok(GitAIConfig {
            provider,
            model,
            api_key,
        })
    }
}

impl Default for GitAIConfig {
    fn default() -> Self {
        GitAIConfig {
            provider: default_provider(),
            model: default_model(),
            api_key: default_api_key(),
        }
    }
}
