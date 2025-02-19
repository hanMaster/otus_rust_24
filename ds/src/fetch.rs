use std::ops::Deref;
use crate::config::config;
use crate::crypto::gen_signature;
use crate::error::Result;
use crate::time::{now_timestamp, timestamp_to_time};
use charts_rs::Series;
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
    time: u128,
}

pub struct ChartData {
    pub series_list: Vec<Series>,
    pub x_axis_data: Vec<String>,
    pub min: f32,
    pub max: f32,
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

pub async fn get_data(symbol: &str, interval: i32, limit: i32) -> Result<ChartData> {
    let data = fetch(symbol, interval, limit).await?;
    let mut list = data.list;
    list.reverse();
    let mut data: Vec<f32> = Vec::new();
    let mut x_axis_data: Vec<String> = Vec::new();
    for i in list {
        let time = timestamp_to_time(&i[0])?;
        x_axis_data.push(time);
        data.push(i[4].parse()?);
        data.push(i[1].parse()?);
        data.push(i[2].parse()?);
        data.push(i[3].parse()?);
    }

    x_axis_data.reverse();

    // println!("data {:?}", data);
    // println!("x_axis {:?}", x_axis_data);

    let mut min_max = data.clone();
    min_max.sort_by(|a, b| a.total_cmp(b));

    let max = min_max.last();

    let series = Series::new(symbol.to_string(), data);
    Ok(ChartData {
        series_list: vec![series],
        x_axis_data,
        min: min_max[0],
        max: *max.unwrap()
    })
}
