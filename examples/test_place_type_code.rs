use henan_toponym::api::*;

/// 本示例用于测试 PlaceTypeCode 的真正用法
/// 
/// 根据 search_by_type.rs 示例中的信息，常见地名类别代码包括：
/// - 21200: 省级行政区
/// - 21300: 地级行政区
/// - 21400: 县级行政区
/// - 21500: 乡级行政区
/// - 22100: 城镇居民点
/// - 22200: 农村居民点
/// - 23100: 山脉
/// - 24100: 河流
/// - 25100: 湖泊
/// 
/// 运行方式:
/// ```
/// cargo run --example test_place_type_code
/// ```
#[tokio::main]
async fn main() {
    println!("测试 PlaceTypeCode 的用法...");
    
    // 测试不同的地名类别代码
    let place_type_codes = vec![
        "21200", // 省级行政区
        "21300", // 地级行政区
        "21400", // 县级行政区
        "21500", // 乡级行政区
        "22100", // 城镇居民点
        "22200", // 农村居民点
        "23100", // 山脉
        "24100", // 河流
        "25100", // 湖泊
    ];
    
    // 限制在河南省内搜索
    let province_code = "41"; // 河南省
    
    for &code in place_type_codes.iter() {
        println!("\n测试地名类别代码: {}", code);
        
        // 创建搜索参数
        let params = SearchParamsBuilder::default()
            .st_name("") // 不限制地名，使用空字符串
            .place_type_code(code) // 设置地名类别代码
            .code(province_code) // 限制在河南省内
            .search_type(SearchType::Fuzzy) // 使用模糊搜索
            .page(1) // 第一页
            .size(5) // 每页5条记录，仅用于测试
            .build()
            .unwrap();
        
        println!("搜索参数: {:?}", params);
        
        // 执行搜索
        match Cli::search(&params).await {
            Ok(records) => {
                println!("找到 {} 条记录", records.len());
                
                // 打印搜索结果
                for (i, record) in records.iter().enumerate().take(3) {
                    println!("{}. {} (ID: {})", i + 1, record.standard_name, record.id);
                    println!("   所在位置: {} {} {}", 
                        record.province_name.as_deref().unwrap_or("未知"),
                        record.city_name.as_deref().unwrap_or("未知"),
                        record.area_name.as_deref().unwrap_or("未知"));
                    println!("   地名类别: {} (代码: {})", 
                        record.place_type,
                        record.place_type_code.as_deref().unwrap_or("未知"));
                }
                
                if records.len() > 3 {
                    println!("   ... 还有 {} 条记录未显示", records.len() - 3);
                }
            }
            Err(e) => {
                println!("搜索失败: {:?}", e);
            }
        }
    }
} 