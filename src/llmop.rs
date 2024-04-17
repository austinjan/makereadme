// This file contains all LLM API related code.
use reqwest;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::error::Error;

// API defines the LLM API.
// ollama generate API enpoints from the API definition.
pub const OLLA_CHAT: &str = r#"/api/chat"#;

// send_request sends a request to the LLM host, return a JSON response.
// fn send_request(url: &str, data: &str) -> Result<String, Box<dyn Error>> {
//     let client = reqwest::blocking::Client::builder()
//         .timeout(Duration::from_secs(600))
//         .build()?;

//     let res = client.post(url).body(data.to_string()).send()?;
//     let body = res.text()?;
//     Ok(body)
// }

/// send a request and return a stream of response
/// url: the url to send the request
/// data: the data to send
/// return: a stream of response
/// error: return Box<dyn Error> if error
pub async fn send_request_stream(url: &str, data: &str) -> Result<Response, Box<dyn Error>> {
    let client = Client::new();

    let res = client.post(url).body(data.to_string()).send().await?;
    Ok(res)
}

// define ollam generate API parameters
#[derive(Serialize, Deserialize)]
pub struct GenerateParams {
    pub prompt: String,
    pub model: String,
    // advanced parameters
    pub format: String, // only accept json so far
}

// message object
#[derive(Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

// /api/chat request parameters
#[derive(Serialize, Deserialize)]
pub struct ChatParams {
    pub messages: Vec<Message>,
    pub model: String,
    pub stream: bool,
}

// /api/chat response
#[derive(Serialize, Deserialize)]
pub struct ChatResponse {
    pub message: Option<Message>,
    pub model: String,
    pub created_at: String,
    pub total_duration: u64,
    pub load_duration: u64,
    pub prompt_eval_duration: u64,
    pub eval_duration: u64,
    pub eval_count: u64,
    pub done: bool,
}

impl ChatResponse {
    // answer return chatresponse.message.content
    pub fn answer(&self) -> Option<String> {
        match &self.message {
            Some(it) => Some(it.content.clone()),
            None => None,
        }
    }

    #[allow(dead_code)]
    // total_duration return total_duration, convert nanoseconds to seconds
    pub fn total_duration(&self) -> Option<f64> {
        Some(self.total_duration as f64 / 1_000_000_000.0)
    }

    #[allow(dead_code)]
    // load_duration return load_duration, convert nanoseconds to seconds
    pub fn load_duration(&self) -> Option<f64> {
        Some(self.load_duration as f64 / 1_000_000_000.0)
    }
}
