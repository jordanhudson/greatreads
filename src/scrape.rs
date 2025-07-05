use anyhow::{Result, anyhow};
use reqwest::Client;
use scraper::{Html, Selector};
use serde_json::Value;

pub async fn scrape_book_page(book_id: u64) -> Result<String> {
    let url = format!("https://www.goodreads.com/book/show/{}", book_id);
    
    let client = Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;
    
    let body = response.text().await?;
    Ok(body)
}

pub fn extract_next_data(html: &str) -> Result<Value> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("#__NEXT_DATA__").unwrap();
    
    let script_element = document
        .select(&selector)
        .next()
        .ok_or_else(|| anyhow!("__NEXT_DATA__ script tag not found"))?;
    
    let json_text = script_element
        .inner_html();
    
    let json: Value = serde_json::from_str(&json_text)?;
    Ok(json)
}