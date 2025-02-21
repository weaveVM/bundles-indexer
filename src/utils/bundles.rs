use anyhow::Error;
use bundler::utils::core::{bundle::Bundle, tx_envelope_writer::TxEnvelopeWrapper};

pub async fn get_envelopes(txid: &str) -> Result<Vec<TxEnvelopeWrapper>, Error> {
    let bundle = Bundle::retrieve_envelopes(txid.to_string()).await?;
    Ok(bundle.envelopes)
}
