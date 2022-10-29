use std::env;

use dotenv::dotenv;
use reqwest::header;
use scraper::{Html, Selector};
use serde_json::json;

async fn fetch_html() -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://item.rakuten.co.jp/realforce/r3hd11/?s-id=bk_pc_item_list_name_n";
    // let url = "https://item.rakuten.co.jp/realforce/r3hh11/?s-id=bk_pc_item_list_name_n";
    let body = reqwest::get(url).await?.text().await?;

    Ok(body)
}

async fn is_sold_out() -> Status {
    let html = fetch_html().await.unwrap();
    let document = Html::parse_document(&html);
    let selector_str = "#normal_basket_10000032 tbody tr:nth-child(2) td span";
    let selector = Selector::parse(selector_str).unwrap();

    let mut result: Status = Status::IsSold;

    for element in document.select(&selector) {
        let unicode = element.text().next();
        println!("{:?}", unicode);
        match unicode {
            Some(_) => result = Status::IsSoldOut,
            None => (),
        };
    }
    result
}

async fn send_message() {
    dotenv().ok();
    let token = env::var("TOKEN").expect("TOKEN must be set");
    let url = "https://api.line.me/v2/bot/message/broadcast";
    let authorization = &("Bearer ".to_string() + &token.to_owned());
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Authorization", authorization.parse().unwrap());

    let body = json!({
        "messages":[
            {
                "type": "text",
                "text": "商品が入荷しました。\n
    https://item.rakuten.co.jp/realforce/r3hd11/?s-id=bk_pc_item_list_name_n"
            }
        ]
    });

    let client = reqwest::Client::new();
    let _res = client
        .post(url)
        .json(&body)
        .headers(headers)
        .send()
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    let status = is_sold_out().await;

    match status {
        Status::IsSold => {
            send_message().await;
        }
        Status::IsSoldOut => (),
    }
}

#[derive(Debug)]
enum Status {
    IsSold,
    IsSoldOut,
}
