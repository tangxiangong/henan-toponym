use crate::{division::*, details::*, search::*};
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
        code: String,
        max_level: QueryLevel,
    ) -> Result<DivisonQueryResult, Error> {
        let req = DivisionQueryParams::latest(code, max_level);
        let cli = self.client.get(DIVISION_QUERY_URL).query(&req);
        let response = cli.send().await?.json::<DivisionQueryResponse>().await?;
        Ok(response.data)
    }

    pub async fn search(params: &SearchParams) -> Result<Vec<Record>, Error> {
        let cli = Client::new().get(SEARCH_URL).query(params);
        let response = cli.send().await?.json::<SearchResponse>().await?;
        Ok(response.records)
    }

    pub async fn details(
        &self,
        id: String,
    ) -> Result<DetailsQueryResponse, Error> {
        let req = DetailsQueryParams::new(id);
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
        let result = cli
            .division("410000000000".to_string(), QueryLevel::GrandChild)
            .await;
        assert!(result.is_ok(), "API调用失败: {:?}", result.err());
        let data = result.unwrap();
        assert!(data.children.is_some());
        if let Some(children) = data.children {
            assert!(!children.is_empty(), "河南省下级行政区划不应为空");
            for child in children {
                println!("{:#?}", child);
            }
        }
    }

    #[tokio::test]
    async fn test_details_query() {
        let id = "411729000000".to_string();
        let req = DetailsQueryParams::new(id);
        let cli = Client::new().post(DETAILS_QUERY_URL).query(&req);
        let response = cli.send().await.unwrap();
        let body = response.text().await.unwrap();
        println!("{}", body);
        
        // let cli = Cli::new();
        // let response = cli.details("411221000000".to_string()).await;
        // assert!(response.is_ok(), "API调用失败: {:?}", response.err());
        // let data = response.unwrap();
        // println!("{:?}", data);
    }

    #[tokio::test]
    async fn test_search() {
        let params = SearchParamsBuilder::default()
            .st_name("唐庄村")
            .search_type(SearchType::Exact)
            .page(1)
            .size(100)
            .build()
            .unwrap();
        
        let records = Cli::search(&params).await;
        assert!(records.is_ok(), "API调用失败: {:?}", records.err());
        
        let records = records.unwrap();
        assert!(!records.is_empty(), "搜索结果不应为空");
        
        // 验证第一条记录包含预期的地名
        let first_record = &records[0];
        assert_eq!(first_record.standard_name, "唐庄村");
        
        println!("找到 {} 条匹配记录", records.len());
        println!("第一条记录: {:#?}", first_record);
    }
}
