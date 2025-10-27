#[test]
fn test_invalid_signature() {
    let mut tx = Transaction {
        from: "vlr123".to_string(),
        to: "vlr456".to_string(),
        amount: 100,
        gas_fee: 10,
        signature: None,
    };

    // Подписываем транзакцию с другим ключом
    let invalid_keypair = Keypair::generate(&mut rand::thread_rng());
    tx.sign(&invalid_keypair);

    assert!(!tx.verify()); // Подпись должна быть неверной
}