use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::transaction::Transaction;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub previous_hash: String,
    pub transactions: Vec<Transaction>,
    pub hash: String,
}

impl Block {
    pub fn genesis() -> Self {
        Block {
            index: 0,
            timestamp: 1620000000,
            previous_hash: "0".to_string(),
            transactions: vec![],
            hash: "genesis_hash".to_string(),
        }
    }

    pub fn new(index: u64, transactions: Vec<Transaction>, miner_address: &str, difficulty: u8) -> Self {
        Block {
            index,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            previous_hash: "previous_hash_placeholder".to_string(),
            transactions,
            hash: Self::calculate_hash(miner_address, difficulty),
        }
    }

    fn calculate_hash(miner_address: &str, _difficulty: u8) -> String {
        let mut hasher = Sha256::new();
        hasher.update(miner_address.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: u8,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            chain: vec![Block::genesis()],
            difficulty: 3,
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.chain.last_mut().unwrap().transactions.push(transaction);
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let block = Block::new(self.chain.len() as u64, transactions, "validator_001", self.difficulty);
        self.chain.push(block);
    }
}
