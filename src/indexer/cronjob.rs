

use crate::utils::constants::BUNDLES_START_BLOCK;
use crate::utils::bundles::get_envelopes;
use crate::utils::rpc::{get_block, init_wvm_rpc, detect_bundles, JsonRpc};
use crate::indexer::cloud::{get_latest_block_id, insert_bundle_entry};
use tokio::time::{sleep, Duration};
use anyhow::Error;


pub async fn index(client: JsonRpc) -> Result<(), Error> {
    let block_nr = get_latest_block_id().await.map_or(BUNDLES_START_BLOCK, |nr| nr + 1);

    let block_bundles = detect_bundles(block_nr, client).await.unwrap();

    for bundle in block_bundles {
        let envelopes = get_envelopes(&bundle).await?;

        for e in envelopes {
            insert_bundle_entry(block_nr, &bundle, &e).await?;
        }
    }

    println!("INDEXED BLOCK #{} -- SLEEPING FOR 1S", block_nr);

    sleep(Duration::from_secs(1)).await;
    Ok(())
}
