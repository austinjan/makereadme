// This file contains all LLM API related code.
use crate::conf;

use reqwest;
use std::error::Error;
use std::io::Read;
use std::io::Write;

// API defines the LLM API.
// ollama generate API enpoints from the API definition.
const OLLA_GEN: &str = r#"/api/generate"#;
const OLLA_EMBED: &str = r#"/api/embeddings"#;

// send_request sends a request to the LLM host, return a JSON response.
fn send_request(url: &str, data: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let res = client.post(url).body(data.to_string()).send()?;
    let body = res.text()?;
    Ok(body)
}

// define ollam generate API parameters
#[derive(Debug)]
pub struct GenerateParams {
    pub prompt: String,
    pub model: String,
    // advanced parameters
    pub format: String, // only accept json so far
}

// send request to ollama server, host and port read from config
pub fn ollama_generate(prompt: &str) -> Result<String, Box<dyn Error>> {
    let conf = match conf::read_config("conf.toml") {
        Ok(it) => it,
        Err(err) => return Err(err),
    };
    let url = format!("http://{}:{}{}", conf.host, conf.port, OLLA_GEN);
    let res = send_request(&url, prompt)?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;

    #[test]
    fn test_send_request() {
        // Start a dummy server
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let url = format!("http://127.0.0.1:{}", port);
        let data = "test data";

        // Spawn a thread to handle the request
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();
                let response = "HTTP/1.1 200 OK\r\n\r\n";
                stream.write(response.as_bytes()).unwrap();
                stream.write(data.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        });

        // Send a request to the dummy server
        let response = send_request(&url, data);

        // Check if the response is Ok
        assert!(response.is_ok());
    }
}
