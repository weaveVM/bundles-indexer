use crate::indexer::cloud::{
    get_latest_block_id, insert_bundle_entry, update_last_processed_block,
};
use crate::utils::bundles::get_envelopes;
use crate::utils::constants::BUNDLES_START_BLOCK;
use crate::utils::rpc::{detect_bundles, get_latest_block_nr, JsonRpc};
use anyhow::Error;
use futures::{stream, StreamExt};
use tokio::spawn;
use tokio::time::{sleep, Duration};

// pub async fn index(client: JsonRpc) -> Result<(), Error> {
//     loop {
//         let block_nr = get_latest_block_id().await.map_or(BUNDLES_START_BLOCK, |nr| nr + 1);

//         match detect_bundles(block_nr, client.clone()).await {
//             Ok(block_bundles) => {
//                 if !block_bundles.is_empty() {
//                     for bundle in block_bundles {
//                         let envelopes = get_envelopes(&bundle).await?;
//                         for e in envelopes {
//                             insert_bundle_entry(block_nr, &bundle, &e).await?;
//                         }
//                     }
//                 }
//             }
//             Err(e) => {
//                 println!("Error processing block {}: {:?}", block_nr, e);
//             }
//         }

//         update_last_processed_block(block_nr).await?;

//         println!("INDEXED BLOCK #{} -- SLEEPING FOR 1s", block_nr);
//         // sleep(Duration::from_secs(1)).await;
//     }
// }

pub async fn index(client: JsonRpc) -> Result<(), Error> {
        let mut wvm_last_rpc_block = get_latest_block_nr(client.clone()).await? as u32;
        let start_block = get_latest_block_id()
            .await
            .map_or(BUNDLES_START_BLOCK, |nr| nr + 1);
        let mut end_block = start_block + 3600;

        if end_block > wvm_last_rpc_block {
            end_block = wvm_last_rpc_block
        };

        let blocks = start_block..end_block;

        // process blocks in parallel with max 100 concurrent tasks
        stream::iter(blocks)
            .map(|block_nr| {
                let client = client.clone();
                spawn(async move {
                    match detect_bundles(block_nr, client).await {
                        Ok(block_bundles) => {
                            if !block_bundles.is_empty() {
                                for bundle in block_bundles {
                                    let envelopes = get_envelopes(&bundle).await?;
                                    for e in envelopes {
                                        insert_bundle_entry(block_nr, &bundle, &e).await?;
                                    }
                                }
                            }
                            update_last_processed_block(block_nr).await?;
                            println!("INDEXED BLOCK #{}", block_nr);
                            Ok::<(), Error>(())
                        }
                        Err(e) => {
                            println!("Error processing block {}: {:?}", block_nr, e);
                            update_last_processed_block(block_nr).await?;
                            Ok(())
                        }
                    }
                })
            })
            .buffer_unordered(100) // process 100 blocks concurrently
            .collect::<Vec<_>>()
            .await;

        println!("Completed batch from {} to {}", start_block, end_block - 1);
        sleep(Duration::from_secs(1)).await;
        Ok(())
}
