use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use base64;
use reqwest::{header, Client};
use serde_derive::Deserialize;
use serde_json::json;
use tokio::{fs::File, io::AsyncReadExt};

use std::env;
use std::fmt::Debug;
use std::path::Path;
use std::sync::Arc;

#[derive(Debug)]
struct State {
    domain_name: String,
    client: Client,
}

#[derive(Debug, Deserialize)]
struct Request {
    to: String,
    subject: String,
    text: String,
}

async fn send_email(data: web::Data<Arc<State>>, request: web::Json<Request>) -> impl Responder {
    handle_send_email(data, request).await
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let (ip, port, workers, secret_path) = read_env();

    let login = read_file(&secret_path, "login").await?.trim().to_string();
    let api_key = read_file(&secret_path, "api_key").await?.trim().to_string();
    let domain_name = read_file(&secret_path, "domain_name")
        .await?
        .trim()
        .to_string();

    let client = build_client(&login, &api_key).map_err(|_| std::io::ErrorKind::Other)?;

    let state = Arc::new(State {
        domain_name,
        client,
    });

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .route("/api/v1/send_email", web::post().to(send_email))
            .default_service(web::route().to(HttpResponse::NotFound))
            .wrap(middleware::Logger::default())
    })
    .bind(format!("{}:{}", ip, port))?
    .workers(workers)
    .run()
    .await
}

fn read_env() -> (String, u64, usize, String) {
    (
        env::var("SERVER_IP").unwrap_or_else(|_| "127.0.0.1".to_string()),
        env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .expect("can not parse server port"),
        env::var("SERVER_WORKERS")
            .unwrap_or_else(|_| "1".to_string())
            .parse()
            .expect("can not parse server workers"),
        env::var("SECRET_PATH").unwrap_or_else(|_| "secret".to_string()),
    )
}

fn build_client(login: &str, api_key: &str) -> Result<Client, reqwest::Error> {
    let mut headers = header::HeaderMap::new();
    let auth_data = format!(
        "Basic {}",
        base64::encode(&format!("{}:{}", login, api_key))
    );
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&auth_data).expect("invalid auth_data"),
    );
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("email-sender"),
    );

    Client::builder().default_headers(headers).build()
}

async fn read_file(path: &str, name: &str) -> std::io::Result<String> {
    let file_path = Path::new(path).join(name);
    let mut data = vec![];
    let mut file = File::open(file_path).await?;
    file.read_to_end(&mut data).await?;
    Ok(String::from_utf8(data).unwrap_or_else(|_| panic!("invalid {}", name)))
}

async fn handle_send_email(
    data: web::Data<Arc<State>>,
    request: web::Json<Request>,
) -> std::io::Result<String> {
    log::info!("{:?}", request);
    let params = [
        ("from", &from(&data.domain_name)),
        ("to", &request.to),
        ("subject", &request.subject),
        ("text", &request.text),
    ];
    let response = data
        .client
        .post(&url(&data.domain_name))
        .form(&params)
        .send()
        .await
        .map_err(|e| {
            log::error!("request error: {:?}", e);
            std::io::ErrorKind::Other
        })?;
    log::info!(
        "sent email, to: {}, http_status: {}, response: {:?}",
        request.to,
        response.status(),
        response.text().await
    );

    Ok(json!({ "status": "ok" }).to_string())
}

fn from(domain_name: &str) -> String {
    format!("noreply@{}", domain_name)
}

fn url(domain_name: &str) -> String {
    format!("https://api.mailgun.net/v3/{}/messages", domain_name)
}
