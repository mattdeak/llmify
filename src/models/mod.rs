pub mod openai;
use std::fmt;

pub trait LanguageModel {
    fn generate(&self, input: &str) -> f64;
}

pub enum OpenAIModel {
    Gpt3Turbo,
    Gpt3Turbo16k,
    Gpt4,
}

impl fmt::Display for OpenAIModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpenAIModel::Gpt3Turbo => write!(f, "gpt-3.5-turbo"),
            OpenAIModel::Gpt3Turbo16k => write!(f, "gpt-3.5-turbo-16k"),
            OpenAIModel::Gpt4 => write!(f, "gpt-4"),
        }
    }
}

struct OpenAILanguageModel {
    model_name: OpenAIModel,
    temperature: f64,
}

impl OpenAILanguageModel {
    pub fn new(model_name: OpenAIModel) -> Self {
        Self {
            model_name,
            temperature: 0.0,
        }
    }

    pub fn set_temperature(&mut self, temperature: f64) {
        self.temperature = temperature;
    }
}

impl LanguageModel for OpenAILanguageModel {
    fn generate(&self, input: &str) -> f64 {
        let mut request = openai::OpenAIChatRequest::new(&self.model_name.to_string(), input);
        request.set_temperature(self.temperature);
        let response = openai::chat(&request);
        let mut score = 0.0;
        for choice in response.choices {
            score += choice.message.content.len() as f64;
        }
        score
    }
}
