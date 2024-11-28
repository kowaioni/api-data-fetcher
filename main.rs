use actix_web::{App, HttpServer, web, HttpResponse, Responder, Error as ActixError};
use reqwest::{Error as ReqwestError, Client};
use serde::{Deserialize, Serialize};
use futures::future::{ready, Ready};

#[derive(Deserialize)]
struct RequestData {
    url: String,
    method: String, // "GET" or "POST"
    payload: Option<serde_json::Value>, // Optional payload for POST requests
}

#[derive(Serialize)]
struct ResponseData {
    data: String,
}

async fn fetch_external_data(mut request_data: web::Json<RequestData>) -> impl Responder {
    let client = Client::new();
    let response_result = match request_data.method.to_uppercase().as_str() {
        "POST" => {
            let body = request_data.payload.take().unwrap_or_default();
            client.post(request_data.url.clone())
                  .json(&body)
                  .send()
                  .await
        },
        // Defaults to GET if not specified or any other method is provided
        _ => client.get(request_data.url.clone())
                   .send()
                   .await,
    };

    match response_result {
        Ok(response) => {
            if response.status().is_success() {
                let body = response.text().await.unwrap_or_else(|_| "".to_string()); // Use unwrap_or_else for better error handling
                HttpResponse::Ok().json(ResponseData { data: body })
            } else {
                HttpResponse::BadRequest().body("Failed to fetch data from the external API.")
            }
        },
        Err(_) => HttpResponse::BadRequest().body("Invalid URL or unable to reach the external API."),
    }
}

// This adjusts the return type to suit actix's expectations for errors in async routes
impl Responder for RequestData {
    type Error = ActixError;
    type Future = Ready<Result<HttpResponse, ActixError>>;

    fn respond_to(self, _: &actix_web::HttpRequest) -> Self::Future {
        ready(Ok(HttpResponse::Ok().finish()))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/fetch_data", web::post().to(fetch_external_data))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}