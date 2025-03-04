use anyhow::Result;
use henan_toponym::details::*;
use std::env;

const _CODE: &str = "410000000000";

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        // 如果提供了参数，则处理单个县级行政区划
        let county_code = &args[1];
        println!("Processing single county: {}", county_code);
        test_single_county_details(county_code).await?;
    } else {
        // 否则处理所有县级行政区划
        println!("Processing all counties...");
        rural_settlements_details().await?;
    }
    
    Ok(())
}
