use crate::division_query::{
    DIVISION_QUERY_URL, DivisionQueryRequest, DivisionQueryResponse, DivisonQueryResult, QueryLevel,
};
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

    pub async fn division_query(
        &self,
        code: String,
        max_level: QueryLevel,
    ) -> Result<DivisonQueryResult, Error> {
        let req = DivisionQueryRequest::latest(code, max_level);
        let cli = self.client.get(DIVISION_QUERY_URL).query(&req);
        let response = cli.send().await?.json::<DivisionQueryResponse>().await?;
        Ok(response.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_division_query() {
        let cli = Cli::new();
        let result = cli
            .division_query("410000000000".to_string(), QueryLevel::Child)
            .await;
        assert!(result.is_ok(), "API调用失败: {:?}", result.err());
        let data = result.unwrap();
        assert!(data.children.is_some());
        if let Some(children) = data.children {
            assert!(!children.is_empty(), "河南省下级行政区划不应为空");
            for child in children {
                println!("{:?}", child);
            }
        }
    }
}
