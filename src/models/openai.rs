use crate::{
    clients::{Message, OpenAIChatRequest, OpenAIChatResponse},
    prompts::Prompt,
};

impl OpenAIChatRequest {
    pub fn from_model_and_str(model: &str, prompt: &str) -> Self {
        let message = Message {
            role: "system".to_string(),
            content: prompt.to_string(),
        };

        Self {
            model: model.to_string(),
            messages: vec![message],
            temperature: None,
        }
    }

    pub fn from_model_and_prompt(model: &str, prompt: &Prompt) -> Self {
        let system_message = Message {
            role: "system".to_string(),
            content: prompt.prompt.clone(),
        };

        let user_message = Message {
            role: "user".to_string(),
            content: prompt.task.clone(),
        };

        Self {
            model: model.to_string(),
            messages: vec![system_message, user_message],
            temperature: None,
        }
    }
}

impl OpenAIChatResponse {
    pub fn get_top_choice(&self) -> String {
        self.choices.first().unwrap().message.content.clone()
    }
}
