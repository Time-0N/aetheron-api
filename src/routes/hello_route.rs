use axum::Router;
use axum::routing::get;
use crate::app_state::AppState;
use crate::handlers::hello_handler::hello_world;

pub fn hello_route() -> Router<AppState> {
    Router::new()
        .route("/world", get(hello_world))
}