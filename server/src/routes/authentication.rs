// Rust Libs
use std::env;
// Rust Libs


// 3rd Party Libs
use axum::{Extension, http::{StatusCode, HeaderMap}, response::IntoResponse, Json};
use jsonwebtoken::{decode, Validation, DecodingKey};
use dotenv::dotenv;
// use serde_json::json;
 // * Bring in prisma connection from main.rs, and return JSON responses/status
use crate::{prisma, types::{entry::jwt::Claims, authentication::user::User}}; // * Query Models & bring in prisma connection from main.rs
// 3rd Party Libs

// Local Imports
// Local Imports

// Types
type Database = Extension<std::sync::Arc<prisma::PrismaClient>>; // * Import prisma connection from main.rs
// Types

pub async fn authentication(
    _ctx: Database,
    Json(payload): Json<User>,
    headers: HeaderMap
) -> impl IntoResponse {
    let auth_header = headers.get("authorization");
    dotenv().ok();
    match auth_header {
        Some(auth_header) => {
            let auth_header = auth_header.to_str().unwrap();
            let signing_key = env::var("JWT_PRIVATE_KEY").unwrap();
            let token_data = decode::<Claims>(&auth_header, &DecodingKey::from_secret(signing_key.as_ref()), &Validation::default());
            match token_data {
                Ok(data) => {
                    if payload.address.to_lowercase() == data.claims.address.to_lowercase() {
                        
                    }
                    else {

                    }
                }
                Err(e) => {
                    println!("{}", e);
                }
            }

        }
        None => {
            
        }
    }
    (StatusCode::OK, "hi")
}