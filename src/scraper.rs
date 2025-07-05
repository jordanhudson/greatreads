use anyhow::Result;
use redis::{AsyncCommands, Client};
use tokio::time::{sleep, Duration};

use crate::scrape::scrape_book_page;

pub async fn run() -> Result<()> {
    println!("Starting scraper worker...");
    
    let client = Client::open("redis://127.0.0.1:6379")?;
    let mut con = client.get_multiplexed_async_connection().await?;
    
    loop {
        match con.lpop::<_, Option<u64>>("book_queue", None).await {
            Ok(Some(book_id)) => {
                println!("Processing book ID: {}", book_id);
                
                match scrape_book_page(book_id).await {
                    Ok(html) => {
                        println!("Successfully scraped book {}, HTML length: {}", book_id, html.len());
                        // TODO: Store in database
                    }
                    Err(e) => {
                        eprintln!("Error scraping book {}: {}", book_id, e);
                        // TODO: Handle retry logic or dead letter queue
                    }
                }
            }
            Ok(None) => {
                println!("Queue empty, sleeping for 1 second...");
                sleep(Duration::from_secs(1)).await;
            }
            Err(e) => {
                eprintln!("Redis error: {}", e);
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}