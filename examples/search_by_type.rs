use henan_toponym::api::*;
use std::env;

/// 本示例展示如何按地名类别代码搜索地名
/// 
/// 常见地名类别代码:
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
/// cargo run --example search_by_type <地名类别代码> [省份代码]
/// ```
/// 
/// 例如:
/// ```
/// # 搜索河南省的农村居民点
/// cargo run --example search_by_type 22200 41
/// 
/// # 搜索全国的湖泊
/// cargo run --example search_by_type 25100
/// ```
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("用法: cargo run --example search_by_type <地名类别代码> [省份代码]");
        println!("例如: cargo run --example search_by_type 22200 41  # 搜索河南省的农村居民点");
        println!("      cargo run --example search_by_type 25100     # 搜索全国的湖泊");
        println!("\n常见地名类别代码:");
        println!("- 21200: 省级行政区");
        println!("- 21300: 地级行政区");
        println!("- 21400: 县级行政区");
        println!("- 21500: 乡级行政区");
        println!("- 22100: 城镇居民点");
        println!("- 22200: 农村居民点");
        println!("- 23100: 山脉");
        println!("- 24100: 河流");
        println!("- 25100: 湖泊");
        return;
    }

    let place_type_code = args[1].clone();
    let province_code = if args.len() >= 3 { Some(args[2].clone()) } else { None };

    // 获取地名类别的描述
    let place_type_desc = match place_type_code.as_str() {
        "21200" => "省级行政区",
        "21300" => "地级行政区",
        "21400" => "县级行政区",
        "21500" => "乡级行政区",
        "22100" => "城镇居民点",
        "22200" => "农村居民点",
        "23100" => "山脉",
        "24100" => "河流",
        "25100" => "湖泊",
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

    println!("开始搜索{}的{}...", province_name, place_type_desc);
    
    // 创建搜索参数
    let mut builder = SearchParamsBuilder::default();
    
    // 设置基本参数
    builder.st_name("")  // 不限制地名，使用空字符串
           .place_type_code(place_type_code.clone())  // 设置地名类别代码
           .search_type(SearchType::Fuzzy)  // 使用模糊搜索
           .page(1)  // 第一页
           .size(100);  // 每页100条记录
    
    // 如果指定了省份代码，则添加到搜索参数中
    if let Some(code) = province_code {
        builder.code(code);
    }
    
    let params = builder.build().unwrap();
    println!("搜索参数: {:?}", params);
    
    // 执行搜索
    match Cli::search(&params).await {
        Ok(records) => {
            println!("找到 {} 条记录", records.len());
            
            // 打印搜索结果
            for (i, record) in records.iter().enumerate() {
                if i >= 20 {
                    println!("... 还有 {} 个结果未显示", records.len() - 20);
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
            
            println!("\n要获取更多信息，可以使用ID查询详细信息:");
            println!("例如: cargo run --example details <ID>");
        }
        Err(e) => {
            println!("搜索失败: {:?}", e);
        }
    }
} 