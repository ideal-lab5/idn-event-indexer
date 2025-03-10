use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref GENESIS_HASH: [u8; 32] = {
        let hash = env::var("IDN_GENESIS_HASH")
            .unwrap_or_else(|_| "af97825bf72091072a08b9dbff88d6664e2061bcb4e28a90f17bd85572d8f8ae".to_string());
        let mut bytes = [0u8; 32];
        hex::decode_to_slice(hash, &mut bytes)
            .expect("Invalid genesis hash format in IDN_GENESIS_HASH");
        bytes
    };

    pub static ref DEFAULT_URL: String = {
        env::var("IDN_WS_URL").unwrap_or_else(|_| "ws://127.0.0.1:1234".to_string())
    };
}
