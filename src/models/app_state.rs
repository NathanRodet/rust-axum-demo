use axum_macros::FromRef;
use sea_orm::DatabaseConnection;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database_conn: DatabaseConnection,
}
