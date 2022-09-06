use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum StatusCodes {
    Success,
    Unauthorized,
    ServerError,
}