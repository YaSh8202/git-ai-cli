use clap::Parser;

use cli::{Cli, Commands};
use config::GitAIConfig;
use error::GitAIError;
use llm::{get_llm, Message, Role};

mod cli;
mod config;
mod error;
mod git;
mod llm;
mod prompt_template;

#[tokio::main]
async fn main() {
    // run().await;
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
    }
}

async fn run() -> Result<(), GitAIError> {
    let cli = Cli::parse();
    // let client = reqwest::Client::new();
    let git = git::GitHelper::new().unwrap();

    let config = match GitAIConfig::build(&cli) {
        Result::Ok(config) => config,
        Err(e) => return Err(e),
    };

    println!("api-key: {:?}", config.api_key);

    let llm = get_llm(config.provider, config.model, config.api_key).unwrap();

    // let llm

    match cli.command {
        Commands::Generate => {
            let diff = git.get_diff(true).unwrap();

            let system_message = Message {
                role: Role::System,
                content: String::from(format! {
                    "You are a commit message generator that follows these rules:
                    1. Write in present tense
                    2. Be concise and direct
                    3. Output only the commit message without any explanations
                    4. Follow the format: <type>(<optional scope>): <commit message>"
                }),
            };

            let user_message = Message {
                role: Role::User,
                content: diff,
            };

            let messages = vec![system_message, user_message];

            let response = llm.complete(&messages).await;

            match response {
                Ok(commit_message) => {
                    println!("Commit message: {}", commit_message);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }

            Ok(())
        }
        Commands::Explain { staged } => {
            println!("Explain command");
            println!("Staged: {}", staged);
            Ok(())
        }
    }
}
