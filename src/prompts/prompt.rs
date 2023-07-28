use super::instructions::BehaviourInstructions;
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
    behaviour_instructions: Option<BehaviourInstructions>,
    task: Option<String>,
}

#[derive(Debug)]
pub struct Prompt {
    pub prefix: String,
    pub behaviour_instructions: Option<BehaviourInstructions>,
    pub task: String,
}

impl PromptBuilder {
    pub fn with_prefix(mut self, prefix: &str) -> Self {
        self.prefix = Some(prefix.to_string());
        self
    }

    pub fn with_behaviour_instructions(mut self, instructions: &BehaviourInstructions) -> Self {
        self.behaviour_instructions = Some(instructions.clone());
        self
    }

    pub fn with_task(mut self, task: &str) -> Self {
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
        let behaviour_instructions = self.behaviour_instructions;
        let task = self.task.unwrap();

        Ok(Prompt {
            prefix,
            behaviour_instructions,
            task,
        })
    }
}

impl Prompt {
    pub fn new() -> PromptBuilder {
        PromptBuilder {
            prefix: None,
            behaviour_instructions: None,
            task: None,
        }
    }
}

impl Display for Prompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(instructions) = &self.behaviour_instructions {
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
