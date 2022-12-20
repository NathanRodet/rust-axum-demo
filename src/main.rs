use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use rust_demo::server::run;

#[tokio::main]
async fn main() {
    // Load the environment variables
    dotenv().ok();
    let database_uri = dotenv!("DATABASE_URL");

    // Run the server
    run(database_uri).await;
}
