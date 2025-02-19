use crate::config::config;
use crate::crypto::gen_signature;
use crate::error::Result;
use crate::time::now_timestamp;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DataResult {
    category: String,
    symbol: String,
    pub list: Vec<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct DataResponse {
    #[serde(rename(deserialize = "retCode"))]
    ret_code: i32,
    #[serde(rename(deserialize = "retMsg"))]
    ret_msg: String,
    pub result: DataResult,
    time: u128
}

pub async fn fetch(symbol: &str, interval: i32, limit: i32) -> Result<DataResult> {
    let api_key = &config().api_key;
    let api_url = &config().api_url;
    let recv_window = &config().recv_window;

    let ts = now_timestamp();
    let payload = format!(
        "symbol={}&interval={}&limit={}&category=spot",
        symbol, interval, limit
    );
    let sign = gen_signature(&payload);

    let client = Client::new()
        .get(format!("{}{}?{}", api_url, "/v5/market/kline", payload))
        .header("X-BAPI-API-KEY", api_key)
        .header("X-BAPI-SIGN", sign)
        .header("X-BAPI-SIGN-TYPE", "2")
        .header("X-BAPI-TIMESTAMP", ts)
        .header("X-BAPI-RECV-WINDOW", recv_window)
        .header("Content-Type", "application/json");

    let data: DataResponse = client.send().await?.json().await?;
    Ok(data.result)
}
