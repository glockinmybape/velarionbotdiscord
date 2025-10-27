use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

/// Получение текущего времени в формате Unix timestamp
pub fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Время не может идти вспять")
        .as_secs()
}

/// Генерация случайной строки заданной длины
pub fn random_string(n: usize) -> String {
    const CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    
    (0..n)
        .map(|_| {
            let idx = rng.gen_range(0..CHARS.len());
            CHARS.chars().nth(idx).unwrap()
        })
        .collect()
}
