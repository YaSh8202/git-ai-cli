use crate::llm::LLMComplete;

use super::AIPrompt;
use super::Command;
use crate::git_entity::diff::Diff;
use crate::git_entity::GitEntity;
use crate::llm::{AIPromptError, LLMProvider, Message, Role};
use indoc::{formatdoc, indoc};

use crate::error::GitAIError;
use async_trait::async_trait;

fn default_commit_types() -> String {
    indoc! {r#"
    {
        "docs": "Documentation only changes",
        "style": "Changes that do not affect the meaning of the code",
        "refactor": "A code change that neither fixes a bug nor adds a feature",
        "perf": "A code change that improves performance",
        "test": "Adding missing tests or correcting existing tests",
        "build": "Changes that affect the build system or external dependencies",
        "ci": "Changes to our CI configuration files and scripts",
        "chore": "Other changes that don't modify src or test files",
        "revert": "Reverts a previous commit",
        "feat": "A new feature",
        "fix": "A bug fix"
    }
    "#}
    .to_string()
}

pub struct GenerateCommand {
    pub git_entity: GitEntity,
}

// #[async_trait]
// use crate::{error::GitAIError, llm::LLMComplete};

impl GenerateCommand {
    pub fn get_ai_prompt(&self) -> Result<AIPrompt, AIPromptError> {
        let GitEntity::Diff(Diff::WorkingTree { diff, .. }) = &self.git_entity else {
            return Err(AIPromptError(
                "`draft` is only supported for working tree diffs".into(),
            ));
        };

        let system_prompt = String::from(indoc! {"
            You are a commit message generator that follows these rules:
            1. Write in present tense
            2. Be concise and direct
            3. Output only the commit message without any explanations
            4. Follow the format: <type>(<optional scope>): <commit message>
        "});

        let user_prompt = String::from(formatdoc! {"
            Generate a concise git commit message written in present tense for the following code diff with the given specifications below:

            The output response must be in format:
            <type>(<optional scope>): <commit message>
            Choose a type from the type-to-description JSON below that best describes the git diff:
            {commit_types}
            Focus on being accurate and concise.
            Commit message must be a maximum of 72 characters.
            Exclude anything unnecessary such as translation. Your entire response will be passed directly into git commit.

            Code diff:
            ```diff
            {diff}
            ```
            ",
            commit_types = default_commit_types(),
        });

        Ok(AIPrompt {
            system_prompt,
            user_prompt,
        })
    }
}

#[async_trait]
impl Command for GenerateCommand {
    async fn execute(&self, llm: LLMProvider) -> Result<(), GitAIError> {
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

        match response {
            Ok(commit_message) => {
                println!("Commit message: {}", commit_message);
            }
            Err(e) => {
                println!("Error: {}", GitAIError::from(e));
            }
        }

        Ok(())
    }
}
