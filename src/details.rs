use anyhow::Result;
use crate::api::*;
use std::path::Path;
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use std::time::Duration;
use csv::Writer;
use rand::Rng;

/// 失败的请求记录
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FailedRequest {
    id: String,
    name: String,
    county_code: String,
    retry_count: u32,
}

/// 获取并存储县级行政区划下的所有农村居民点详细信息
pub async fn rural_settlements_details() -> Result<()> {
    // 读取county_codes.txt文件中的县级行政区划代码
    let county_codes = tokio::fs::read_to_string("county_codes.txt").await?;
    let county_codes: Vec<&str> = county_codes.lines().collect();
    let total_counties = county_codes.len();
    
    // 创建输出目录
    let output_dir = Path::new("rural_settlements");
    if !output_dir.exists() {
        tokio::fs::create_dir_all(output_dir).await?;
    }
    
    // 创建CSV文件
    let csv_path = output_dir.join("all_settlements.csv");
    let mut csv_writer = Writer::from_path(&csv_path)?;
    
    // 创建失败请求记录文件
    let mut failed_requests: Vec<FailedRequest> = Vec::new();
    let failed_requests_path = output_dir.join("failed_requests.json");
    
    // 如果存在之前的失败请求记录，则加载它们
    if Path::new(&failed_requests_path).exists() {
        let failed_json = tokio::fs::read_to_string(&failed_requests_path).await?;
        if !failed_json.is_empty() {
            failed_requests = serde_json::from_str(&failed_json)?;
            println!("加载了 {} 个之前失败的请求", failed_requests.len());
        }
    }
    
    let cli = Cli::new();
    let mut total_settlements = 0;
    let mut processed_counties = 0;
    
    // 处理每个县级行政区划
    for county_code in county_codes {
        processed_counties += 1;
        println!("[{}/{}] 正在处理县级行政区: {}", processed_counties, total_counties, county_code);
        
        // 创建搜索参数
        let params = SearchParamsBuilder::default()
            .st_name("")
            .place_type_code("22200") // 农村居民点的类别代码
            .code(county_code)
            .search_type(SearchType::Fuzzy)
            .page(1)
            .size(1000)
            .build()
            .unwrap();
        
        // 搜索农村居民点
        let records = match search_with_retry(&cli, &params).await {
            Ok(records) => records,
            Err(e) => {
                eprintln!("搜索县级行政区 {} 失败: {}", county_code, e);
                // 添加随机延迟后继续下一个县
                let delay = rand::rng().random_range(2000..=5000);
                tokio::time::sleep(Duration::from_millis(delay)).await;
                continue;
            }
        };
        
        // 严格筛选农村居民点
        let rural_settlements: Vec<_> = records.into_iter()
            .filter(|r| {
                // 精确筛选农村居民点
                (r.place_type == "农村居民点" && r.place_type_code.as_deref() == Some("22200")) ||
                // 有些数据可能标记不完整，但名称包含"村"且类别代码正确的也应该包含
                (r.standard_name.contains("村") && r.place_type_code.as_deref() == Some("22200"))
            })
            .collect();
        
        println!("找到 {} 个农村居民点，县级行政区: {}", rural_settlements.len(), county_code);
        total_settlements += rural_settlements.len();
        
        // 获取每个农村居民点的详细信息
        for record in rural_settlements {
            println!("获取居民点详细信息: {} ({})", record.standard_name, record.id);
            
            // 获取详细信息，添加重试机制
            match fetch_details_with_retry(&cli, &record.id, &record.standard_name, 3).await {
                Ok(details) => {
                    // 直接将 DetailsQueryResponse 写入 CSV
                    csv_writer.serialize(&details)?;
                    // 确保每条记录写入后立即刷新，避免数据丢失
                    csv_writer.flush()?;
                }
                Err(e) => {
                    eprintln!("获取详细信息失败 {} ({}): {}", record.standard_name, record.id, e);
                    failed_requests.push(FailedRequest {
                        id: record.id,
                        name: record.standard_name,
                        county_code: county_code.to_string(),
                        retry_count: 0,
                    });
                    
                    // 每当有失败请求时，立即更新失败请求记录文件
                    let json = serde_json::to_string_pretty(&failed_requests)?;
                    tokio::fs::write(&failed_requests_path, json).await?;
                }
            }
            
            // 添加随机延迟，避免请求过快被限制
            let delay = rand::rng().random_range(800..=2000);
            tokio::time::sleep(Duration::from_millis(delay)).await;
        }
        
        // 每处理完一个县，保存一次失败请求记录
        if !failed_requests.is_empty() {
            let json = serde_json::to_string_pretty(&failed_requests)?;
            tokio::fs::write(&failed_requests_path, json).await?;
        }
        
        // 每处理完一个县，添加一个较长的随机延迟
        let county_delay = rand::rng().random_range(3000..=8000);
        tokio::time::sleep(Duration::from_millis(county_delay)).await;
    }
    
    // 确保CSV文件被正确写入
    csv_writer.flush()?;
    
    println!("处理完成");
    println!("总农村居民点数: {}", total_settlements);
    println!("处理的县级行政区数: {}/{}", processed_counties, total_counties);
    println!("失败的请求数: {}", failed_requests.len());
    
    // 如果有失败的请求，尝试重试
    if !failed_requests.is_empty() {
        println!("开始重试失败的请求...");
        retry_failed_requests(&cli, &mut failed_requests, &mut csv_writer).await?;
    }
    
    Ok(())
}

