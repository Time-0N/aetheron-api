use std::sync::Arc;
use crate::services::hello_service::HelloService;

#[derive(Clone)]
pub struct AppState {
    pub hello_service: Arc<HelloService>
}

impl AppState {
    pub fn new() -> Self {
        let hello_service = Arc::new(HelloService::new());

        AppState {
            hello_service,
        }
    }
}