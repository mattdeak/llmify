use std::fmt::Display;

#[derive(Debug)]
/// A struct that holds the instructions for formatting the prompt.
/// The split token is used to mark the beginning of the output
struct FormatInstructions {
    pub instructions: String,
    output_token: String,
}

impl Display for FormatInstructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.instructions)
    }
}

impl FormatInstructions {
    pub fn new(instructions: &str, output_token: &str) -> Self {
        Self {
            instructions: instructions.to_string(),
            output_token: output_token.to_string(),
        }
    }
}
