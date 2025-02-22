use crate::utils::env_vars::get_env_var;
use anyhow::Error;
use planetscale_driver::{query, Database, PSConnection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Database)]
pub struct BundleIndexer {
    pub id: i32,
    pub block_id: u32,
    pub bundle_txid: String,
    pub envelope_txid: String,
}

#[derive(Debug, Serialize, Deserialize, Database)]
pub struct BundleTxId {
    pub bundle_txid: String,
}

#[derive(Debug, Serialize, Deserialize, Database)]
pub struct EnvelopeTxId {
    pub envelope_txid: String,
}

#[derive(Deserialize, Database)]
struct MaxBlock {
    max_block: u32,
}

async fn ps_client() -> Result<PSConnection, Error> {
    let host = get_env_var("DATABASE_HOST")?;
    let username = get_env_var("DATABASE_USERNAME")?;
    let password = get_env_var("DATABASE_PASSWORD")?;

    let conn: PSConnection = PSConnection::new(&host, &username, &password);
    Ok(conn)
}

pub async fn insert_bundle_entry(
    block_id: u32,
    bundle_txid: &str,
    envelope_txid: &str,
) -> Result<(), Error> {
    let conn = ps_client().await?;
    let query_str = format!(
       "INSERT INTO bundles_indexer(block_id, bundle_txid, envelope_txid) VALUES({}, \"{}\", \"{}\")",
       block_id, bundle_txid, envelope_txid
   );
    let res = query(&query_str).execute(&conn).await?;
    println!("Insert operation successful: {:?}", res);
    Ok(())
}

pub async fn get_bundle_by_envelope(envelope_txid: &str) -> Result<String, Error> {
    let conn = ps_client().await?;
    let query_str = format!(
        "SELECT bundle_txid FROM bundles_indexer WHERE envelope_txid = \"{}\"",
        envelope_txid
    );
    let result: BundleTxId = query(&query_str).fetch_one(&conn).await?;
    Ok(result.bundle_txid)
}

pub async fn get_envelope_by_bundle(bundle_txid: &str) -> Result<Vec<String>, Error> {
    let conn = ps_client().await?;
    let query_str = format!(
        "SELECT envelope_txid FROM bundles_indexer WHERE bundle_txid = \"{}\"",
        bundle_txid
    );
    let results: Vec<EnvelopeTxId> = query(&query_str).fetch_all(&conn).await?;
    Ok(results.into_iter().map(|r| r.envelope_txid).collect())
}

pub async fn get_entries_by_block(block_id: u32) -> Result<Vec<BundleIndexer>, Error> {
    let conn = ps_client().await?;
    let query_str = format!(
        "SELECT * FROM bundles_indexer WHERE block_id = {}",
        block_id
    );
    let results: Vec<BundleIndexer> = query(&query_str).fetch_all(&conn).await?;
    Ok(results)
}

pub async fn update_last_processed_block(block_nr: u32) -> Result<(), Error> {
    let conn = ps_client().await?;
    let query_str = format!("INSERT INTO block_tracker(last_block) VALUES({})", block_nr);
    println!("Executing query: {}", query_str);
    query(&query_str).execute(&conn).await?;
    Ok(())
}

pub async fn get_latest_block_id() -> Result<u32, Error> {
    let conn = ps_client().await?;
    let query_str = "SELECT last_block as max_block FROM block_tracker ORDER BY id DESC LIMIT 1";

    let result: MaxBlock = query(query_str).fetch_one(&conn).await?;
    Ok(result.max_block)
}
