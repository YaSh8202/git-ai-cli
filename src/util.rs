use crate::error::GitAIError;
use termimad::print_text;

pub fn print_markdown(content: String) -> Result<(), GitAIError> {
    print_text(&content);
    Ok(())
}
