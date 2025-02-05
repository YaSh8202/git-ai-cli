use std::str::FromStr;

use clap::{command, Parser, Subcommand, ValueEnum};


#[derive(Parser)]
#[command(name = "git-ai")]
#[command(about = "AI Powered Cli tool for git commits")]
pub struct Cli {

    #[arg(value_enum, short='p', long="provider")]
    pub provider: Option<LLMProvider>,

    #[arg(short, long)]
    pub model: Option<String>,

    #[arg(short, long)]
    pub api_key: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum LLMProvider {
    Openai,
    Phind,
    Claude
}

impl FromStr for LLMProvider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "openai" => Ok(LLMProvider::Openai),
            "phind" => Ok(LLMProvider::Phind),
            "claude" => Ok(LLMProvider::Claude),
            _ => Err("Invalid provider".to_string()),
        }
    }
}

#[derive(Subcommand)]
pub enum Commands {
    Generate,
    Explain {
        #[arg(long)]
        staged: bool,
    },
}