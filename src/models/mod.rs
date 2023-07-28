pub mod openai;
use crate::prompts::prompt::Prompt;

use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub struct GenerationError(String);

impl Display for GenerationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GenerationError: {}", self.0)
    }
}

impl Error for GenerationError {}

pub trait LanguageModel {
    fn generate(&self, input: Prompt) -> Result<String, GenerationError>;
}
