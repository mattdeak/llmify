pub const SUMMARIZE: &str = r#"Analyze the input provided and summarize it in your own words. 
If the text appears to be raw data instead of text, try to infer what the raw data might represent, then analyze and summarize the data.
Be analytical and concise. Do not include any of the original text in your response. If the text cannot be summarized, write "Cannot be summarized"
The text to summarize will be formatted as:

INPUT: [text to summarize]
"#;

pub const QUERY: &str = r#"Analyze the input and the user-provided question and answer the question. 
If the question is unanswerable based on the content of the input, write "Cannot be answered".
Be clear and concise. 

The formatting of the input will be:
QUESTION: [text to analyze]
INPUT: [question to answer]
"#;
