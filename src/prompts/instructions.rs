use std::fmt::Display;

/// A struct that holds the instructions for formatting the prompt.
/// The split token is used to mark the beginning of the output
#[derive(Clone, Debug)]
pub struct BehaviourInstructions {
    pub instructions: String,
    pub output_token: String,
}

impl Display for BehaviourInstructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.instructions)
    }
}

impl BehaviourInstructions {
    pub fn new(instructions: &str, output_token: &str) -> Self {
        Self {
            instructions: instructions.to_string(),
            output_token: output_token.to_string(),
        }
    }
}
