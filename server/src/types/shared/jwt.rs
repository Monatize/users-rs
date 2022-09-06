use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub address: String,
    pub signature: String,
    pub nonce: String,
    pub exp: usize
}