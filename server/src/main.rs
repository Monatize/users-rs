use axum::{
    extract::Extension,
    http::{HeaderValue, Method},
    routing::{get, post},
    Router,
};
use std::{net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;
use tower_http::cors::{CorsLayer, Any};
pub mod routes;
pub mod types;
pub mod utils;
use routes::{authentication::authentication, entry::entry, handler::handler};
pub mod prisma;
use dotenv::dotenv;
use prisma::PrismaClient;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // * Create New Prisma-Client-Rust session
    let client: Arc<PrismaClient> = Arc::new(prisma::new_client().await.unwrap());

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/entry", post(entry))
        .route("/authorization", post(authentication))
        .layer(
            ServiceBuilder::new().layer(Extension(client)).layer(
                CorsLayer::very_permissive()
            ),
        );

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
