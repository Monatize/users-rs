use serde::Deserialize;

use crate::types::shared::status::StatusCodes;

#[derive(Deserialize)]
pub struct Data {
    pub verified: bool
}

#[derive(Deserialize)]
pub struct AuthError {
    pub status: StatusCodes,
    pub data: Data,
    pub message: String
}
