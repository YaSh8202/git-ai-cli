use crate::cli::LLMProviderType;
use crate::error::GitAIError;
use crate::llm::LLMProvider;
use async_trait::async_trait;
use dialoguer::{console::Term, theme::ColorfulTheme, Input, Select};
use std::env;
use std::fs;

use super::Command;

pub struct ConfigureCommand {}

impl ConfigureCommand {
    fn get_default_model_for_provider(provider: &LLMProviderType) -> &'static str {
        match provider {
            LLMProviderType::Openai => "gpt-4o-mini",
            LLMProviderType::Phind => "Phind-70B",
            LLMProviderType::Anthropic => "claude-3-opus-20240229",
            LLMProviderType::Grok => "mixtral-8x7b-32768",
        }
    }

    fn get_environment_variable_name_for_provider(provider: &LLMProviderType) -> &'static str {
        match provider {
            LLMProviderType::Openai => "OPENAI_API_KEY",
            LLMProviderType::Phind => "PHIND_API_KEY",
            LLMProviderType::Anthropic => "ANTHROPIC_API_KEY",
            LLMProviderType::Grok => "GROQ_API_KEY",
        }
    }

    fn save_config(&self, provider: LLMProviderType, model: &str, api_key: &str) -> Result<(), GitAIError> {
        // Create ~/.gitai directory if it doesn't exist
        let home_dir = dirs::home_dir().ok_or_else(|| {
            GitAIError::ConfigError("Could not determine home directory".to_string())
        })?;
        let config_dir = home_dir.join(".gitai");
        fs::create_dir_all(&config_dir).map_err(|e| {
            GitAIError::ConfigError(format!("Could not create config directory: {}", e))
        })?;

        // Get provider as string
        let provider_str = match provider {
            LLMProviderType::Openai => "openai",
            LLMProviderType::Phind => "phind",
            LLMProviderType::Anthropic => "anthropic",
            LLMProviderType::Grok => "grok",
        };

        // Create or update .env file to persist configuration
        let env_file = config_dir.join(".env");
        
        // Get the API key environment variable name for the provider
        let api_key_env_var = Self::get_environment_variable_name_for_provider(&provider);
        
        // Create the content for the .env file
        let env_content = format!(
            "PROVIDER={}\nMODEL={}\n{}={}\n",
            provider_str, model, api_key_env_var, api_key
        );

        fs::write(&env_file, env_content).map_err(|e| {
            GitAIError::ConfigError(format!("Could not write config file: {}", e))
        })?;

        println!("Configuration saved successfully!");
        println!("Provider: {}", provider_str);
        println!("Model: {}", model);
        println!("Config directory: {}", config_dir.display());

        Ok(())
    }
}

#[async_trait]
impl Command for ConfigureCommand {
    async fn execute(&self, _llm: LLMProvider) -> Result<(), GitAIError> {
        let theme = ColorfulTheme::default();
        
        // Select provider
        let items = vec!["OpenAI", "Phind", "Anthropic", "Groq"];
        let selection = Select::with_theme(&theme)
            .with_prompt("Select your LLM provider")
            .items(&items)
            .default(0)
            .interact_on(&Term::stderr())?;

        let provider = match selection {
            0 => LLMProviderType::Openai,
            1 => LLMProviderType::Phind,
            2 => LLMProviderType::Anthropic,
            3 => LLMProviderType::Grok,
            _ => unreachable!(),
        };

        // Default model based on selected provider
        let default_model = Self::get_default_model_for_provider(&provider);
        
        // Input model name with default value based on provider
        let model: String = Input::with_theme(&theme)
            .with_prompt("Model name")
            .default(default_model.to_string())
            .interact_on(&Term::stderr())?;

        // Input API key
        let api_key_var = Self::get_environment_variable_name_for_provider(&provider);
        let default_api_key = env::var(api_key_var).unwrap_or_default();
        
        let api_key: String = Input::with_theme(&theme)
            .with_prompt("API Key")
            .default(default_api_key)
            .interact_on(&Term::stderr())?;

        // Save config
        self.save_config(provider, &model, &api_key)?;

        Ok(())
    }
}