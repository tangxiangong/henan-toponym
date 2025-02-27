use henan_toponym::api::*;

const CODE: &str = "410000000000";

#[tokio::main]
async fn main() {
    let cli = Cli::new();
    let henan = cli.division(CODE, QueryLevel::GrandChild).await.unwrap();
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
    println!("{:#?}", codes);
    println!("{:#?}", henan.children.last().unwrap());
}
