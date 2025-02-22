pub mod indexer;
pub mod utils;

use crate::indexer::api::{get_envelope_raw, get_root, resolve_envelope, get_envelopes_of_bundle, get_bundles_of_block, get_envelopes_of_block};
use crate::indexer::cronjob::index;
use crate::utils::rpc::init_wvm_rpc;

use axum::{routing::get, Router};

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    // load secrets from Secrets.toml into env var;
    secrets.into_iter().for_each(|(key, val)| {
        std::env::set_var(key, val);
    });

    let provider = init_wvm_rpc().await.unwrap();

    // server routes
    let router = Router::new()
        .route("/", get(get_root))
        .route("/envelope/:envelope_txid", get(get_envelope_raw))
        .route("/envelopes/:bundle_txid", get(get_envelopes_of_bundle))
        .route("/resolve/:envelope_txid", get(resolve_envelope))
        .route("/block/bundles/:block_nr", get(get_bundles_of_block))
        .route("/block/envelopes/:block_nr", get(get_envelopes_of_block));

    tokio::task::spawn(async move {
        loop {
            index(provider.clone()).await.unwrap();
        }
    });

    Ok(router.into())
}
