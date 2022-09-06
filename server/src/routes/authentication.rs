// Rust Libs
// Rust Libs


// 3rd Party Libs
use axum::{Extension, http::{StatusCode, HeaderMap}, response::IntoResponse, Json, headers::{authorization::Bearer, Authorization}, TypedHeader, extract::{RequestParts, FromRequest}};
use serde_json::json; // * Bring in prisma connection from main.rs, and return JSON responses/status
use crate::prisma; // * Query Models & bring in prisma connection from main.rs
// 3rd Party Libs

// Local Imports
// Local Imports

// Types
type Database = Extension<std::sync::Arc<prisma::PrismaClient>>; // * Import prisma connection from main.rs
// Types

pub async fn authentication(
    _ctx: Database,
    headers: HeaderMap
) -> impl IntoResponse {
    let auth_header = headers.get("authorization");
    match auth_header {
        Some(auth_header) => {
            println!("{:?}", auth_header);
        }
        None => {
        }
    }
    (StatusCode::OK, "hi")
}