use reqwest::{Client, header::{USER_AGENT, HeaderValue}};
use serde_json::json;
use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Response {
    choices: Choices,
}

#[derive(Serialize, Deserialize)]
struct Choices {
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let response = Client::new()
        .post("https://api.openai.com/v1/engines/davinci-codex/completions")
        .header(USER_AGENT, HeaderValue::from_static("reqwest"))
        .header("Content-Type", HeaderValue::from_static("application/json"))
        .bearer_auth("<insert your api key hereeeee")
        .json(&json!({
            "prompt": "Tell me a story about some fox.",
            "temperature": 0.5,
            "max_tokens": 100
        }))
        .send()
        .await?
        .text()
        .await?;

       
    println!("{}", response);
 
    Ok(())
}
