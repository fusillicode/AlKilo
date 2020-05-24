#![feature(async_closure)]
use fantoccini::{Client, Locator};
use futures::stream::{self, StreamExt};

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let mut c = Client::new("http://localhost:4444")
        .await
        .expect("failed to connect to WebDriver");

    c.goto("https://www.carrefour.it/spesa-online").await?;

    let categories = c.find_all(Locator::Css(".category-list ul li")).await?;

    let names: Vec<Option<String>> = stream::iter(categories)
        .then(async move |mut el| el.text().await.ok())
        .collect()
        .await;

    println!("{:?}", names);

    c.close().await
}
