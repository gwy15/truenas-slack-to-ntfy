use actix_web::{web, App, Error, HttpServer};
use std::env;
use tracing::*;

#[derive(serde::Deserialize)]
pub struct Request {
    text: String,
}
#[derive(Clone)]
pub struct Config {
    ntfy_base_url: String,
}

async fn forward(
    path: web::Path<(String,)>,
    body: web::Json<Request>,
    config: web::Data<Config>,
) -> Result<web::Json<serde_json::Value>, Error> {
    let topic = path.into_inner().0;
    let content = body.into_inner().text;
    let url = format!("{}/{topic}", config.ntfy_base_url);
    let response = reqwest::Client::new()
        .post(url)
        .body(content)
        .send()
        .await
        .map_err(|e| {
            warn!(?e, "send request to ntfy failed");
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("send request to ntfy failed: {e:?}"),
            )
        })?;

    let response = response.json().await.map_err(|e| {
        warn!(?e, "forward request to ntfy failed");
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("forward request to ntfy failed: {e:?}"),
        )
    })?;
    Ok(web::Json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    let bind = env::var("BIND")
        .ok()
        .and_then(|s| s.parse::<std::net::SocketAddr>().ok())
        .unwrap_or_else(|| "0.0.0.0:80".parse().unwrap());
    let ntfy_base_url = env::var("NTFY_BASE_URL").unwrap_or_else(|_| "https://ntfy.sh".to_string());
    let config = Config { ntfy_base_url };
    let listen_base_path = env::var("LISTEN_BASE_PATH").unwrap_or_else(|_| "/".to_string());
    let path = format!("{listen_base_path}{{topic}}");

    info!("listening on path {path} and {bind}.");
    info!("will forward requests to {}", config.ntfy_base_url);

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(config.clone()))
            .route(&path, web::post().to(forward))
    })
    .bind(bind)?
    .run()
    .await
}
