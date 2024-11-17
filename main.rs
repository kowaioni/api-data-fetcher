use actix_web::{App, HttpServer, web, HttpResponse, Responder};
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct RequestData {
    url: String,
}

#[derive(Serialize)]
struct ResponseData {
    data: String,
}

async fn fetch_external_data(request_data: web::Json<RequestData>) -> impl Responder {
    let client = reqwest::Client::new();
    let response_result = client.get(request_data.url.clone())
        .send()
        .await;

    match response_result {
        Ok(response) => {
            if response.status().is_success() {
                let body = response.text().await.unwrap_or_default();
                HttpResponse::Ok().json(ResponseData { data: body })
            } else {
                HttpResponse::BadRequest().body("Failed to fetch data from the external API.")
            }
        },
        Err(_) => HttpResponse::BadRequest().body("Invalid URL or unable to reach the external API."),
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