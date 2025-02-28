use crate::api::*;

pub async fn county_division(code: &str) -> Vec<String> {
    let cli = Cli::new();
    let henan = cli.division(code, QueryLevel::GrandChild).await.unwrap();
    let codes = henan
        .children
        .iter()
        .map(|c| {
            if c.children.is_empty() {
                vec![c.code.clone()]
            } else {
                c.children
                    .iter()
                    .map(|c| c.code.clone())
                    .collect::<Vec<_>>()
            }
        })
        .collect::<Vec<_>>();

    codes.into_iter().flatten().collect()
}

pub async fn filter_county_division(code: &str) -> Vec<String> {
    let cli = Cli::new();
    let henan = cli.division(code, QueryLevel::GrandChild).await.unwrap();
    let codes = henan
        .children
        .iter()
        .map(|c| {
            if c.children.is_empty() {
                vec![c.code.clone()]
            } else {
                c.children
                    .iter()
                    .filter(|c| c.r#type == "县")
                    .map(|c| c.code.clone())
                    .collect::<Vec<_>>()
            }
        })
        .collect::<Vec<_>>();

    codes.into_iter().flatten().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::Cli;

    #[tokio::test]
    #[ignore]
    async fn test_county_division() {
        let codes = filter_county_division("410000000000").await;
        println!("{:#?}", codes.len());
        let res = Cli::new()
            .division(&codes[0], QueryLevel::GrandChild)
            .await
            .unwrap();
        println!("{:#?}", res);
        let res = Cli::new().division(&res.children[0].code, QueryLevel::Child).await.unwrap();
        println!("{:#?}", res);
    }
}
