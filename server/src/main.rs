use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

pub mod routes;
use routes::{entry::entry, handler::handler};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/entry", post(entry));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
