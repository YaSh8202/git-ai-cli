use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CommitReference {
    Single(String),
    Range { from: String, to: String },
    TripleDots { from: String, to: String },
}
#[derive(Debug, Error)]
pub enum ReferenceParseError {
    #[error("Invalid commit reference")]
    Empty,
}

impl FromStr for CommitReference {
    type Err = ReferenceParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ReferenceParseError::Empty);
        }

        // Handle the ... and .. cases
        if let Some((from, to)) = s.split_once("...") {
            let from = if from.is_empty() { "HEAD" } else { from };
            let to = if to.is_empty() { "HEAD" } else { to };

            Ok(CommitReference::TripleDots {
                from: from.to_string(),
                to: to.to_string(),
            })
        } else if let Some((from, to)) = s.split_once("..") {
            let from = if from.is_empty() { "HEAD" } else { from };
            let to = if to.is_empty() { "HEAD" } else { to };

            Ok(CommitReference::Range {
                from: from.to_string(),
                to: to.to_string(),
            })
        } else {
            Ok(CommitReference::Single(s.to_string()))
        }
    }
}
