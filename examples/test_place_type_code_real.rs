use henan_toponym::api::*;

/// 本示例用于测试实际存在的地名类别代码
/// 
/// 根据测试结果，我们发现以下地名类别代码是实际存在的：
/// - 27400: 企业
/// - 21770: 区片
/// - 21620: 社区
/// - 27100: 党政机关
/// 
/// 运行方式:
/// ```
/// cargo run --example test_place_type_code_real
/// ```
#[tokio::main]
async fn main() {
    println!("测试实际存在的地名类别代码...");
    
    // 测试实际存在的地名类别代码
    let place_type_codes = vec![
        "27400", // 企业
        "21770", // 区片
        "21620", // 社区
        "27100", // 党政机关
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