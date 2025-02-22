use axum::Json;
use serde_json::{json, Value};

pub async fn get_root() -> Json<Value> {
    Json(json!({"status": "running"}))
}
