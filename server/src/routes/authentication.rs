// Rust Libs
// Rust Libs


// 3rd Party Libs
use axum::http::StatusCode;
use axum::response::IntoResponse;
// 3rd Party Libs

// Local Imports
// Local Imports

// Types
// Types

pub async fn authentication() -> impl IntoResponse {
    (StatusCode::OK, "hi")
}