use crate::indexer::cloud::{
    get_latest_block_id, insert_bundle_entry, update_last_processed_block,
};
use crate::utils::bundles::get_envelopes;
use crate::utils::constants::BUNDLES_START_BLOCK;
use crate::utils::rpc::{detect_bundles, get_block, init_wvm_rpc, JsonRpc};
use anyhow::Error;
use tokio::time::{sleep, Duration};

pub async fn index(client: JsonRpc) -> Result<(), Error> {
    loop {
        let block_nr = get_latest_block_id()
            .await
            .map_or(BUNDLES_START_BLOCK, |nr| nr + 1);

        match detect_bundles(block_nr, client.clone()).await {
            Ok(block_bundles) => {
                if !block_bundles.is_empty() {
                    for bundle in block_bundles {
                        let envelopes = get_envelopes(&bundle).await?;
                        for e in envelopes {
                            insert_bundle_entry(block_nr, &bundle, &e).await?;
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error processing block {}: {:?}", block_nr, e);
            }
        }

        // Always update last processed block, regardless of success or error
        update_last_processed_block(block_nr).await?;

        println!("INDEXED BLOCK #{} -- SLEEPING FOR 1s", block_nr);
        // sleep(Duration::from_secs(1)).await;
    }
}
