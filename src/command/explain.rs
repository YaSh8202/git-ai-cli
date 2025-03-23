use crate::llm::LLMComplete;

use super::AIPrompt;
use super::Command;
use crate::git_entity::diff::Diff;
use crate::git_entity::GitEntity;
use crate::llm::{AIPromptError, LLMProvider, Message, Role};
use crate::util::print_markdown;
use indoc::{formatdoc, indoc};
use spinoff::{spinners, Color, Spinner};

use crate::error::GitAIError;
use async_trait::async_trait;

pub struct ExplainCommand {
    pub git_entity: GitEntity,
}

impl ExplainCommand {
    pub fn get_ai_prompt(&self) -> Result<AIPrompt, AIPromptError> {
        let system_prompt = String::from(indoc! {"
            You are a helpful assistant that explains Git changes in a concise way.
            Focus only on the most significant changes and their direct impact.
            When answering specific questions, address them directly and precisely.
            Keep explanations brief but informative and don't ask for further explanations.
            Use markdown for clarity.
        "});

        let base_content = match &self.git_entity {
            GitEntity::Commit(commit) => {
                formatdoc! {"
                    Context - Commit:

                    Message: {msg}
                    Changes:
                    ```diff
                    {diff}
                    ```
                    ",
                    msg = commit.message,
                    diff = commit.diff
                }
            }
            GitEntity::Diff(Diff::WorkingTree { diff, .. } | Diff::CommitsRange { diff, .. }) => {
                formatdoc! {"
                    Context - Changes:

                    ```diff
                    {diff}
                    ```
                    "
                }
            }
        };

        let user_prompt = match &self.git_entity {
            GitEntity::Commit(_) => formatdoc! {"
                    {base_content}
                    
                    Provide a short explanation covering:
                    1. Core changes made
                    2. Direct impact
                    "
            },
            GitEntity::Diff(Diff::WorkingTree { .. }) => formatdoc! {"
                    {base_content}
                    
                    Provide:
                    1. Key changes
                    2. Notable concerns (if any)
                    "
            },
            GitEntity::Diff(Diff::CommitsRange { .. }) => formatdoc! {"
                    {base_content}
                    
                    Provide:
                    1. Core changes made
                    2. Direct impact
                    "
            },
        };
        Ok(AIPrompt {
            system_prompt,
            user_prompt,
        })
    }
}

#[async_trait]
impl Command for ExplainCommand {
    async fn execute(&self, llm: LLMProvider) -> Result<(), GitAIError> {
        print_markdown(self.git_entity.format_static_details())?;

        let mut spinner = Spinner::new(
            spinners::Dots,
            "Generating summary...".to_string(),
            Color::Green,
        );

        let ai_prompt = self.get_ai_prompt().unwrap();

        let system_message = Message {
            role: Role::System,
            content: ai_prompt.system_prompt,
        };

        let user_message = Message {
            role: Role::User,
            content: ai_prompt.user_prompt,
        };

        let messages = vec![system_message, user_message];

        let response = llm.complete(&messages).await;

        spinner.stop();

        match response {
            Ok(response) => {
                print_markdown(response)?;
            }
            Err(e) => {
                println!("Error: {}", GitAIError::from(e));
            }
        }

        Ok(())
    }
}
