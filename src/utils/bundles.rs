use anyhow::Error;
use bundler::utils::core::{bundle::Bundle, tx_envelope_writer::TxEnvelopeWrapper};

use crate::utils::rpc::get_block;

pub async fn get_envelopes(txid: &str) -> Result<Vec<String>, Error> {
    let bundle = Bundle::retrieve_envelopes(txid.to_string()).await?;
    let envelopes: Vec<String> = bundle.envelopes.iter().map(|envelope| envelope.clone().hash).collect();
    Ok(envelopes)
}