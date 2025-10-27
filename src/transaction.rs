use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub timestamp: u64,
    pub hash: String,
}

impl Transaction {
    pub fn new(from: &str, to: &str, amount: u64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let hash = Self::calculate_hash(from, to, amount, timestamp);

        Transaction {
            from: from.to_string(),
            to: to.to_string(),
            amount,
            timestamp,
            hash,
        }
    }

    pub fn calculate_hash(from: &str, to: &str, amount: u64, timestamp: u64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(from.as_bytes());
        hasher.update(to.as_bytes());
        hasher.update(amount.to_string().as_bytes());
        hasher.update(timestamp.to_string().as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn is_valid(&self) -> bool {
        self.hash == Self::calculate_hash(&self.from, &self.to, self.amount, self.timestamp)
    }
}
