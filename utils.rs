use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use dotenv::dotenv;

fn init_env() {
    dotenv().ok();
}

async fn send_get_request(url: &str, headers: HashMap<&str, &str>) -> Result<String, Error> {
    let client = Client::new();
    let mut request = client.get(url);

    for (key, value) in headers {
        request = request.header(key, value);
    }

    let response = request.send().await?;

    if response.status().is_success() {
        let body = response.text().await?;
        Ok(body)
    } else {
        Err(Error::from(response.error_for_status().unwrap_err()))
    }
}

async fn send_post_request<T: Serialize + ?Sized>(url: &str, headers: HashMap<&str, &str>, body: &T) -> Result<String, Error> {
    let client = Client::new();
    let mut request = client.post(url);

    for (key, value) in headers {
        request = request.header(key, value);
    }

    let response = request.json(body).send().await?;

    if response.status().is_success() {
        let body = response.text().await?;
        Ok(body)
    } else {
        Err(Error::from(response.error_for_status().unwrap_err()))
    }
}

async fn parse_json_response<T: for<'de> Deserialize<'de>>(response_data: String) -> Result<T, serde_json::Error> {
    serde_json::from_str(&response_data)
}

async fn example_usage() -> Result<(), Box<dyn std::error::Error>> {
    init_env();

    let api_key = env::var("API_KEY")?;

    let url = "https://api.example.com/data";
    let mut headers = HashMap::new();
    headers.insert("Authorization", &api_key);
    headers.insert("Content-Type", "application/json");

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