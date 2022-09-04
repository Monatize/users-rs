use axum::response::Json;
use serde_json::{ Value, json };
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct User {
    address: String,
    signature: String,
    nonce: String,
}

pub async fn entry(
    Json(payload): Json<User>
) -> Json<Value> {
    Json(json!({
        "address": payload.address,
        "signature": payload.signature,
        "nonce": payload.nonce
    }))
}
