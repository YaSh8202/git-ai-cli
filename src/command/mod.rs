use crate::{error::GitAIError, git_entity::diff::Diff, llm::LLMProvider};
use async_trait::async_trait;
use crate::git_entity::GitEntity;

mod generate;
mod explain;
mod configure;

pub struct GitAICommand {
    provider: LLMProvider,
}

pub struct AIPrompt {
    pub system_prompt: String,
    pub user_prompt: String,
}

impl GitAICommand {
    pub fn new(provider: LLMProvider) -> Self {
        Self { provider }
    }

    pub async fn execute(&self, command_type: CommandType) -> Result<(), GitAIError> {
        command_type
            .create_command()?
            .execute(self.provider.clone())
            .await
    }
}

pub enum CommandType {
    Generate,
    Explain { git_entity: GitEntity },
    Configure,
}

impl CommandType {
    pub fn create_command(self) -> Result<Box<dyn Command>, GitAIError> {
        match self {
            CommandType::Generate => Ok(Box::new(generate::GenerateCommand {
                git_entity: GitEntity::Diff(Diff::from_working_tree(true).unwrap()),
            })),
            CommandType::Explain { git_entity } => Ok(Box::new(explain::ExplainCommand { git_entity })),
            CommandType::Configure => Ok(Box::new(configure::ConfigureCommand {})),
        }
    }
}

#[async_trait]
pub trait Command {
    async fn execute(&self, llm: LLMProvider) -> Result<(), GitAIError>;
}
