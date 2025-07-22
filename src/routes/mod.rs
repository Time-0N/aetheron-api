mod hello_route;

use axum::Router;
use crate::app_state::AppState;
use crate::routes::hello_route::hello_route;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/api/hello", hello_route())
}