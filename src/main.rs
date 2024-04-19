mod conf;
mod fileop;
mod llmop;
use futures_util::stream::StreamExt;
use reqwest::Response;

#[tokio::main]
async fn main() {
    //  send gathering files content to stdout
    println!("Gathering files content...");
    let file_contents = match fileop::list_files_content(".") {
        Ok(it) => it,
        Err(err) => {
            eprintln!("Failed to list files content: {}", err);
            return;
        }
    };

    let conf = match conf::read_config_or_default("conf.toml") {
        Ok(it) => it,
        Err(err) => {
            eprintln!("Failed to read configuration: {}", err);
            return;
        }
    };
    let url = format!("http://{}:{}{}", conf.host, conf.port, llmop::OLLA_CHAT);

    let user_message = llmop::Message {
        role: "user".to_string(),
        content: file_contents,
    };

    let param = llmop::ChatParams {
        messages: vec![user_message],
        model: "llama2".to_string(),
        stream: false,
    };

    let param: String = match serde_json::to_string(&param) {
        Ok(it) => it,
        Err(err) => {
            eprintln!("Failed to serialize request: {}", err);
            return;
        }
    };

    let stream: Response = match llmop::send_request_stream(&url, &param).await {
        Ok(it) => it,
        Err(err) => {
            eprintln!("Failed to send request: {}", err);
            return;
        }
    };

    if stream.status().is_success() {
        println!(">>>>>>>>");
        let mut buf = stream.bytes_stream();
        while let Some(item) = buf.next().await {
            match item {
                Ok(it) => {
                    let body = String::from_utf8(it.to_vec()).unwrap();
                    let res: llmop::ChatResponse = match serde_json::from_str(&body) {
                        Ok(it) => it,
                        Err(err) => {
                            eprintln!("Failed to parse response: {}", err);
                            return;
                        }
                    };
                    let answer: String = match res.answer() {
                        None => {
                            eprintln!("Failed to get answer");
                            return;
                        }
                        Some(it) => it,
                    };
                    // print response
                    print!("{}", answer);
                    // check done flag if ture break
                    if res.done {
                        break;
                    }
                }
                Err(err) => {
                    eprintln!("Failed to read response: {}", err);
                    return;
                }
            }
        }
    }
}
