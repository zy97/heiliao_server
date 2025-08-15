use std::{
    collections::HashMap,
    fs,
    sync::{Arc, Mutex},
    time::Duration,
};
mod config;
use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, cookie::time::format_description::parse,
    get, web,
};
use config::{APP_CONFIG, PLAYER_HTML};
use reqwest;
use scraper::{Html, Selector, selector};
use serde_json::Value;
#[get("/greet/{name}")]
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}
#[get("/hl/{id}")]
async fn hl(
    config: web::Data<Arc<Mutex<HashMap<String, String>>>>,
    id: web::Path<String>,
) -> impl Responder {
    let config = config.lock().unwrap();
    let heiliao = config.get("heiliao").unwrap();
    let url = format!("{}//archives/{}/", heiliao, id);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await;
    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let html = resp.text().await.unwrap();
                let document = Html::parse_document(&html);
                let selector = Selector::parse(".dplayer").unwrap();
                let video = document.select(&selector).next().unwrap();
                let video_config = video.value().attr("config").unwrap_or("");
                //序列化
                let video_config: Value = serde_json::from_str(video_config).unwrap();
                let url = video_config["video"]["url"].as_str().unwrap();
                println!("Video URL: {}", url);
                let player_html = PLAYER_HTML.clone();
                let player_html = player_html.replace("#####", url);
                HttpResponse::Ok()
                    .content_type("text/html")
                    .body(player_html)
            } else {
                HttpResponse::BadRequest().body("Failed to fetch data")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 创建配置
    let config: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    {
        let mut config = config.lock().unwrap();
        config.insert("heiliao".to_string(), APP_CONFIG.config.heiliao.clone());
        config.insert(
            "meiridasai".to_string(),
            APP_CONFIG.config.meiridasai.clone(),
        );
    }
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .service(hl)
    })
    .bind(("0.0.0.0", 17618))?
    .run()
    .await
}
