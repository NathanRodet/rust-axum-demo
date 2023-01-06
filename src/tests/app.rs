use axum::{routing::get, Router};
use dotenvy_macro::dotenv;
use sea_orm::Database;

use crate::{models::app_state::AppState, routes::index::hello_world};

pub async fn app_test() -> Router {
    let database_uri = dotenv!("DATABASE_URL").to_owned();
    let database_conn = Database::connect(database_uri).await.unwrap();
    let app_state = AppState { database_conn };

    let guest_nest = Router::new().route("/", get(hello_world));

    Router::new().nest("", guest_nest).with_state(app_state)
}
