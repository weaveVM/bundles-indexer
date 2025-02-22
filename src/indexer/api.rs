use crate::indexer::cloud::get_bundle_by_envelope;
use crate::utils::bundles::get_envelope_from_bundle;
use axum::response::IntoResponse;
use axum::{extract::Path, Json};
use reqwest::{header, StatusCode};
use serde_json::{json, Value};

pub async fn get_root() -> Json<Value> {
    Json(json!({"status": "running"}))
}

pub async fn get_envelope_raw(Path(envelope_txid): Path<String>) -> Json<Value> {
    let bundle_txid = get_bundle_by_envelope(&envelope_txid)
        .await
        .unwrap_or_default();
    let envelope = get_envelope_from_bundle(&bundle_txid, &envelope_txid)
        .await
        .unwrap();
    Json(serde_json::to_value(&envelope).unwrap())
}

pub async fn resolve_envelope(Path(envelope_txid): Path<String>) -> impl IntoResponse {
    let bundle_txid = get_bundle_by_envelope(&envelope_txid)
        .await
        .unwrap_or_default();

    if bundle_txid.len() == 0 {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "envelope index out of range"})),
        )
            .into_response();
    }

    let envelope = get_envelope_from_bundle(&bundle_txid, &envelope_txid)
        .await
        .unwrap();

    let input: String = match Some(envelope.clone().input) {
        Some(input) => input,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "No input data found in envelope" })),
            )
                .into_response()
        }
    };

    let content_type = envelope
        .tags
        .clone()
        .map(|tags| {
            tags.iter()
                .find(|tag| tag.name.to_lowercase() == "content-type")
                .map(|tag| tag.value.clone())
        })
        .flatten()
        .unwrap_or_else(|| "application/octet-stream".to_string());

    let processed_data = match hex::decode(input.trim_start_matches("0x")) {
        Ok(data) => data,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Failed to process input data" })),
            )
                .into_response()
        }
    };

    (
        [
            (header::CONTENT_TYPE, content_type),
            (
                header::CACHE_CONTROL,
                "public, max-age=31536000".to_string(),
            ),
        ],
        processed_data,
    )
        .into_response()
}
