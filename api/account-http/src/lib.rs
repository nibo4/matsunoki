use axum::routing::get;
use axum::Router;
use kernel::Kernel;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::trace::TraceLayer;

pub mod handler;
pub mod kernel;
pub mod middleware;

pub fn router(kernel: Kernel) -> Router {
    Router::new()
        .route("/hc", get(handler::health_check::health_check_handler))
        .layer(AddExtensionLayer::new(kernel))
        .layer(TraceLayer::new_for_http())
}
