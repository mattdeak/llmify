pub mod openai;
use crate::prompts::Prompt;

use super::clients::{OpenAIChatRequest, OpenAIClient};
use std::{fmt, str::FromStr};

pub trait LanguageModel<T> {
    fn generate(&self, input: T) -> String;
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

pub struct OpenAILanguageModel<'a> {
    client: &'a OpenAIClient,
    pub model_name: OpenAIModel,
}

impl<'a> OpenAILanguageModel<'a> {
    pub fn new(client: &'a OpenAIClient, model_name: OpenAIModel) -> Self {
        Self { client, model_name }
    }
}

impl<'a, S: AsRef<str>> LanguageModel<S> for OpenAILanguageModel<'a> {
    fn generate(&self, input: S) -> String {
        let request =
            OpenAIChatRequest::from_model_and_str(&self.model_name.to_string(), input.as_ref());
        let response = self.client.send_chat_request(request);
        response.get_top_choice()
    }
}

impl<'a> LanguageModel<Prompt> for OpenAILanguageModel<'a> {
    fn generate(&self, input: Prompt) -> String {
        let request =
            OpenAIChatRequest::from_model_and_prompt(&self.model_name.to_string(), &input);
        let response = self.client.send_chat_request(request);
        response.get_top_choice()
    }
}
