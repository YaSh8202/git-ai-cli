use crate::cli::LLMProviderType;
use crate::error::GitAIError;
use crate::Cli;
use serde::{Deserialize, Deserializer};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct GitAIConfig {
    #[serde(
        default = "default_provider",
        deserialize_with = "deserialize_llm_provider"
    )]
    pub provider: LLMProviderType,

    pub model: Option<String>,

    pub api_key: Option<String>,
}

fn default_provider() -> LLMProviderType {
    LLMProviderType::Phind
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
        let default = GitAIConfig::from_file()?;

        let provider = cli.provider.as_ref().cloned().unwrap_or(default.provider);
        let model = cli.model.clone().or(default.model);
        let api_key = cli.api_key.clone().or(default.api_key);

        Ok(GitAIConfig {
            provider,
            model,
            api_key,
        })
    }

    fn get_config_path() -> Result<PathBuf, GitAIError> {
        let home_dir = dirs::home_dir().ok_or_else(|| {
            GitAIError::ConfigError("Could not determine home directory".to_string())
        })?;
        Ok(home_dir.join(".gitai").join(".env"))
    }

    fn from_file() -> Result<Self, GitAIError> {
        let config_path = Self::get_config_path()?;
        
        // If config file doesn't exist, return default config
        if !config_path.exists() {
            return Ok(GitAIConfig::default());
        }

        let file = File::open(config_path).map_err(|e| {
            GitAIError::ConfigError(format!("Could not open config file: {}", e))
        })?;

        let mut provider = default_provider();
        let mut model = None;
        let mut api_key = None;

        // Parse the .env file
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            let line = line.map_err(|e| {
                GitAIError::ConfigError(format!("Error reading config file: {}", e))
            })?;

            if let Some((key, value)) = line.split_once('=') {
                match key {
                    "PROVIDER" => {
                        provider = value.parse().unwrap_or(default_provider());
                    },
                    "MODEL" => {
                        if !value.is_empty() {
                            model = Some(value.to_string());
                        }
                    },
                    "OPENAI_API_KEY" if provider == LLMProviderType::Openai => {
                        if !value.is_empty() {
                            api_key = Some(value.to_string());
                        }
                    },
                    "PHIND_API_KEY" if provider == LLMProviderType::Phind => {
                        if !value.is_empty() {
                            api_key = Some(value.to_string());
                        }
                    },
                    "ANTHROPIC_API_KEY" if provider == LLMProviderType::Anthropic => {
                        if !value.is_empty() {
                            api_key = Some(value.to_string());
                        }
                    },
                    "GROQ_API_KEY" if provider == LLMProviderType::Grok => {
                        if !value.is_empty() {
                            api_key = Some(value.to_string());
                        }
                    },
                    _ => {}
                }
            }
        }

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
            model: None,
            api_key: None,
        }
    }
}
