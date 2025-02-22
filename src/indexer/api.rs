use crate::indexer::cloud::get_bundle_by_envelope;
use crate::utils::bundles::get_envelope_from_bundle;
use axum::{extract::Path, Json};
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
