use rocket::catch;
use serde_json::{json, Value};

#[catch(404)]
pub fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[catch(500)]
pub fn internal_error() -> Value {
    json!({
        "status": "error",
        "reason": "Internal Server Error."
    })
}
