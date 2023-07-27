use serde::{Deserialize, Serialize};

impl OpenAIChatRequest {
    pub fn new(model: &str, prompt: &str) -> Self {
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

    pub fn set_temperature(&mut self, temperature: f64) {
        self.temperature = Some(temperature);
    }
}

impl OpenAIChatResponse {
    fn get_top_choice(&self) -> &str {
        self.choices.first().unwrap().message.content.as_str()
    }
}
