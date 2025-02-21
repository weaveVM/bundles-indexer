use crate::utils::constants::WVM_RPC_URL;
use alloy::primitives::BlockNumber;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::{BlockTransactions, Transaction};
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

pub async fn get_block(
    block_nr: u32,
    provider: JsonRpc,
) -> Result<BlockTransactions<Transaction>, Error> {
    let nr = BlockNumber::from(block_nr);
    let block = provider
        .get_block_by_number(nr.into(), alloy::rpc::types::BlockTransactionsKind::Full)
        .await?
        .unwrap_or_default();
    Ok(block.transactions)
}
