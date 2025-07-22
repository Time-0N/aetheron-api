use axum::{Json};
use crate::{dto, services};

pub async fn hello_world() -> Json<dto::hello_response::HelloResponse> {
    Json(services::hello_service::HelloService::get_hello_message())
}