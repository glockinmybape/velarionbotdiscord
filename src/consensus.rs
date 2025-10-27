use tokio::time::{self, Duration};
use crate::blockchain::{Blockchain, Block};
use crate::state::State;
use crate::transaction::Transaction;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Validator {
    pub address: String,
    pub stake: u64,
}

pub fn select_validator(validators: &[Validator]) -> Option<String> {
    if validators.is_empty() {
        return None;
    }

    let total_stake: u64 = validators.iter().map(|v| v.stake).sum();
    if total_stake == 0 {
        return None;
    }

    let random_value = rand::random::<u64>() % total_stake;
    let mut cumulative_stake = 0;

    for validator in validators {
        cumulative_stake += validator.stake;
        if random_value < cumulative_stake {
            return Some(validator.address.clone());
        }
    }

    None
}

pub async fn start_block_generation(
    blockchain: &mut Blockchain,
    state: &mut State,
    validators: &[Validator],
    block_interval: u64,
) {
    loop {
        time::sleep(Duration::from_secs(block_interval)).await;

        if let Some(validator) = select_validator(validators) {
            let pending_transactions = blockchain.chain.last().unwrap().transactions.clone();
            let new_block = Block::new(
                blockchain.chain.len() as u64,
                pending_transactions,
                &validator,
                blockchain.difficulty,
            );

            if validate_block(&new_block, blockchain, state) {
                blockchain.add_block(pending_transactions);
                for tx in &pending_transactions {
                    state.apply_transaction(tx);
                }
            }
        }
    }
}

fn validate_block(new_block: &Block, blockchain: &Blockchain, state: &State) -> bool {
    if new_block.previous_hash != blockchain.chain.last().unwrap().hash {
        return false;
    }

    if !new_block.transactions.iter().all(|tx| tx.is_valid()) {
        return false;
    }

    true
}
