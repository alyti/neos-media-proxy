//! Example websocket server.
//!
//! Run with
//!
//! ```not_rust
//! cd examples && cargo run -p example-websockets
//! ```

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        TypedHeader,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};

use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
};
use tokio::fs;
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use secrecy::ExposeSecret;
mod config;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "neos_media_proxy=debug,media_provider=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = load_config(None).await?;
    tracing::debug!("loaded config: {:?}", config);

    let providers = media_provider::providers();
    tracing::debug!("providers: {:?}", providers);

    for (_, profile) in config.profiles {
        let session = match providers.get(profile.provider.as_str()) {
            Some(p) => p.login(&profile.hostname, &profile.username, &profile.password.expose_secret()).await?,
            None => {
                tracing::error!("unknown profile provider: {}", profile.provider);
                continue
            },
        };

        session.libraries().await

    }

    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    // build our application with some routes
    let app = Router::new()
        .fallback(
            get_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
                .handle_error(|error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                }),
        )
        // routes are matched from bottom to top, so we have to put `nest` at the
        // top since it matches all routes
        .route("/ws", get(ws_handler))
        // logging so we can see whats going on
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    // run it with hyper
    let addr = config
        .server
        .addr
        .unwrap_or(SocketAddr::from(([127, 0, 0, 1], 3000)));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn load_config(path: Option<&Path>) -> eyre::Result<config::Config> {
    let file = fs::read_to_string(path.unwrap_or(Path::new("config.toml"))).await?;
    Ok(toml::from_str(file.as_str())?)
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        println!("`{}` connected", user_agent.as_str());
    }

    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(t) => {
                    println!("client sent str: {:?}", t);
                }
                Message::Binary(_) => {
                    return; // binary is useless for neos, we reject these connections
                }
                Message::Ping(_) => {
                    println!("socket ping");
                }
                Message::Pong(_) => {
                    println!("socket pong");
                }
                Message::Close(_) => {
                    println!("client disconnected");
                    return;
                }
            }
        } else {
            println!("client disconnected");
            return;
        }
    }

    loop {
        if socket
            .send(Message::Text(String::from("Hi!")))
            .await
            .is_err()
        {
            println!("client disconnected");
            return;
        }
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}
