use axum::routing::get;
use axum::Router;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};

pub mod handler;
pub mod middleware;

pub fn router() -> Router {
    Router::new()
        .route("/hc", get(handler::health_check::health_check_handler))
        .layer(TraceLayer::new_for_http())
}
