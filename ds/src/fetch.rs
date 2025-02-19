use crate::crypto::gen_signature;
use crate::time::now_timestamp;
use crate::{API_KEY, API_URL, RECV_WINDOW};
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DataResult {
    category: String,
    symbol: String,
    list: Vec<Vec<String>>,
}
#[derive(Deserialize, Debug)]
// #[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct DataResponse {
    #[serde(rename(deserialize = "retCode"))]
    ret_code: i32,
    #[serde(rename(deserialize = "retMsg"))]
    ret_msg: String,
    result: DataResult,
    time: u128
}


pub async fn fetch(symbol: &str, interval: i32) -> Result<DataResponse, reqwest::Error> {
    let ts = now_timestamp();
    let payload = format!(
        "symbol={}&interval={}&limit=10&category=spot",
        symbol, interval
    );
    let sign = gen_signature(&payload);
    let client = Client::new()
        .get(format!("{}{}?{}", API_URL, "/v5/market/kline", payload))
        .header("X-BAPI-API-KEY", API_KEY)
        .header("X-BAPI-SIGN", sign)
        .header("X-BAPI-SIGN-TYPE", "2")
        .header("X-BAPI-TIMESTAMP", ts)
        .header("X-BAPI-RECV-WINDOW", RECV_WINDOW)
        .header("Content-Type", "application/json");

    let data = client.send().await?.json().await;
    data
}
