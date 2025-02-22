pub mod indexer;
pub mod utils;

use crate::indexer::api::{get_envelope_raw, get_root};
use crate::indexer::cronjob::index;
use crate::utils::rpc::init_wvm_rpc;

use axum::{routing::get, Router};
use tokio::task;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    // load secrets from Secrets.toml into env var;
    secrets.into_iter().for_each(|(key, val)| {
        println!("{:?} {:?}", key, val);
        std::env::set_var(key, val);
    });

    let provider = init_wvm_rpc().await.unwrap();

    // server routes
    let router = Router::new()
        .route("/", get(get_root))
        .route("/envelope/:txid", get(get_envelope_raw));

    // task::spawn(async move {
    //     loop {
    //         index(provider.clone()).await.unwrap();
    //     }
    // });

    Ok(router.into())
}
