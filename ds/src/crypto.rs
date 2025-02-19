use crate::config::config;
use crate::time::now_timestamp;
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub fn gen_signature(payload: &str) -> String {
    let api_key = &config().api_key;
    let secret_key = &config().secret_key;
    let recv_window = &config().recv_window;

    let mut hmac_sha256 = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes()).unwrap();
    let param_str = format!("{}{}{}{}", now_timestamp(), api_key, recv_window, payload);
    hmac_sha256.update(param_str.as_bytes());
    let hmac_result = hmac_sha256.finalize().into_bytes();
    hmac_result
        .iter()
        .map(|&x| format!("{:x?}", x))
        .collect::<String>()
}