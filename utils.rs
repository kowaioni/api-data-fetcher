use reqwest::{Client, Error, header::HeaderMap};
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;

fn init_env() {
    dotenv().ok();
}

async fn send_get_request(url: &str, headers: HeaderMap) -> Result<String, Error> {
    let client = Client::new();
    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await?;

    match response.error_for_status_ref() {
        Ok(_) => Ok(response.text().await?),
        Err(e) => Err(Error::from(e.to_owned())),
    }
}

async fn send_post_request<T: Serialize + ?Sized>(url: &str, headers: HeaderMap, body: &T) -> Result<String, Error> {
    let client = Client::new();
    let response = client
        .post(url)
        .headers(headers)
        .json(body)
        .send()
        .await?;

    match response.error_for_status_ref() {
        Ok(_) => Ok(response.text().await?),
        Err(e) => Err(Error::from(e.to_owned())),
    }
}

async fn parse_json_response<T: for<'de> Deserialize<'de>>(response_data: String) -> Result<T, serde_json::Error> {
    serde_json::from_str(&response_data)
}

async fn example_usage() -> Result<(), Box<dyn std::error::Error>> {
    init_env();

    let api_key = env::var("API_KEY")?;

    let url = "https://api.example.com/data";
    
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", api_key.parse()?);
    headers.insert("Content-Type", "application/json".parse()?);

    let get_response = send_get_request(url, headers.clone()).await?;
    println!("GET Response: {}", get_response);

    let post_body = serde_json::json!({
        "data": "Example data"
    });
    let post_response = send_post_request(url, headers, &post_body).await?;
    println!("POST Response: {}", post_response);

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = example_usage().await {
        eprintln!("Error: {}", e);
    }
}