use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub address: String,
    pub signature: String,
    pub nonce: String,
}