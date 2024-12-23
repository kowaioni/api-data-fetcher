use actix_web::{web, App, HttpResponse, HttpServer, HttpRequest, Responder, Error};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Mutex};
use dotenv::dotenv;
use std::env;

#[derive(Serialize, Deserialize)]
struct ApiPreset {
    url: String,
    method: String,
    headers: HashMap<String, String>,
    body: String,
}

struct AppState {
    presets: Mutex<HashMap<String, ApiPreset>>,
}

async fn fetch_data(req: HttpRequest, body: web::Bytes, data: web::Data<AppState>) -> impl Responder {
    let query = req.query_string();
    let presets_lock = data.presets.lock().expect("Failed to lock mutex");
    let preset = presets_lock.get(query);
    
    let url = preset.map_or_else(|| req.uri().to_string(), |p| p.url.clone());

    let client = reqwest::Client::new();
    let mut request_builder = client.request(req.method().clone(), &url);
    if let Some(p) = preset {
        for (key, value) in &p.headers {
            request_builder = request_builder.header(key, value);
        }
    }

    let response = request_builder.body(body.to_vec()).send().await;

    match response {
        Ok(res) => match res.text().await {
            Ok(body) => HttpResponse::Ok().content_type("application/json").body(body),
            Err(e) => HttpResponse::InternalServerError().body(format!("Failed to read response body: {}", e)),
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Request failed: {}", e)),
    }
}

async fn save_preset(data: web::Data<AppState>, preset: web::Json<ApiPreset>) -> impl Responder {
    let mut presets = data.presets.lock().expect("Failed to lock mutex");
    presets.insert(preset.url.clone(), preset.into_inner());
    HttpResponse::Ok().body("Preset saved")
}

async fn load_preset(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let query = req.query_string();
    let presets = data.presets.lock().expect("Failed to lock mutex");
    if let Some(preset) = presets.get(query) {
        HttpResponse::Ok().json(preset)
    } else {
        HttpResponse::NotFound().body("Preset not found")
    }
}

#[actix_web::main] 
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    let app_data = web::Data::new(AppState {
        presets: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .route("/api/fetch", web::to(fetch_data))
            .route("/api/save_preset", web::post().to(save_preset))
            .route("/api/load_preset", web::get().to(load_preset))
    })
    .bind(bind_address)?
    .run()
    .await
}