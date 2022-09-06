use axum::{
    routing::{get, post},
    Router,
    extract::Extension
};
use std::{net::SocketAddr, sync::Arc};
pub mod routes;
pub mod utils;
pub mod types;
use routes::{entry::entry, handler::handler};
pub mod prisma;
use prisma::{PrismaClient};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // * Create New Prisma-Client-Rust session
    let client: Arc<PrismaClient> = Arc::new(prisma::new_client().await.unwrap());

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/entry", post(entry))
        .layer(Extension(client));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}