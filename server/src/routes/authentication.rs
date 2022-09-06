// Rust Libs
use std::env;
// Rust Libs


// 3rd Party Libs
use axum::{Extension, http::{StatusCode, HeaderMap}, response::IntoResponse, Json};
use jsonwebtoken::{decode, Validation, DecodingKey};
use dotenv::dotenv;
use serde_json::json;
 // * Bring in prisma connection from main.rs, and return JSON responses/status
use crate::{prisma, types::{shared::{jwt::Claims, status::StatusCodes}, authentication::{user::User, success::{AuthSuccess, SuccessData}, error::{AuthError, Data}}}}; // * Query Models & bring in prisma connection from main.rs
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
                        let success_data = SuccessData {
                            verified: true
                        };

                        let success_struct = AuthSuccess {
                            status: StatusCodes::Success,
                            data: success_data,
                            message: String::from("Authorized")
                        };

                        (StatusCode::OK, Json(json!(success_struct)))
                    }
                    else {
                        let error_data = Data {
                            verified: false
                        };
    
                        let error_struct = AuthError {
                            status: StatusCodes::Unauthorized,
                            data: error_data,
                            message: String::from("Unauthorized")
                        };

                        (StatusCode::UNAUTHORIZED, Json(json!(error_struct)))
                    }
                }
                Err(_) => {
                    let error_data = Data {
                        verified: false
                    };

                    let error_struct = AuthError {
                        status: StatusCodes::Unauthorized,
                        data: error_data,
                        message: String::from("Unauthorized")
                    };

                    (StatusCode::UNAUTHORIZED, Json(json!(error_struct)))
                }
            }

        }
        None => {
            let error_data = Data {
                verified: false
            };

            let error_struct = AuthError {
                status: StatusCodes::Unauthorized,
                data: error_data,
                message: String::from("Unauthorized")
            };

            (StatusCode::UNAUTHORIZED, Json(json!(error_struct))) 
        }
    }
}

