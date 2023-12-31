use super::instructions::BehaviourInstructions;

pub const SUMMARIZE: &str = r#"Analyze the input provided and summarize it in your own words. 
If the text appears to be raw data instead of text, try to infer what the raw data might represent, then analyze and summarize the data.
Be analytical and concise. Do not include any of the original text in your response. If the text cannot be summarized, write "Cannot be summarized"
The text to summarize will be formatted as:

INPUT: [text to summarize]
"#;

pub const QUERY_DATA: &str = r#"Analyze the input and the user-provided question and answer the question. 
If the question is unanswerable based on the content of the input, write "Cannot be answered".
Be clear and concise. 

The formatting of the input will be:
QUESTION: [question to answer]
INPUT: [text to analyze]
"#;

pub const ASK_QUESTION: &str = r#"Provide a response to the question that the user asks. 
Try to answer the question, but if the answer is uncertain make sure to say so.
Be clear and concise.

The formatting of the input will be:
QUESTION: [question to answer]
"#;

pub struct InstructionSelector;

impl InstructionSelector {
    pub fn self_critique() -> BehaviourInstructions {
        BehaviourInstructions {
            instructions: r#"Format your output like so:
        INITIAL: [initial text]
        CRITIQUE: [critique of initial text]
        FINAL: [final text]
        "#
            .into(),
            output_token: "FINAL: ".to_string(),
        }
    }

    pub fn tree_of_thoughts() -> BehaviourInstructions {
        BehaviourInstructions {
            instructions: r#"Format your output like so:
Imagine three different experts are asked to complete this task.
All experts will write down 1 step of their thinking, then share it with the group.
Then all experts will go on to the next step, etc.
If any expert realises they're wrong at any point then they leave.
If all remaining experts agree, then you're done.
When you're done, write the final answer as:
"FINAL: [your final answer]""#
                .to_string(),
            output_token: "FINAL: ".to_string(),
        }
    }
}
