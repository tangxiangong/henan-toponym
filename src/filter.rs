use crate::api::*;
use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

/// 获取并存储所有县级行政区划的前六位代码
pub async fn counties(code: &str) -> io::Result<()> {
    let cli = Cli::new();
    let province = cli
        .division(code, QueryLevel::GrandChild)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // 创建文件用于保存县级代码
    let mut output_file = File::create("county_codes.txt").await?;
    let mut count = 0;

    // 处理所有地级市
    for city in &province.children {
        if city.children.is_empty() {
            // 省直辖县级市，直接存储前六位
            if let Some(code_prefix) = get_code_prefix(&city.code) {
                output_file
                    .write_all(format!("{}\n", code_prefix).as_bytes())
                    .await?;
                count += 1;
            }
            continue;
        }

        // 处理地级市下的县级区域
        for county in &city.children {
            if county.r#type == "县" || county.r#type == "县级市" {
                // 直接写入县级代码的前六位
                if let Some(code_prefix) = get_code_prefix(&county.code) {
                    output_file
                        .write_all(format!("{}\n", code_prefix).as_bytes())
                        .await?;
                    count += 1;
                }
            }
        }
    }

    // 确保所有数据都写入文件
    output_file.flush().await?;

    println!("Total county codes: {}", count);
    Ok(())
}

/// 获取行政代码的前六位
fn get_code_prefix(code: &str) -> Option<String> {
    if code.len() >= 6 {
        Some(code[..6].to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_county_division() {
        let result = counties("410000000000").await;
        match result {
            Ok(_) => println!("County codes saved successfully"),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
