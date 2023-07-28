use super::instructions::FormatInstructions;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct PromptBuilderError(String);

impl Display for PromptBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for PromptBuilderError {}

pub struct PromptBuilder {
    prefix: Option<String>,
    format_instructions: Option<FormatInstructions>,
    task: Option<String>,
}

#[derive(Debug)]
pub struct Prompt {
    pub prefix: String,
    pub formatting_instructions: Option<FormatInstructions>,
    pub task: String,
}

impl PromptBuilder {
    pub fn prefix(mut self, prefix: &str) -> Self {
        self.prefix = Some(prefix.to_string());
        self
    }

    pub fn formatting_instructions(mut self, instructions: &FormatInstructions) -> Self {
        self.formatting_instructions = instructions.clone();
        self
    }

    pub fn task(mut self, task: &str) -> Self {
        self.task = Some(task.to_string());
        self
    }

    pub fn build(self) -> Result<Prompt, PromptBuilderError> {
        if self.task.is_none() {
            return Err(PromptBuilderError("No task provided".to_string()));
        }

        if self.prefix.is_none() {
            return Err(PromptBuilderError("No prefix provided".to_string()));
        }

        let prefix = self.prefix.unwrap();
        let formatting_instructions = self.formatting_instructions;
        let task = self.task.unwrap();

        Ok(Prompt {
            prefix,
            formatting_instructions,
            task,
        })
    }
}

impl Prompt {
    pub fn new() -> PromptBuilder {
        PromptBuilder {
            prefix: None,
            formatting_instructions: None,
            task: None,
        }
    }
}

impl Display for Prompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(instructions) = &self.formatting_instructions {
            write!(
                f,
                "{}\n{}\nInput: {}",
                &self.prefix, instructions, &self.task
            )
        } else {
            write!(f, "{}\nInput: {}", &self.prefix, &self.task)
        }
    }
}
