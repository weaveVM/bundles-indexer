
DROP TABLE IF EXISTS bundles_indexer;
DROP TABLE IF EXISTS block_tracker;

CREATE TABLE IF NOT EXISTS bundles_indexer (
    id INT AUTO_INCREMENT PRIMARY KEY,
    block_id INT,
    bundle_txid VARCHAR(66),
    envelope_txid VARCHAR(66)
);

CREATE INDEX idx_bundles_indexer_bundle_id ON bundles_indexer(block_id);
CREATE INDEX idx_bundles_indexer_bundle_txid ON bundles_indexer(bundle_txid);
CREATE INDEX idx_bundles_indexer_envelope_txid ON bundles_indexer(envelope_txid);

CREATE TABLE IF NOT EXISTS block_tracker (
    id INT AUTO_INCREMENT PRIMARY KEY,
    last_block INT UNSIGNED
);

CREATE INDEX idx_block_tracker_last_block ON block_tracker(last_block);
