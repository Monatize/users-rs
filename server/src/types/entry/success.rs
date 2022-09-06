use serde::{Deserialize, Serialize};
use crate::types::entry::status::StatusCodes;

#[derive(Debug, Deserialize, Serialize)]
pub struct EntrySuccess {
    pub status: StatusCodes,
    pub data: SuccessData,
    pub message: String, 
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SuccessData {
    pub token: String
}