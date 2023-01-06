use axum::http::StatusCode;

use crate::models::index::TeapotResponse;

pub async fn hello_world() -> Result<String, (StatusCode, String)> {
    todo!("Create a get endpoint to return 'Hello World!' message.")
}

pub async fn teapot() -> Result<TeapotResponse, (StatusCode, String)> {
    todo!("Create a get endpoint to return a TeapotResponse.")
}
