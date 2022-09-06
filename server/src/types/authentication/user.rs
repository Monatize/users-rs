use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    pub address: String
}