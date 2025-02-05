use clap::Parser;

use cli::{Cli, Commands};

mod git;
mod llm;
mod cli;


#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let cli = Cli::parse();
    let client = reqwest::Client::new();
    let git = git::GitHelper::new().unwrap();

    match cli.command {
        Commands::Generate => {
            let diff = git.get_diff(true).unwrap();

            // println!("Diff: {}", diff);

            let config = llm::openai::OpenAIConfig::new(
                "sk-**".to_string(),
                None,
            );

            let llm = llm::openai::OpenAIProvider::new(client, config);

            let system_message = llm::Message {
                role: llm::Role::System,
                content: String::from(format! {
                    "You are a commit message generator that follows these rules:
                    1. Write in present tense
                    2. Be concise and direct
                    3. Output only the commit message without any explanations
                    4. Follow the format: <type>(<optional scope>): <commit message>"
                }),
            };

            let user_message = llm::Message {
                role: llm::Role::User,
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
        }
        Commands::Explain { staged } => {
            println!("Explain command");
            println!("Staged: {}", staged);
        }
    }
}
