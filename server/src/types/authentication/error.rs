use serde::Deserialize;

use crate::types::shared::status::StatusCodes;

#[derive(Deserialize)]
pub struct Data {
    pub verified: Option<bool>
}

pub struct AuthError {
    pub status: StatusCodes
}
