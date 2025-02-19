use hmac::{Hmac, Mac};
use sha2::Sha256;
use crate::{API_KEY, RECV_WINDOW, SECRET_KEY};
use crate::time::now_timestamp;

pub fn gen_signature(payload: &str) -> String {
    let mut hmac_sha256 = Hmac::<Sha256>::new_from_slice(SECRET_KEY.as_bytes()).unwrap();
    let param_str = format!("{}{}{}{}", now_timestamp(), API_KEY, RECV_WINDOW, payload);
    hmac_sha256.update(param_str.as_bytes());
    let hmac_result = hmac_sha256.finalize().into_bytes();
    hmac_result
        .iter()
        .map(|&x| format!("{:x?}", x))
        .collect::<String>()
}