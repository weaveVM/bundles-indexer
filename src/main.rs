pub mod indexer;
pub mod utils;

use crate::utils::constants::BUNDLES_START_BLOCK;
use crate::utils::rpc::{get_block, init_wvm_rpc, detect_bundles};
use crate::indexer::cronjob::index;

#[tokio::main]
async fn main() {
    let provider = init_wvm_rpc().await.unwrap();
    loop {
        index(provider.clone()).await.unwrap();
    }
}
