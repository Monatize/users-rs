use serde::{Serialize, Deserialize};

use crate::types::shared::status::StatusCodes;

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub verified: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthError {
    pub status: StatusCodes,
    pub data: Data,
    pub message: String
}
