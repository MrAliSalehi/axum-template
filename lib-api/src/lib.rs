use axum::Router;
use axum::http::StatusCode;
use axum::middleware::from_fn_with_state;
use axum_client_ip::ClientIpSource;
use axum_helmet::{Helmet, HelmetLayer};
use bytes::Bytes;
use http::{Response, header};
use http_body_util::Full;
use lib_core::app_state::AppState;
use lib_shared::{Res, instrument};
use std::any::Any;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;
use tower::{BoxError, ServiceBuilder};
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse};
use tracing::{Level, error, info};

pub mod components;
pub mod middlewares;
pub mod models;
pub mod utils;

pub async fn run() -> Res {
    let app_state = AppState::new().await;

    let req_tracing = tower_http::trace::TraceLayer::new_for_http()
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));
    let app = Router::new()
        //.merge components
        .merge(components::routes(app_state.clone()))
        .layer(
            ServiceBuilder::new() //executes from top to bottom
                .layer(req_tracing)
                .layer(axum::error_handling::HandleErrorLayer::new(unhandled_err))
                .layer(tower_http::catch_panic::CatchPanicLayer::custom(
                    handle_panic,
                ))
                .layer(tower_http::timeout::TimeoutLayer::new(Duration::from_secs(
                    10,
                )))
                .layer(HelmetLayer::new(build_helmet()))
                .layer(cors())
                .layer(ClientIpSource::ConnectInfo.into_extension())
                .layer(tower::buffer::BufferLayer::new(1024)),
        )
        .layer(from_fn_with_state(
            app_state.clone(),
            middlewares::metrics::metrics_middleware,
        ));

    info!(port = app_state.env.api_port, "api running");

    axum::serve(
        TcpListener::bind(format!("0.0.0.0:{}", app_state.env.api_port)).await?,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}

fn build_helmet() -> Helmet {
    Helmet::new()
        .add(axum_helmet::XContentTypeOptions::nosniff())
        .add(axum_helmet::ReferrerPolicy::StrictOriginWhenCrossOrigin)
        .add(axum_helmet::XFrameOptions::SameOrigin) // deprecated
        .add(axum_helmet::XDownloadOptions::NoOpen)
        .add(axum_helmet::CrossOriginEmbedderPolicy::RequireCorp)
        .add(
            axum_helmet::ContentSecurityPolicy::new()
                //disable iframe or cross-content
                .frame_src(vec!["'none'"])
                .frame_ancestors(vec!["'none'"])
                .default_src(vec!["'self'"]),
        )
}

#[instrument(skip(e))]
async fn unhandled_err(e: BoxError) -> (StatusCode, String) {
    error!(error = e, "unhandled internal error");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Untracked Internal Error".to_owned(),
    )
}
#[instrument]
fn handle_panic(_: Box<dyn Any + Send + 'static>) -> Response<Full<Bytes>> {
    error!("unknown panic traced");
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Full::from("Internal Panic"))
        .unwrap()
}

#[inline]
fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_headers(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_origin(tower_http::cors::Any)
        .allow_private_network(true)
}
