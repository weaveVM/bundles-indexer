pub mod indexer;
pub mod utils;

use crate::utils::constants::BUNDLES_START_BLOCK;
use crate::utils::rpc::{get_block, init_wvm_rpc};

#[tokio::main]
async fn main() {
    let client = init_wvm_rpc().await.unwrap();
    let block = get_block(BUNDLES_START_BLOCK, client).await.unwrap();
    println!("{:#?}", block);
}
