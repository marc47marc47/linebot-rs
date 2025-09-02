use axum::{
    Router,
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::post,
};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

use crate::utils::verify_signature;
use crate::{Config, LineApiClient};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub line_client: LineApiClient,
}

pub fn create_app(config: Config) -> Router {
    let line_client = LineApiClient::new(config.channel_access_token.clone());

    let state = Arc::new(AppState {
        config: config.clone(),
        line_client,
    });

    Router::new()
        .route("/webhook", post(crate::webhook::handlers::handle_webhook))
        .route("/health", axum::routing::get(health_check))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    signature_middleware,
                )),
        )
        .with_state(state)
}

pub async fn start_server(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let app = create_app(config.clone());

    let bind_address = format!("{}:{}", config.host, config.port);
    info!("Starting server on {}", bind_address);

    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

async fn signature_middleware(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Response {
    // Skip signature verification for health check
    if request.uri().path() == "/health" {
        return next.run(request).await;
    }

    let signature = match headers.get("x-line-signature") {
        Some(sig) => match sig.to_str() {
            Ok(s) => s,
            Err(_) => {
                return (StatusCode::BAD_REQUEST, "Invalid signature header").into_response();
            }
        },
        None => {
            return (StatusCode::BAD_REQUEST, "Missing signature header").into_response();
        }
    };

    let (parts, body) = request.into_parts();
    let body_bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, "Failed to read body").into_response();
        }
    };

    if !verify_signature(&state.config.channel_secret, &body_bytes, signature) {
        return (StatusCode::UNAUTHORIZED, "Invalid signature").into_response();
    }

    // Reconstruct the request with the body
    let new_request = Request::from_parts(parts, axum::body::Body::from(body_bytes));

    next.run(new_request).await
}
