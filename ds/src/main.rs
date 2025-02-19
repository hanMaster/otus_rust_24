use crate::fetch::fetch;

const API_KEY: &str = "";
const SECRET_KEY: &str = "";
const RECV_WINDOW: &str = "5000";
const API_URL: &str = "https://api-testnet.bybit.com";

mod time;
mod crypto;
mod fetch;

#[tokio::main]
async fn main() {
    let data = fetch("BTCUSDT", 5).await;
    if let Ok(data) = data {
        println!("{:#?}", data);
    }
}
