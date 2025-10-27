use std::collections::HashMap;
use tokio::time::{self, Duration};
use warp::Filter;
use crate::blockchain::Blockchain;
use crate::consensus::start_block_generation;

mod blockchain;
mod transaction;
mod state;
mod database;
mod consensus;
mod p2p;
mod rpc;
mod crypto;
mod utils;

#[tokio::main]
async fn main() {
    // Инициализация блокчейна
    let mut blockchain = Blockchain::new();

    // Добавление тестовой транзакции
    blockchain.add_transaction(transaction::Transaction::new(
        "test_wallet_1",
        "test_wallet_2",
        50,
    ));

    // Запуск консенсуса (PoS)
    let validators = vec![
        consensus::Validator {
            address: "validator_001".to_string(),
            stake: 1000,
        },
        consensus::Validator {
            address: "validator_002".to_string(),
            stake: 500,
        },
    ];

    tokio::spawn(async move {
        start_block_generation(&mut blockchain, &mut state::State::new(), &validators, 10).await;
    });

    // Запуск RPC-сервера
    let blockchain_rpc = blockchain.clone();
    let rpc_api = rpc::create_rpc_router(blockchain_rpc, state::State::new());

    warp::serve(rpc_api)
        .run(([127, 0, 0, 1], 8000));
}
