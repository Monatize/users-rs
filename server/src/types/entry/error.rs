use serde::{Deserialize, Serialize};
use crate::types::shared::status::StatusCodes;

#[derive(Debug, Deserialize, Serialize)]
pub struct EntryError {
    pub status: StatusCodes,
    pub data: Data,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub token: Option<String> // * Return nothing, just standardizing structs 
}