use clap::Parser;
use std::process;

use cli::{Cli, Commands};
use config::GitAIConfig;
use error::GitAIError;
use llm::{get_llm, Message, Role};
use command::{Command, CommandType, GitAICommand};

mod cli;
mod config;
mod error;
mod git;
mod llm;
mod prompt_template;
mod command;
mod git_entity;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("\x1b[91m\rerror:\x1b[0m {e}");
        process::exit(1);
    }
}

async fn run() -> Result<(), GitAIError> {
    let cli = Cli::parse();
    let git = git::GitHelper::new().unwrap();

    let config = match GitAIConfig::build(&cli) {
        Result::Ok(config) => config,
        Err(e) => return Err(e),
    };

    println!("api-key: {:?}", config.api_key);

    let llm = get_llm(config.provider, config.model, config.api_key).unwrap();
    let command = GitAICommand::new(llm);

    match cli.command {
        Commands::Generate => {
            // command.
            command.execute(CommandType::Generate).await?;
            Ok(())
        }
        Commands::Explain {
            staged,
            diff,
            reference,
        } => {
            println!("Explain command {} {} {:?}", staged, diff, reference);
            println!("Staged: {}", staged);

            if (diff) {
                let diff = git.get_diff(staged).unwrap();
                println!("Diff: {}", diff);
            }

            Ok(())
        }
    }
}
