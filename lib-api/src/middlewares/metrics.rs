use axum::extract::{MatchedPath, Request, State};
use axum::middleware::Next;
use axum::response::IntoResponse;
use lib_core::app_state::AppState;
use opentelemetry::KeyValue;
use std::convert::Infallible;
use tokio::time::Instant;

// Middleware to collect metrics
pub async fn metrics_middleware(
    state: State<AppState>,
    matched_path: Option<MatchedPath>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, Infallible> {
    let start = Instant::now();
    state.metrics.active_requests.add(1, &[]);

    let path = get_route_path(&req, matched_path);

    let response = next.run(req).await;

    let elapsed = start.elapsed().as_secs_f64();
    state.metrics.active_requests.add(-1, &[]);

    let status_code = response.status().as_u16();

    state.metrics.request_count.add(
        1,
        &[
            KeyValue::new("path", path.clone()),
            KeyValue::new("status_code", status_code.to_string()),
        ],
    );
    state.metrics.request_duration.record(
        elapsed,
        &[
            KeyValue::new("path", path.clone()),
            KeyValue::new("status_code", status_code.to_string()),
        ],
    );

    if status_code >= 500 {
        state.metrics.errors_count.add(
            1,
            &[
                KeyValue::new("path", path),
                KeyValue::new("status_code", status_code.to_string()),
            ],
        );
    }

    Ok(response.into_response())
}

// Utility function to get the path, accounting for route prefixes
fn get_route_path(req: &Request, matched_path: Option<MatchedPath>) -> String {
    if let Some(matched_path) = matched_path {
        matched_path.as_str().to_owned()
    } else {
        // Fallback:  This could include the router prefix.
        req.uri().path().to_owned()
    }
}
