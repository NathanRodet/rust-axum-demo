use crate::models::app_state::AppState;
use crate::routes::index::hello_world;
use axum::routing::get;
use axum::Router;

pub async fn create_routes(app_state: AppState) -> Router {
    let index_nest = Router::new().route("/", get(hello_world));

    // You must implement a route for each of the following endpoints:
    // - POST /tasks
    // - GET /tasks
    // - GET /tasks/:id
    // - PUT /tasks/:id
    // - DELETE /tasks/:id
    // You can also nest routes in a router, like the index_nest above.

    Router::new().nest("", index_nest).with_state(app_state)
}
