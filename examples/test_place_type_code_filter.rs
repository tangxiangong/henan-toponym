use henan_toponym::api::*;

/// 本示例用于测试 PlaceTypeCode 是否能够正确过滤搜索结果
/// 
/// 运行方式:
/// ```
/// cargo run --example test_place_type_code_filter
/// ```
#[tokio::main]
async fn main() {
    println!("测试 PlaceTypeCode 过滤功能...");
    
    // 使用一个具体的地名进行搜索
    let st_name = "郑州";
    
    // 限制在河南省内搜索
    let province_code = "41"; // 河南省
    
    // 先不使用 PlaceTypeCode 过滤
    println!("\n不使用 PlaceTypeCode 过滤，搜索地名: {}", st_name);
    let params1 = SearchParamsBuilder::default()
        .st_name(st_name)
        .code(province_code)
        .search_type(SearchType::Fuzzy)
        .page(1)
        .size(5)
        .build()
        .unwrap();
    
    println!("搜索参数: {:?}", params1);
    
    // 执行搜索
    match Cli::search(&params1).await {
        Ok(records) => {
            println!("找到 {} 条记录", records.len());
            
            // 打印搜索结果
            for (i, record) in records.iter().enumerate().take(5) {
                println!("{}. {} (ID: {})", i + 1, record.standard_name, record.id);
                println!("   所在位置: {} {} {}", 
                    record.province_name.as_deref().unwrap_or("未知"),
                    record.city_name.as_deref().unwrap_or("未知"),
                    record.area_name.as_deref().unwrap_or("未知"));
                println!("   地名类别: {} (代码: {})", 
                    record.place_type,
                    record.place_type_code.as_deref().unwrap_or("未知"));
            }
        }
        Err(e) => {
            println!("搜索失败: {:?}", e);
        }
    }
    
    // 使用 PlaceTypeCode 过滤 - 企业
    println!("\n使用 PlaceTypeCode=27400 (企业) 过滤，搜索地名: {}", st_name);
    let params2 = SearchParamsBuilder::default()
        .st_name(st_name)
        .place_type_code("27400") // 企业
        .code(province_code)
        .search_type(SearchType::Fuzzy)
        .page(1)
        .size(5)
        .build()
        .unwrap();
    
    println!("搜索参数: {:?}", params2);
    
    // 执行搜索
    match Cli::search(&params2).await {
        Ok(records) => {
            println!("找到 {} 条记录", records.len());
            
            // 打印搜索结果
            for (i, record) in records.iter().enumerate().take(5) {
                println!("{}. {} (ID: {})", i + 1, record.standard_name, record.id);
                println!("   所在位置: {} {} {}", 
                    record.province_name.as_deref().unwrap_or("未知"),
                    record.city_name.as_deref().unwrap_or("未知"),
                    record.area_name.as_deref().unwrap_or("未知"));
                println!("   地名类别: {} (代码: {})", 
                    record.place_type,
                    record.place_type_code.as_deref().unwrap_or("未知"));
            }
        }
        Err(e) => {
            println!("搜索失败: {:?}", e);
        }
    }
    
    // 使用 PlaceTypeCode 过滤 - 党政机关
    println!("\n使用 PlaceTypeCode=27100 (党政机关) 过滤，搜索地名: {}", st_name);
    let params3 = SearchParamsBuilder::default()
        .st_name(st_name)
        .place_type_code("27100") // 党政机关
        .code(province_code)
        .search_type(SearchType::Fuzzy)
        .page(1)
        .size(5)
        .build()
        .unwrap();
    
    println!("搜索参数: {:?}", params3);
    
    // 执行搜索
    match Cli::search(&params3).await {
        Ok(records) => {
            println!("找到 {} 条记录", records.len());
            
            // 打印搜索结果
            for (i, record) in records.iter().enumerate().take(5) {
                println!("{}. {} (ID: {})", i + 1, record.standard_name, record.id);
                println!("   所在位置: {} {} {}", 
                    record.province_name.as_deref().unwrap_or("未知"),
                    record.city_name.as_deref().unwrap_or("未知"),
                    record.area_name.as_deref().unwrap_or("未知"));
                println!("   地名类别: {} (代码: {})", 
                    record.place_type,
                    record.place_type_code.as_deref().unwrap_or("未知"));
            }
        }
        Err(e) => {
            println!("搜索失败: {:?}", e);
        }
    }
} 