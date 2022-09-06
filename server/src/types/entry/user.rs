use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    pub address: String,
    pub signature: String,
    pub nonce: String,
}