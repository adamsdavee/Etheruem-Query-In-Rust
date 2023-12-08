
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Blockbook {
    pub coin: String,
    // host: String,
    // version: String,
    // git_commit: String,
    // build_time: String,
    // sync_mode: bool,
    // initial_sync: bool,
    // in_sync: bool,
    // best_height: i64,
    // last_block_time: String,
    // in_sync_mempool: bool,
    // last_mempool_time: String,
    // mempool_size: i64,
    // decimals: i64,
    // db_size: i64,
    // about: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Backend {
    pub chain: String,
    // blocks: i64,
    // headers: i64,
    // best_block_hash: String,
    // difficulty: String,
    // size_on_disk: i64,
    // version: String,
    // subversion: String,
    // protocol_version: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainStatus {
    pub blockbook: Blockbook,
    pub backend: Backend,
}

// {
//     "blockbook": {
//       "coin": "Bitcoin",
//       "host": "d2b14ae537c5",
//       "version": "devel",
//       "gitCommit": "6e0a045",
//       "buildTime": "2022-10-10T08:40:58+00:00",
//       "syncMode": true,
//       "initialSync": false,
//       "inSync": true,
//       "bestHeight": 759521,
//       "lastBlockTime": "2022-10-20T13:40:09.556273135Z",
//       "inSyncMempool": true,
//       "lastMempoolTime": "2022-10-20T13:41:10.21158272Z",
//       "mempoolSize": 2449,
//       "decimals": 8,
//       "dbSize": 381468138336,
//       "about": "Blockbook - blockchain indexer for Trezor wallet https://trezor.io/. Do not use for any other purpose."
//     },
//     "backend": {
//       "chain": "main",
//       "blocks": 759521,
//       "headers": 759521,
//       "bestBlockHash": "0000000000000000000606c73e60548cefbd4e3553e76dc13d2c55be204795d7",
//       "difficulty": "35610794164371.65",
//       "sizeOnDisk": 492471630827,
//       "version": "230000",
//       "subversion": "/Satoshi:23.0.0/",
//       "protocolVersion": "70016"
//     }
//   }