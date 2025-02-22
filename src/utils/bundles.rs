use anyhow::Error;
use bundler::utils::core::{bundle::Bundle, tx_envelope_writer::TxEnvelopeWrapper};

pub async fn get_envelopes(txid: &str) -> Result<Vec<String>, Error> {
    let bundle = Bundle::retrieve_envelopes(txid.to_string()).await?;
    let envelopes: Vec<String> = bundle
        .envelopes
        .iter()
        .map(|envelope| envelope.clone().hash)
        .collect();
    Ok(envelopes)
}

pub async fn get_envelope_from_bundle(
    bundle_txid: &str,
    envelope_txid: &str,
) -> Result<TxEnvelopeWrapper, Error> {
    let bundle = Bundle::retrieve_envelopes(bundle_txid.to_string()).await?;
    let envelope = bundle
        .envelopes
        .iter()
        .find(|envelope| envelope.hash == envelope_txid)
        .ok_or(Error::msg("error finding envelope"))?;
    Ok(envelope.clone())
}
