use rocksdb::{DB, Options};
use serde_json::to_string;

pub struct BlockchainStorage {
    db: DB,
}

impl BlockchainStorage {
    pub fn open(path: &str) -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path).unwrap();
        BlockchainStorage { db }
    }

    pub fn save_block(&self, block: &Block) {
        let key = format!("block_{}", block.index);
        let value = to_string(block).unwrap();
        self.db.put(key, value).unwrap();
    }

    pub fn get_block(&self, index: u64) -> Option<Block> {
        let key = format!("block_{}", index);
        match self.db.get(key) {
            Ok(Some(value)) => serde_json::from_slice(&value).ok(),
            _ => None,
        }
    }
}
