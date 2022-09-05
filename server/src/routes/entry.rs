use axum::{response::Json, Extension};
use serde_json::{ Value, json };
use serde::{Deserialize};

use crate::prisma;
use crate::utils;
type Database = Extension<std::sync::Arc<prisma::PrismaClient>>;

#[derive(Deserialize)]
pub struct User {
    address: String,
    signature: String,
    nonce: String,
}

pub async fn entry(
    Json(payload): Json<User>,
    _ctx: Database
) -> Json<Value> {   
    let [address, signature, nonce] = [payload.address, payload.signature, payload.nonce];

    if address.trim() == "" || signature.trim() == "" || nonce.trim() == "" {
        return Json(json!({
            "success": false,
            "code": 400,
            "error": "Please include all fields!"
        }))
    }

    let pubkey = utils::get_pubkey::get_pubkey(address, nonce, signature);

    match pubkey {
        Ok(pubkey) => {
            println!("{}", pubkey)
        }
        Err(e) => {
            println!("{}", e)
        }
    }

    Json(json!({
        "test": "testing"
    }))
}
