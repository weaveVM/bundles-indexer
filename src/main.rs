pub mod indexer;
pub mod utils;

use crate::indexer::api::{
    get_bundles_of_block, get_envelope_raw, get_envelopes_of_block, get_envelopes_of_bundle,
    get_root, get_stats, resolve_envelope,
};
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
        .route("/v1", get(get_root))
        .route("/v1/envelope/:envelope_txid", get(get_envelope_raw))
        .route("/v1/envelopes/:bundle_txid", get(get_envelopes_of_bundle))
        .route("/v1/resolve/:envelope_txid", get(resolve_envelope))
        .route("/v1/block/bundles/:block_nr", get(get_bundles_of_block))
        .route("/v1/block/envelopes/:block_nr", get(get_envelopes_of_block))
        .route("/v1/stats", get(get_stats));

    tokio::task::spawn(async move {
        loop {
            match index(provider.clone()).await {
                Ok(_) => println!("\nIndexer batch completed successfully\n"),
                Err(e) => {
                    println!("\nIndexer error: {:?}, restarting...", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        }
    });

    Ok(router.into())
}
