use henan_toponym::api::*;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("用法: cargo run --example search <地名> [行政区划代码]");
        return;
    }

    let st_name = args[1].clone();
    let code = if args.len() >= 3 { Some(args[2].clone()) } else { None };

    let mut binding = SearchParamsBuilder::default();
    let mut builder = binding
        .st_name(st_name)
        .search_type(SearchType::Fuzzy)
        .page(1)
        .size(10);

    if let Some(code_str) = code {
        builder = builder.code(code_str);
    }

    let params = builder.build().unwrap();
    println!("搜索参数: {:?}", params);

    match Cli::search(&params).await {
        Ok(records) => {
            println!("找到 {} 条记录", records.len());
            for (i, record) in records.iter().enumerate() {
                println!("{}. {} ({})", i + 1, record.standard_name, record.id);
                println!("   省: {}, 市: {}, 区/县: {}", 
                    record.province_name.as_deref().unwrap_or("未知"),
                    record.city_name.as_deref().unwrap_or("未知"),
                    record.area_name.as_deref().unwrap_or("未知"));
                println!("   类型: {}", record.place_type);
                println!();
            }
        }
        Err(e) => {
            println!("搜索失败: {:?}", e);
        }
    }
} 