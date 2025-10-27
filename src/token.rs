use std::collections::HashMap;

#[derive(Debug)]
pub struct VelarionToken {
    pub name: String,
    pub symbol: String,
    pub max_supply: u64,
    pub decimals: u8,
    pub total_supply: u64,
    pub balances: HashMap<String, u64>,
}

impl VelarionToken {
    pub fn new() -> Self {
        Self {
            name: "Velarion Token".to_string(),
            symbol: "VLR".to_string(),
            max_supply: 1_000_000_000,
            decimals: 8,
            total_supply: 0,
            balances: HashMap::new(),
        }
    }

    // Создание генезис-эмиссии
    pub fn genesis(&mut self, creator: &str) {
        if self.total_supply == 0 {
            self.balances.insert(creator.to_string(), self.max_supply);
            self.total_supply = self.max_supply;
        }
    }

    // Проверка баланса
    pub fn balance_of(&self, address: &str) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }

    // Перевод токенов
    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), &'static str> {
        let from_balance = self.balance_of(from);
        if from_balance < amount {
            return Err("Insufficient balance");
        }

        self.balances.insert(from.to_string(), from_balance - amount);
        let to_balance = self.balance_of(to);
        self.balances.insert(to.to_string(), to_balance + amount);

        Ok(())
    }
}
