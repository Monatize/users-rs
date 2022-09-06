// * Rust Libs
use std::{net::SocketAddr, sync::Arc};
// * Rust Libs

// * 3rd Party Libs
use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use prisma::PrismaClient;
pub mod prisma;
use dotenv::dotenv;
// * 3rd Party Libs

// * Local Imports
pub mod routes;
pub mod types;
pub mod utils;
use routes::{authentication::authentication, entry::entry, handler::handler};
// * Local Imports





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
