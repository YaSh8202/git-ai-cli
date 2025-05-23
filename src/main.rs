use clap::Parser;
use std::process;

use cli::{Cli, Commands};
use command::{CommandType, GitAICommand};
use commit_reference::CommitReference;
use config::GitAIConfig;
use error::GitAIError;
use git_entity::commit::Commit;
use git_entity::diff::Diff;
use git_entity::GitEntity;
use llm::get_llm;
mod cli;
mod command;
mod commit_reference;
mod config;
mod error;
mod git_entity;
mod llm;
mod util;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("\x1b[91m\rerror:\x1b[0m {e}");
        process::exit(1);
    }
}

async fn run() -> Result<(), GitAIError> {
    let cli = Cli::parse();

    let config = match GitAIConfig::build(&cli) {
        Result::Ok(config) => config,
        Err(e) => return Err(e),
    };

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
            let git_entity = if diff {
                GitEntity::Diff(Diff::from_working_tree(staged)?)
            } else if let Some(CommitReference::Single(input)) = reference {
                // let sha = if input == "-" {
                //     read_from_stdin()?
                // } else {
                // input
                // };
                GitEntity::Commit(Commit::new(input)?)
            } else if let Some(CommitReference::Range { from, to }) = reference {
                GitEntity::Diff(Diff::from_commits_range(&from, &to, false)?)
            } else if let Some(CommitReference::TripleDots { from, to }) = reference {
                GitEntity::Diff(Diff::from_commits_range(&from, &to, true)?)
            } else {
                return Err(GitAIError::InvalidArguments(
                    "`explain` expects SHA-1 or --diff to be present".into(),
                ));
            };

            command.execute(CommandType::Explain { git_entity }).await?;

            Ok(())
        }
        Commands::Configure => {
            command.execute(CommandType::Configure).await?;
            Ok(())
        }
    }
}
