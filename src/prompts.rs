pub const SUMMARIZE: &str = "Analyze the text provided and summarize it in your own words. If the text appears to be raw data instead of text, try to infer what the raw data might represent, then analyze and summarize the data. Be analytical and concise. Do not include any of the original text in your response. If the text cannot be summarized, write \"Cannot be summarized\".";

pub fn format_prompt(prompt: &str, task: &str, custom_instructions: &Option<String>) -> String {
    let mut task = task.to_string();

    if let Some(custom_instructions) = custom_instructions {
        let formatted_custom_instructions = format!(
            "\nAdditionally, closely follow these instructions: {}",
            custom_instructions.trim()
        );
        task.push_str(custom_instructions);
    }
    format!("{}\n\nInput: {}", prompt, task)
}