/// 带重试机制的搜索请求
async fn search_with_retry(_cli: &Cli, params: &SearchParams) -> Result<Vec<Record>, reqwest::Error> {
    let mut retries = 0;
    let max_retries = 5; // 增加最大重试次数
    
    loop {
        match Cli::search(params).await {
            Ok(records) => return Ok(records),
            Err(e) => {
                retries += 1;
                if retries >= max_retries {
                    return Err(e);
                }
                eprintln!("搜索失败，重试 {}/{}...", retries, max_retries);
                // 指数退避策略，每次重试等待时间增加
                let wait_time = 2u64.pow(retries) + rand::rng().random_range(0..=1000);
                tokio::time::sleep(Duration::from_millis(wait_time)).await;
            }
        }
    }
}

/// 带重试机制的详细信息获取
async fn fetch_details_with_retry(
    cli: &Cli,
    id: &str,
    name: &str,
    max_retries: u32,
) -> Result<DetailsQueryResponse, reqwest::Error> {
    let mut retries = 0;
    
    loop {
        match cli.details(id).await {
            Ok(details) => return Ok(details),
            Err(e) => {
                retries += 1;
                if retries >= max_retries {
                    return Err(e);
                }
                eprintln!("获取详细信息失败 {}，重试 {}/{}...", name, retries, max_retries);
                // 指数退避策略，每次重试等待时间增加
                let wait_time = 2u64.pow(retries as u32) + rand::rng().random_range(0..=1000);
                tokio::time::sleep(Duration::from_millis(wait_time)).await;
            }
        }
    }
}

/// 重试失败的请求
async fn retry_failed_requests(
    cli: &Cli,
    failed_requests: &mut Vec<FailedRequest>,
    csv_writer: &mut Writer<std::fs::File>,
) -> Result<()> {
    let mut retry_count = 0;
    let max_retries = 5; // 增加最大重试次数
    let final_failed_path = "rural_settlements/final_failed_requests.json";
    
    while !failed_requests.is_empty() && retry_count < max_retries {
        retry_count += 1;
        println!("第 {} 次重试，剩余 {} 个失败请求", retry_count, failed_requests.len());
        
        let mut successful_requests = HashSet::new();
        
        for request in failed_requests.iter_mut() {
            if request.retry_count >= 3 { // 单个请求最多重试3次
                continue;
            }
            
            println!("重试请求: {} ({})", request.name, request.id);
            
            match fetch_details_with_retry(cli, &request.id, &request.name, 3).await {
                Ok(details) => {
                    // 直接将 DetailsQueryResponse 写入 CSV
                    csv_writer.serialize(&details)?;
                    // 确保每条记录写入后立即刷新
                    csv_writer.flush()?;
                    successful_requests.insert(request.id.clone());
                    println!("成功获取: {} ({})", request.name, request.id);
                }
                Err(e) => {
                    eprintln!("重试失败 {} ({}): {}", request.name, request.id, e);
                    request.retry_count += 1;
                }
            }
            
            // 添加随机延迟
            let delay = rand::rng().random_range(1000..=3000);
            tokio::time::sleep(Duration::from_millis(delay)).await;
        }
        
        // 移除成功的请求
        failed_requests.retain(|r| !successful_requests.contains(&r.id));
        
        // 每次重试批次后保存当前失败请求状态
        let json = serde_json::to_string_pretty(&failed_requests)?;
        tokio::fs::write(final_failed_path, json).await?;
        
        // 批次之间添加较长延迟
        let batch_delay = rand::rng().random_range(5000..=10000);
        tokio::time::sleep(Duration::from_millis(batch_delay)).await;
    }
    
    // 保存最终的失败请求记录
    if !failed_requests.is_empty() {
        let json = serde_json::to_string_pretty(&failed_requests)?;
        tokio::fs::write(final_failed_path, json).await?;
        println!("最终仍有 {} 个请求失败，已保存到 {}", failed_requests.len(), final_failed_path);
    } else {
        println!("所有失败请求已成功重试！");
    }
    
    Ok(())
}

