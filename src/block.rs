use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};
use crate::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Block {
            index: 0,
            timestamp,
            transactions,
            previous_hash: String::from("0"),
            nonce: 0,
        }
    }

    pub fn hash(&self) -> String {
        let data = serde_json::to_string(self).unwrap();
        let hash = Sha256::digest(data.as_bytes());
        format!("{:x}", hash)
    }
}

// Генезис-блок с эмиссией VLR
pub const GENESIS_BLOCK: Block = Block {
    index: 0,
    timestamp: 1635000000,
    transactions: vec![Transaction {
        from: "vlr0000".to_string(), // Адрес создателя
        to: "vlr1234".to_string(),   // Валидатор
        amount: 1_000_000_000,       // Максимальная эмиссия VLR
        gas_fee: 0,
        signature: None,
        token: "VLR".to_string(),    // Указан символ токена
    }],
    previous_hash: "0000000000000000".to_string(),
    nonce: 0,
};
