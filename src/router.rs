use crate::models::app_state::AppState;
use crate::routes::index::hello_world;
use axum::routing::get;
use axum::Router;

pub async fn create_routes(app_state: AppState) -> Router {
    let index_nest = Router::new().route("/", get(hello_world));

    // You must implement a nest for each of the following routes and endpoints:
    // - POST /tasks
    // - GET /tasks
    // - GET /tasks/:id
    // - PUT /tasks/:id
    // - DELETE /tasks/:id

    Router::new().nest("", index_nest).with_state(app_state)
}
