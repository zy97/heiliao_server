use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};
mod config;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use config::{APP_CONFIG, PLAYER_HTML};
use moka::future::Cache;
use reqwest;
use scraper::{Html, Selector};
use serde_json::Value;
const ONE_WEEK_IN_SECONDS: u64 = 60 * 60 * 24 * 7;

#[get("/hl/{id}")]
async fn hl(
    config: web::Data<Arc<Mutex<HashMap<String, String>>>>,
    cache: web::Data<Cache<String, String>>,
    id: web::Path<String>,
) -> impl Responder {
    let key = format!("hl_{}", id);
    let video_url = cache.get(&key).await;
    match video_url {
        Some(url) => {
            let player_html = PLAYER_HTML.clone();
            let player_html = player_html.replace("#####", &url);
            HttpResponse::Ok()
                .content_type("text/html")
                .body(player_html)
        }
        None => {
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
                        let mut video_addresses = Vec::new();
                        let video_selectors = document.select(&selector);
                        for video in video_selectors {
                            let video_config = video.value().attr("config").unwrap_or("");
                            //序列化
                            let video_config: Value = serde_json::from_str(video_config).unwrap();
                            let url = video_config["video"]["url"].as_str().unwrap();

                            video_addresses.push(url.to_string());
                        }
                        let url = video_addresses.join("%&%&");
                        cache.insert(key, url.to_string()).await;
                        let player_html = PLAYER_HTML.clone();
                        let player_html = player_html.replace("#####", &url);
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
    }
}
#[get("/mrds/{id}")]
async fn mrds(
    config: web::Data<Arc<Mutex<HashMap<String, String>>>>,
    cache: web::Data<Cache<String, String>>,
    id: web::Path<String>,
) -> impl Responder {
    let key = format!("mrds_{}", id);
    let video_url = cache.get(&key).await;
    match video_url {
        Some(url) => {
            let player_html = PLAYER_HTML.clone();
            let player_html = player_html.replace("#####", &url);
            HttpResponse::Ok()
                .content_type("text/html")
                .body(player_html)
        }
        None => {
            let config = config.lock().unwrap();
            let url = config.get("meiridasai").unwrap();
            let url = format!("{}//archives/{}/", url, id);
            let client = reqwest::Client::new();
            let response = client.get(&url).send().await;
            match response {
                Ok(resp) => {
                    if resp.status().is_success() {
                        let html = resp.text().await.unwrap();
                        let document = Html::parse_document(&html);
                        let selector = Selector::parse(".dplayer").unwrap();
                        let mut video_addresses = Vec::new();
                        let video_selectors = document.select(&selector);
                        for video in video_selectors {
                            let video_config = video.value().attr("data-config").unwrap_or("");
                            //序列化
                            let video_config: Value = serde_json::from_str(video_config).unwrap();
                            let url = video_config["video"]["url"].as_str().unwrap();

                            video_addresses.push(url.to_string());
                        }
                        let url = video_addresses.join("%&%&");
                        cache.insert(key, url.to_string()).await;
                        let player_html = PLAYER_HTML.clone();
                        let player_html = player_html.replace("#####", &url);
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
    }
}
#[get("/cl/{id:.*}")]
async fn caoliu(
    config: web::Data<Arc<Mutex<HashMap<String, String>>>>,
    cache: web::Data<Cache<String, String>>,
    id: web::Path<String>,
) -> impl Responder {
    let key = format!("cl_{}", id);
    let video_url = cache.get(&key).await;
    match video_url {
        Some(url) => {
            let player_html = PLAYER_HTML.clone();
            let player_html = player_html.replace("#####", &url);
            HttpResponse::Ok()
                .content_type("text/html")
                .body(player_html)
        }
        None => {
            let config = config.lock().unwrap();
            let url = config.get("caoliu").unwrap();
            let url = format!("{}//htm_data/{}.html", url, id);
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("cookie", "ismob=0".parse().unwrap());
            let client = reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap();
            let response = client.get(&url).send().await;
            match response {
                Ok(resp) => {
                    if resp.status().is_success() {
                        let html = resp.text().await.unwrap();
                        let document = Html::parse_document(&html);
                        let selector = Selector::parse("#conttpc video").unwrap();
                        let mut video_addresses = Vec::new();
                        let video_selectors = document.select(&selector);
                        for video in video_selectors {
                            let url = video.value().attr("src").unwrap_or("");
                            video_addresses.push(url.to_string());
                        }
                        let url = video_addresses.join("%&%&");
                        cache.insert(key, url.to_string()).await;
                        let player_html = PLAYER_HTML.clone();
                        let player_html = player_html.replace("#####", &url);
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
    let cache: Cache<String, String> = Cache::builder()
        .time_to_live(Duration::from_secs(ONE_WEEK_IN_SECONDS))
        .build();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(cache.clone()))
            .service(hl)
            .service(mrds)
    })
    .bind(("0.0.0.0", 17618))?
    .run()
    .await
}
