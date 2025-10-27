use sha2::{Digest, Sha256};
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};

/// Хеширование данных с использованием SHA
