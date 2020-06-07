#![feature(async_closure)]
use fantoccini::{Client, Locator};
use futures::stream::{self, StreamExt};
use std::{thread, time};

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let mut c = Client::new("http://localhost:9515")
        .await
        .expect("failed to connect to WebDriver");

    c.goto("https://www.carrefour.it/ricerca?search=&isNoF=false&page=1")
        .await?;
    c.find(Locator::Css("div.delivery-button.js-delivery-button"))
        .await?
        .click()
        .await?;
    c.find(Locator::Css("input.js-autocomplete-maps"))
        .await?
        .send_keys("Milano 20125")
        .await?;
    thread::sleep(time::Duration::from_secs(3));
    c.find(Locator::Css("input.js-autocomplete-maps"))
        .await?
        .send_keys('\u{e00f}'.to_string().as_ref())
        .await?;
    thread::sleep(time::Duration::from_secs(3));
    c.find(Locator::Css("input.js-autocomplete-maps"))
        .await?
        .send_keys('\u{e007}'.to_string().as_ref())
        .await?;
    thread::sleep(time::Duration::from_secs(30000000));
    c.find(Locator::Css("#zipCodeMultiple"))
        .await?
        .select_by_value("Milano 20125")
        .await?;

    // let mut categories = c.find_all(Locator::Css(".category-list ul li")).await?;

    // let names: Vec<Option<String>> = stream::iter(&mut categories)
    //     .then(async move |el| el.text().await.ok())
    //     .collect()
    //     .await;

    // println!("{:?}", names);
    // println!("{:?}", categories);

    c.close().await
}
