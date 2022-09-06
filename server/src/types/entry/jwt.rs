use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct JWT {
    pub address: String,
    pub signature: String,
    pub nonce: String,
    pub exp: usize
}