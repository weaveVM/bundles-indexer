<p align="center">
  <a href="https://wvm.dev">
    <img src="https://raw.githubusercontent.com/weaveVM/.github/main/profile/bg.png">
  </a>
</p>


## About
`bundles-indexer` is an indexer for WeaveVM's [0xbabe bundles](https://github.com/weaveVM/bundler) for a easier envelopes retrieval. Despite being a cloud indexing tool, bundles & envelopes data is retrieved from WeaveVM network.

## REST API methods: V1

#### API endpoint: [indexer.wvm.network](https://indexer.wvm.network)

### 1- Retrieve bundles TXIDs for a given block number

```bash
GET /v1/block/bundles/block_nr
```

### 2- Retrieve envelopes TXIDs for a given block number

```bash
GET /v1/block/envelopes/block_nr
```

### 3- Retrieve envelope raw tx object

```bash
GET /v1/envelope/:envelope_txid
```

### 4- Retrieve envelopes TXIDs for a given bundle TXID

```bash
GET /v1/envelopes/:bundle_txid
```

### 5- Resolve an envelope data for a given envelope TXID (gateway-like feature)

```bash
GET /v1/resolve/:envelope_txid
```

### 6- Return indexer stats

```bash
GET /v1/stats
```

## License
This project is licensed under the [MIT License](./LICENSE)