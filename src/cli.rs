use std::str::FromStr;
use crate::commit_reference::CommitReference;
use clap::{command, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "gitai")]
#[command(about = "AI Powered Cli tool for git commits")]
pub struct Cli {
    #[arg(value_enum, short = 'p', long = "provider")]
    pub provider: Option<LLMProviderType>,

    #[arg(short, long)]
    pub model: Option<String>,

    #[arg(short, long)]
    pub api_key: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum LLMProviderType {
    Openai,
    Phind,
    Anthropic,
    Grok
}

impl FromStr for LLMProviderType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "openai" => Ok(LLMProviderType::Openai),
            // "phind" => Ok(LLMProviderType::Phind),
            "anthropic" => Ok(LLMProviderType::Anthropic),
            _ => Err("Invalid provider".to_string()),
        }
    }
}

#[derive(Subcommand)]
pub enum Commands {
    Generate,
    Explain {
        #[arg(group = "target", value_parser = clap::value_parser!(CommitReference))]
        reference: Option<CommitReference>,

        #[arg(long, group = "target")]
        diff: bool,

        /// use staged changes
        #[arg(long)]
        staged: bool,
    },
}
