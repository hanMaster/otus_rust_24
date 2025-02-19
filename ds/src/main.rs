use ds::fetch::fetch;
pub use ds::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let data = fetch("BTCUSDT", 5, 3).await?;
    println!("{:#?}", data.list);
    Ok(())
}
