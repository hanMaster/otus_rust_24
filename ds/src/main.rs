use ds::chart::gen_candlestick_svg;
pub use ds::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    gen_candlestick_svg().await?;
    Ok(())
}
