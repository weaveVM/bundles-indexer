
DROP TABLE IF EXISTS bundles_indexer;

CREATE TABLE IF NOT EXISTS bundles_indexer (
    id INT AUTO_INCREMENT PRIMARY KEY,
    block_id INT,
    bundle_txid VARCHAR(66),
    envelope_txid VARCHAR(66) UNIQUE
);

CREATE INDEX idx_bundles_indexer_bundle_id ON bundles_indexer(block_id);
CREATE INDEX idx_bundles_indexer_bundle_txid ON bundles_indexer(bundle_txid);
CREATE INDEX idx_bundles_indexer_envelope_txid ON bundles_indexer(envelope_txid);
