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
        let cli = self.client.get(DIVISION_QUERY_URL).query(&req);
        let response = cli.send().await?.json::<DivisionQueryResponse>().await?;
        Ok(response.data)
    }

    pub async fn search(params: &SearchParams) -> Result<Vec<Record>, Error> {
        let cli = Client::new().get(SEARCH_URL).query(params);
        let response = cli.send().await?.json::<SearchResponse>().await?;
        Ok(response.records)
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
            .st_name("洛阳")
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
}
