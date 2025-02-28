use anyhow::Result;
use henan_toponym::filter::*;

const CODE: &str = "410000000000";

#[tokio::main]
async fn main() -> Result<()> {
    counties(CODE).await?;
    Ok(())
}
