use super::{details::*, division::*, search::*};
use reqwest::{Client, Error};

pub struct Cli {
    client: Client,
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

impl Cli {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn division(
        &self,
        code: &str,
        max_level: QueryLevel,
    ) -> Result<DivisonQueryResult, Error> {
        let req = DivisionQueryParams::latest(code, max_level);
        let cli: reqwest::RequestBuilder = self.client.get(DIVISION_QUERY_URL).query(&req);
        let response = cli.send().await?.json::<DivisionQueryResponse>().await?;
        Ok(response.data)
    }

    pub async fn search(params: &SearchParams) -> Result<Vec<Record>, Error> {
        let client = Client::new();
        
        // 创建一个新的参数对象，确保行政区划代码正确
        let mut query_params = std::collections::HashMap::new();
        query_params.insert("stName".to_string(), params.st_name().to_string());
        
        // 使用简化的行政区划代码
        if let Some(code) = params.simplified_code() {
            query_params.insert("code".to_string(), code);
        }
        
        if let Some(place_type_code) = params.place_type_code() {
            query_params.insert("PlaceTypeCode".to_string(), place_type_code.to_string());
        }
        
        if let Some(year) = params.year() {
            query_params.insert("year".to_string(), year.to_string());
        }
        
        if let Some(search_type) = params.search_type() {
            let type_str = match search_type {
                SearchType::Exact => "精确",
                SearchType::Fuzzy => "模糊",
            };
            query_params.insert("searchType".to_string(), type_str.to_string());
        }
        
        if let Some(page) = params.page() {
            query_params.insert("page".to_string(), page.to_string());
        }
        
        if let Some(size) = params.size() {
            query_params.insert("size".to_string(), size.to_string());
        }
        
        let request_builder = client.get(SEARCH_URL).query(&query_params);
        
        // 打印请求URL
        let request = request_builder.try_clone().unwrap().build().unwrap();
        println!("请求URL: {:?}", request.url());
        
        // 发送请求
        let response = request_builder.send().await?;
        let status = response.status();
        println!("响应状态码: {}", status);
        
        if !status.is_success() {
            let text = response.text().await?;
            println!("错误响应: {}", text);
            return Ok(Vec::new());
        }
        
        let text = response.text().await?;
        println!("响应内容: {}", text);
        
        match serde_json::from_str::<SearchResponse>(&text) {
            Ok(search_response) => Ok(search_response.records),
            Err(e) => {
                println!("解析错误: {:?}", e);
                Ok(Vec::new())
            }
        }
    }

    pub async fn details(&self, id: &str) -> Result<DetailsQueryResponse, Error> {
        let req = DetailsQueryParams::new(id.to_string());
        let cli = self.client.post(DETAILS_QUERY_URL).query(&req);
        let response = cli.send().await?.json::<DetailsQueryResponse>().await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_division_query() {
        let cli = Cli::new();
        let result = cli.division("410000000000", QueryLevel::GrandChild).await;
        assert!(result.is_ok(), "API调用失败: {:?}", result.err());
        let data = result.unwrap();
        assert!(!data.children.is_empty(), "河南省下级行政区划不应为空");
    }

    #[tokio::test]
    async fn test_details_query() {
        let cli = Cli::new();
        let response = cli.details("7531bd84-5dd9-4323-b8fe-50b5c9d5f793").await;
        assert!(response.is_ok(), "API调用失败: {:?}", response.err());
    }

    #[tokio::test]
    #[ignore]
    async fn test_search() {
        let params = SearchParamsBuilder::default()
            .st_name("洛阳市")
            .search_type(SearchType::Fuzzy)
            .page(1)
            .size(100)
            .build()
            .unwrap();

        let records = Cli::search(&params).await;
        assert!(records.is_ok(), "API调用失败: {:?}", records.err());

        let records = records.unwrap();
        assert!(!records.is_empty(), "搜索结果不应为空");
        println!("{:#?}", records[0]);
    }
    
    #[tokio::test]
    #[ignore]
    async fn test_search_rural_settlements() {
        // 创建搜索参数，搜索河南省的所有农村居民点(代码 22200)
        let params = SearchParamsBuilder::default()
            .st_name("河南省") // 设置搜索区域为河南省
            .place_type_code("22200") // 设置地名类别代码为农村居民点
            .code("41") // 河南省的行政区划代码简写
            .search_type(SearchType::Fuzzy) // 使用模糊搜索
            .page(1) // 第一页
            .size(100) // 每页100条记录
            .build()
            .unwrap();

        println!("开始搜索河南省的农村居民点...");
        let records = Cli::search(&params).await;
        assert!(records.is_ok(), "API调用失败: {:?}", records.err());

        let records = records.unwrap();
        println!("找到 {} 个农村居民点", records.len());
        
        if !records.is_empty() {
            // 打印第一条记录的详细信息
            println!("第一条记录:");
            println!("ID: {}", records[0].id);
            println!("标准名称: {}", records[0].standard_name);
            println!("地名类别: {}", records[0].place_type);
            println!("所在省: {}", records[0].province_name.as_deref().unwrap_or("未知"));
            println!("所在市: {}", records[0].city_name.as_deref().unwrap_or("未知"));
            println!("所在区县: {}", records[0].area_name.as_deref().unwrap_or("未知"));
            
            // 如果有坐标信息，打印坐标
            if let Some(ref gdm) = records[0].gdm {
                println!("坐标: {:?}", gdm.coordinates);
            }
        }
    }
}
