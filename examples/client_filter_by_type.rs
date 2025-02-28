use henan_toponym::api::*;
use std::env;

/// 本示例展示如何在客户端实现地名类别代码过滤
/// 
/// 由于 API 的 PlaceTypeCode 参数不起作用，我们需要在客户端对返回的结果进行过滤
/// 
/// 运行方式:
/// ```
/// cargo run --example client_filter_by_type <地名> <地名类别代码> [省份代码]
/// ```
/// 
/// 例如:
/// ```
/// # 搜索河南省的"郑州"相关地名，并过滤出企业类型
/// cargo run --example client_filter_by_type 郑州 27400 41
/// 
/// # 搜索全国的"北京"相关地名，并过滤出地级行政区类型
/// cargo run --example client_filter_by_type 北京 21300
/// ```
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("用法: cargo run --example client_filter_by_type <地名> <地名类别代码> [省份代码]");
        println!("例如: cargo run --example client_filter_by_type 郑州 27400 41  # 搜索河南省的'郑州'相关地名，并过滤出企业类型");
        println!("      cargo run --example client_filter_by_type 北京 21300     # 搜索全国的'北京'相关地名，并过滤出地级行政区类型");
        println!("\n已知的地名类别代码:");
        println!("- 27400: 企业");
        println!("- 21770: 区片");
        println!("- 21620: 社区");
        println!("- 27100: 党政机关");
        println!("- 23512: 主干路");
        println!("- 21300: 地级行政区");
        println!("- 21500: 乡级行政区");
        return;
    }

    let st_name = args[1].clone();
    let place_type_code = args[2].clone();
    let province_code = if args.len() >= 4 { Some(args[3].clone()) } else { None };

    // 获取地名类别的描述
    let place_type_desc = match place_type_code.as_str() {
        "27400" => "企业",
        "21770" => "区片",
        "21620" => "社区",
        "27100" => "党政机关",
        "23512" => "主干路",
        "21300" => "地级行政区",
        "21500" => "乡级行政区",
        _ => "未知类别",
    };

    // 获取省份名称
    let province_name = match province_code.as_deref() {
        Some("11") => "北京市",
        Some("12") => "天津市",
        Some("13") => "河北省",
        Some("14") => "山西省",
        Some("15") => "内蒙古自治区",
        Some("21") => "辽宁省",
        Some("22") => "吉林省",
        Some("23") => "黑龙江省",
        Some("31") => "上海市",
        Some("32") => "江苏省",
        Some("33") => "浙江省",
        Some("34") => "安徽省",
        Some("35") => "福建省",
        Some("36") => "江西省",
        Some("37") => "山东省",
        Some("41") => "河南省",
        Some("42") => "湖北省",
        Some("43") => "湖南省",
        Some("44") => "广东省",
        Some("45") => "广西壮族自治区",
        Some("46") => "海南省",
        Some("50") => "重庆市",
        Some("51") => "四川省",
        Some("52") => "贵州省",
        Some("53") => "云南省",
        Some("54") => "西藏自治区",
        Some("61") => "陕西省",
        Some("62") => "甘肃省",
        Some("63") => "青海省",
        Some("64") => "宁夏回族自治区",
        Some("65") => "新疆维吾尔自治区",
        Some(code) => code,
        None => "全国",
    };

    println!("开始搜索{}的'{}'相关地名，并过滤出{}类型...", province_name, st_name, place_type_desc);
    
    // 创建搜索参数
    let mut builder = SearchParamsBuilder::default();
    
    // 设置基本参数
    builder.st_name(st_name)
           .search_type(SearchType::Fuzzy)
           .page(1)
           .size(100);  // 获取较多的记录以便过滤
    
    // 如果指定了省份代码，则添加到搜索参数中
    if let Some(code) = province_code {
        builder.code(code);
    }
    
    let params = builder.build().unwrap();
    println!("搜索参数: {:?}", params);
    
    // 执行搜索
    match Cli::search(&params).await {
        Ok(records) => {
            println!("API 返回 {} 条记录", records.len());
            
            // 在客户端进行过滤
            let filtered_records: Vec<_> = records.into_iter()
                .filter(|record| record.place_type_code.as_deref() == Some(place_type_code.as_str()))
                .collect();
            
            println!("过滤后剩余 {} 条记录", filtered_records.len());
            
            // 打印过滤后的搜索结果
            for (i, record) in filtered_records.iter().enumerate() {
                if i >= 20 {
                    println!("... 还有 {} 个结果未显示", filtered_records.len() - 20);
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
            }
            
            if filtered_records.is_empty() {
                println!("\n未找到符合条件的记录。");
            }
        }
        Err(e) => {
            println!("搜索失败: {:?}", e);
        }
    }
} 