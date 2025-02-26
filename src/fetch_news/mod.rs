// fetch_news.rs

use tokio::task;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::{UpdateDetails, Vector};

#[derive(Deserialize, Debug)]
struct NewsResponse {
    updates: Vec<UpdateDetails>,
}

pub fn get_news_vector() -> Vector<UpdateDetails> {
    let client = Client::new();
    let response = client.get("https://raw.githubusercontent.com/SanseLGUH/scp-zone-launcher-website-data/refs/heads/main/news.json").send().unwrap();
    let news_response: NewsResponse = response.json().unwrap();

    news_response.updates.into()  // You can convert this Vec<UpdateDetails> into a Vector<UpdateDetails> later if needed
}