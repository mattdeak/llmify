use std::fmt;
use std::str::FromStr;

use crate::{
    clients::{Message, OpenAIChatRequest, OpenAIChatResponse, OpenAIClient},
    prompts::prompt::Prompt,
};

use super::{GenerationError, LanguageModel};

pub enum OpenAIModel {
    Gpt3Turbo,
    Gpt3Turbo16k,
    Gpt4,
}

pub struct OpenAILanguageModel<'a> {
    client: &'a OpenAIClient,
    pub model_name: OpenAIModel,
}

impl OpenAIChatRequest {
    pub fn new(model: &str, prompt: &Prompt) -> Self {
        let system_prompt = match &prompt.format_instructions {
            Some(instructions) => format!("{}{}", instructions, prompt.prefix),
            None => prompt.prefix.clone(),
        };

        let system_message = Message {
            role: "system".to_string(),
            content: system_prompt,
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
impl fmt::Display for OpenAIModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpenAIModel::Gpt3Turbo => write!(f, "gpt-3.5-turbo"),
            OpenAIModel::Gpt3Turbo16k => write!(f, "gpt-3.5-turbo-16k"),
            OpenAIModel::Gpt4 => write!(f, "gpt-4"),
        }
    }
}

impl FromStr for OpenAIModel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gpt-3.5-turbo" => Ok(OpenAIModel::Gpt3Turbo),
            "gpt-3.5-turbo-16k" => Ok(OpenAIModel::Gpt3Turbo16k),
            "gpt-4" => Ok(OpenAIModel::Gpt4),
            _ => Err(()),
        }
    }
}

impl<'a> OpenAILanguageModel<'a> {
    pub fn new(client: &'a OpenAIClient, model_name: OpenAIModel) -> Self {
        Self { client, model_name }
    }
}

impl<'a> LanguageModel for OpenAILanguageModel<'a> {
    fn generate(&self, input: Prompt) -> Result<String, GenerationError> {
        let request = OpenAIChatRequest::new(&self.model_name.to_string(), &input);
        let response = self.client.send_chat_request(request);
        let choice = response.get_top_choice();

        if let Some(instructions) = input.format_instructions {
            let split = choice
                .split(&instructions.output_token)
                .collect::<Vec<&str>>();

            if split.len() > 1 {
                return Ok(split[1].to_string());
            }

            return Err(GenerationError(format!(
                "Could not split output with token {}",
                instructions.output_token
            )));
        }

        Ok(choice)
    }
}
