use crate::utils::constants::WVM_RPC_URL;
use alloy::primitives::{BlockNumber, Address};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::{BlockTransactions, Transaction as BlockTx};
use alloy::consensus::{Typed2718, Transaction};
use crate::utils::constants::BUNDLES_BABE1_ADDRESS;
use std::str::FromStr;
use anyhow::Error;

pub type JsonRpc = alloy::providers::fillers::FillProvider<
    alloy::providers::fillers::JoinFill<
        alloy::providers::Identity,
        alloy::providers::fillers::JoinFill<
            alloy::providers::fillers::GasFiller,
            alloy::providers::fillers::JoinFill<
                alloy::providers::fillers::BlobGasFiller,
                alloy::providers::fillers::JoinFill<
                    alloy::providers::fillers::NonceFiller,
                    alloy::providers::fillers::ChainIdFiller,
                >,
            >,
        >,
    >,
    alloy::providers::RootProvider,
>;

pub async fn init_wvm_rpc() -> Result<JsonRpc, Error> {
    let url = WVM_RPC_URL.parse()?;
    let provider: JsonRpc = ProviderBuilder::new().on_http(url);
    Ok(provider)
}

pub async fn get_latest_block_nr(provider: JsonRpc) -> Result<u64, Error> {
     Ok(provider.get_block_number().await?)
}

pub async fn get_block(
    block_nr: u32,
    provider: JsonRpc,
) -> Result<BlockTransactions<BlockTx>, Error> {
    let nr = BlockNumber::from(block_nr);
    let block = provider
        .get_block_by_number(nr.into(), alloy::rpc::types::BlockTransactionsKind::Full)
        .await?
        .unwrap_or_default();
    Ok(block.transactions)
}

pub async fn detect_bundles(block_nr: u32, provider: JsonRpc) -> Result<Vec<String>, Error> {
    let block = get_block(block_nr, provider).await?;
    let mut bundles_txid: Vec<String> = Vec::new();
    let bundles_0xbabe1_addr = Some(Address::from_str(BUNDLES_BABE1_ADDRESS)?);
    for tx in block.into_transactions_vec() {
        if tx.is_eip1559() && tx.to() == bundles_0xbabe1_addr {
            let tx_hash = tx.inner.tx_hash().to_string();
            bundles_txid.push(tx_hash);
        }
    }

    Ok(bundles_txid)
}
