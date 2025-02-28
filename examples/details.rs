use henan_toponym::api::*;
use std::env;

/// 本示例展示如何查询地名的详细信息
/// 
/// 运行方式:
/// ```
/// cargo run --example details <地名ID>
/// ```
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("用法: cargo run --example details <地名ID>");
        println!("示例: cargo run --example details 7531bd84-5dd9-4323-b8fe-50b5c9d5f793");
        return;
    }

    let id = args[1].clone();
    println!("正在查询ID为 {} 的地名详细信息...", id);
    
    let cli = Cli::new();
    match cli.details(&id).await {
        Ok(details) => {
            println!("\n地名详细信息:");
            println!("标准名称: {}", details.standard_name);
            println!("地名代码: {}", details.place_code);
            println!("地名类别: {} (代码: {})", details.place_type, details.place_type_code);
            println!("所在位置: {} {} {}", 
                details.province_name,
                details.city_name.as_deref().unwrap_or(""),
                details.area_name.as_deref().unwrap_or(""));
            
            // 打印坐标信息
            println!("坐标类型: {}", details.gdm.r#type);
            println!("坐标: {:?}", details.gdm.coordinates);
            
            // 打印地名来历和含义
            if !details.place_origin.is_empty() {
                println!("\n地名来历:");
                println!("{}", details.place_origin);
            }
            
            if !details.place_meaning.is_empty() {
                println!("\n地名含义:");
                println!("{}", details.place_meaning);
            }
            
            // 打印历史沿革
            if let Some(history) = details.government_history {
                if !history.is_empty() {
                    println!("\n历史沿革:");
                    println!("{}", history);
                }
            }
            
            // 打印历史地名
            if let Some(old_name) = details.old_name {
                if !old_name.is_empty() {
                    println!("\n历史地名:");
                    println!("{}", old_name);
                }
            }
            
            // 打印罗马字母拼写
            println!("\n罗马字母拼写: {}", details.roman_alphabet_spelling);
            
            // 打印少数民族语书写
            if !details.ethnic_minorities_writing.is_empty() {
                println!("少数民族语书写: {}", details.ethnic_minorities_writing);
            }
        }
        Err(e) => {
            println!("查询失败: {:?}", e);
        }
    }
} 