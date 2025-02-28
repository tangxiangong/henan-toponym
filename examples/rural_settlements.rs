use henan_toponym::api::*;

/// 本示例展示如何搜索河南省的所有农村居民点(代码 22200)
/// 
/// 运行方式:
/// ```
/// cargo run --example rural_settlements
/// ```
#[tokio::main]
async fn main() {
    println!("开始搜索河南省的农村居民点...");
    
    // 创建搜索参数，搜索河南省的所有农村居民点(代码 22200)
    let params = SearchParamsBuilder::default()
        .st_name("") // 不限制地名，使用空字符串
        .place_type_code("22200") // 设置地名类别代码为农村居民点
        .code("410726") // 河南省的行政区划代码简写
        .search_type(SearchType::Fuzzy) // 使用模糊搜索
        .page(1) // 第一页
        .size(100) // 每页100条记录
        .build()
        .unwrap();

    println!("搜索参数: {:?}", params);
    
    // 执行搜索
    match Cli::search(&params).await {
        Ok(records) => {
            println!("找到 {} 个农村居民点", records.len());
            
            // 过滤结果，确保只显示农村居民点
            let rural_settlements: Vec<_> = records.iter()
                .filter(|record| record.place_type_code.as_deref() == Some("22200"))
                .collect();
                
            println!("其中农村居民点数量: {}", rural_settlements.len());
            
            // 打印搜索结果
            for (i, record) in rural_settlements.iter().enumerate() {
                if i >= 10 {
                    println!("... 还有 {} 个结果未显示", rural_settlements.len() - 10);
                    break;
                }
                
                println!("\n{}. {} (ID: {})", i + 1, record.standard_name, record.id);
                println!("   所在位置: {} {} {}", 
                    record.province_name.as_deref().unwrap_or("未知"),
                    record.city_name.as_deref().unwrap_or("未知"),
                    record.area_name.as_deref().unwrap_or("未知"));
                println!("   地名类别: {} (代码: {})", 
                    record.place_type,
                    record.place_type_code.as_deref().unwrap_or("未知"));
                
                // 如果有坐标信息，打印坐标
                if let Some(ref gdm) = record.gdm {
                    println!("   坐标: {:?}", gdm.coordinates);
                }
                
                // 如果有少数民族语书写，打印
                if let Some(ethnic) = &record.ethnic_minorities_writing {
                    println!("   少数民族语书写: {}", ethnic);
                }
            }
            
            println!("\n要获取更多信息，可以使用ID查询详细信息:");
            println!("例如: cargo run --example details <ID>");
        }
        Err(e) => {
            println!("搜索失败: {:?}", e);
        }
    }
} 