/// 获取并存储单个县级行政区划下的所有农村居民点详细信息（用于测试）
pub async fn test_single_county_details(county_code: &str) -> Result<()> {
    // 创建输出目录
    let output_dir = Path::new("rural_settlements");
    if !output_dir.exists() {
        tokio::fs::create_dir_all(output_dir).await?;
    }
    
    let cli = Cli::new();
    
    println!("正在处理县级行政区: {}", county_code);
    
    // 创建搜索参数，搜索特定县级行政区划下的所有农村居民点(代码 22200)
    let params = SearchParamsBuilder::default()
        .st_name("") // 不指定地名，搜索所有
        .place_type_code("22200") // 设置地名类别代码为农村居民点
        .code(county_code) // 县级行政区划代码
        .search_type(SearchType::Fuzzy) // 使用模糊搜索
        .page(1) // 第一页
        .size(1000) // 每页1000条记录
        .build()
        .unwrap();
    
    // 搜索农村居民点
    let records = match search_with_retry(&cli, &params).await {
        Ok(records) => records,
        Err(e) => {
            eprintln!("搜索县级行政区 {} 失败: {}", county_code, e);
            return Ok(());
        }
    };
    
    // 严格筛选农村居民点
    let rural_settlements: Vec<_> = records.into_iter()
        .filter(|r| {
            // 精确筛选农村居民点
            (r.place_type == "农村居民点" && r.place_type_code.as_deref() == Some("22200")) ||
            // 有些数据可能标记不完整，但名称包含"村"且类别代码正确的也应该包含
            (r.standard_name.contains("村") && r.place_type_code.as_deref() == Some("22200"))
        })
        .collect();
    
    if rural_settlements.is_empty() {
        println!("未找到农村居民点，县级行政区: {}", county_code);
        return Ok(());
    }
    
    println!("找到 {} 个农村居民点，县级行政区: {}", rural_settlements.len(), county_code);
    
    // 创建CSV文件
    let csv_path = output_dir.join(format!("{}.csv", county_code));
    let mut csv_writer = Writer::from_path(&csv_path)?;
    
    // 创建JSON文件（用于备份和查看）
    let json_path = output_dir.join(format!("{}.json", county_code));
    let mut details_vec = Vec::new();
    
    // 获取每个农村居民点的详细信息
    for record in rural_settlements {
        println!("获取居民点详细信息: {} ({})", record.standard_name, record.id);
        
        // 获取详细信息，添加重试机制
        match fetch_details_with_retry(&cli, &record.id, &record.standard_name, 3).await {
            Ok(details) => {
                // 写入CSV
                csv_writer.serialize(&details)?;
                csv_writer.flush()?;
                
                // 添加到JSON数组
                details_vec.push(details);
                
                println!("成功获取: {}", record.standard_name);
            }
            Err(e) => {
                eprintln!("获取详细信息失败 {} ({}): {}", record.standard_name, record.id, e);
            }
        }
        
        // 添加随机延迟，避免请求过快
        let delay = rand::rng().random_range(800..=2000);
        tokio::time::sleep(Duration::from_millis(delay)).await;
    }
    
    // 将详细信息写入JSON文件
    let json = serde_json::to_string_pretty(&details_vec)?;
    tokio::fs::write(json_path, json).await?;
    
    println!("已保存 {} 个农村居民点详细信息，县级行政区: {}", details_vec.len(), county_code);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_search_rural_settlements() {
        // 测试单个县级行政区划的农村居民点搜索
        let county_code = "410122"; // 中牟县
        
        let params = SearchParamsBuilder::default()
            .st_name("") // 不指定地名，搜索所有
            .place_type_code("22200") // 设置地名类别代码为农村居民点
            .code(county_code) // 县级行政区划代码
            .search_type(SearchType::Fuzzy) // 使用模糊搜索
            .page(1) // 第一页
            .size(10) // 每页10条记录
            .build()
            .unwrap();
        
        let records = Cli::search(&params).await;
        assert!(records.is_ok(), "API调用失败: {:?}", records.err());
        
        let records = records.unwrap();
        println!("找到 {} 个地点，县级行政区: {}", records.len(), county_code);
        
        // 严格筛选农村居民点
        let rural_settlements: Vec<_> = records.iter()
            .filter(|r| {
                // 精确筛选农村居民点
                (r.place_type == "农村居民点" && r.place_type_code.as_deref() == Some("22200")) ||
                // 有些数据可能标记不完整，但名称包含"村"且类别代码正确的也应该包含
                (r.standard_name.contains("村") && r.place_type_code.as_deref() == Some("22200"))
            })
            .collect();
        
        println!("找到 {} 个农村居民点，县级行政区: {}", rural_settlements.len(), county_code);
        
        if !rural_settlements.is_empty() {
            println!("第一个农村居民点: {:#?}", rural_settlements[0]);
            
            // 测试获取详细信息
            let cli = Cli::new();
            let details = cli.details(&rural_settlements[0].id).await;
            assert!(details.is_ok(), "获取详细信息失败: {:?}", details.err());
            
            let details = details.unwrap();
            println!("居民点详细信息: {:#?}", details);
        }
    }
    
    #[tokio::test]
    #[ignore]
    async fn test_rural_settlements_details() {
        let result = rural_settlements_details().await;
        assert!(result.is_ok(), "处理农村居民点详细信息失败: {:?}", result.err());
    }
    
    #[tokio::test]
    async fn test_single_county() {
        let county_code = "410122"; // 中牟县
        let result = test_single_county_details(county_code).await;
        assert!(result.is_ok(), "处理单个县级行政区划失败: {:?}", result.err());
    }
}
