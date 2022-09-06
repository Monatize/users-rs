use serde::{Deserialize, Serialize};
use crate::types::entry::status::StatusCodes;

#[derive(Debug, Deserialize, Serialize)]
pub struct EntrySuccess {
    status: StatusCodes,
    data: Data,
    message: String, 
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    token: String
}