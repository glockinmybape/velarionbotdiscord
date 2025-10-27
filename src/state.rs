use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct State {
    pub balances: HashMap<String, u64>,
}

impl State {
    pub fn new() -> Self {
        State {
            balances: HashMap::new(),
        }
    }

    pub fn update_balance(&mut self, address: &str, amount: i64) -> bool {
        if let Some(balance) = self.balances.get_mut(address) {
            if *balance as i64 + amount < 0 {
                return false;
            }
            *balance = (*balance as i64 + amount) as u64;
            true
        } else {
            if amount < 0 {
                return false;
            }
            self.balances.insert(address.to_string(), amount as u64);
            true
        }
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }

    pub fn apply_transaction(&mut self, tx: &Transaction) -> bool {
        if self.get_balance(&tx.from) < tx.amount {
            return false;
        }

        self.update_balance(&tx.from, -(tx.amount as i64));
        self.update_balance(&tx.to, tx.amount as i64);
        true
    }
}
