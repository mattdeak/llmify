use serde::{Deserialize, Serialize};

const OPENAI_API_CHAT_URL: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct OpenAIChatRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    pub index: usize,
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIChatResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub choices: Vec<Choice>,
}

pub struct OpenAIClient {
    api_key: String,
}

impl OpenAIClient {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }

    pub fn send_chat_request(&self, request: OpenAIChatRequest) -> OpenAIChatResponse {
        let client = reqwest::blocking::Client::new();

        let request_body = serde_json::to_string(&request).unwrap();
        let response = client
            .post(OPENAI_API_CHAT_URL)
            .bearer_auth(self.api_key)
            .header("Content-Type", "application/json")
            .body(request_body)
            .send()
            .unwrap();

        let response_body = response.text().unwrap();
        serde_json::from_str(&response_body).unwrap()
    }
}
