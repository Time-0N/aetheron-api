use crate::dto::hello_response::HelloResponse;

#[derive(Clone)]
pub struct HelloService {}

impl HelloService {
    pub fn new() -> Self {
        Self {}
    }
    pub fn get_hello_message() -> HelloResponse {
        HelloResponse {
            message: "Hello World".to_string(),
        }
    }
}