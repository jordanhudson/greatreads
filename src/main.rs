mod scrape;
mod api;
mod scraper;
mod admin;

use clap::{Parser, Subcommand};
use scrape::{scrape_book_page, extract_next_data};

#[derive(Parser)]
#[command(name = "greatreads")]
#[command(about = "A Goodreads scraper and API")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Api,
    Scraper,
    Admin,
    Test { book_id: u64 },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Api => api::run().await?,
        Commands::Scraper => scraper::run().await?,
        Commands::Admin => admin::run().await?,
        Commands::Test { book_id } => test_scrape(book_id).await?,
    }
    Ok(())
}

async fn save_book(key: &str, value: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    println!("Saving Book: {}", key);
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

async fn save_contributor(key: &str, value: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    println!("Saving Contributor: {}", key);
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

async fn save_work(key: &str, value: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    println!("Saving Work: {}", key);
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

async fn test_scrape(book_id: u64) -> Result<(), Box<dyn std::error::Error>> {
    let raw_html = scrape_book_page(book_id).await?;
    let raw_page_data = extract_next_data(&raw_html)?;
    
    let apollo_state = raw_page_data
        .get("props")
        .and_then(|props| props.get("pageProps"))
        .and_then(|page_props| page_props.get("apolloState"));
    
    match apollo_state {
        Some(state) => {
            if let Some(obj) = state.as_object() {
                for (key, value) in obj {
                    if key.starts_with("Book") {
                        save_book(key, value).await?;
                    } else if key.starts_with("Contributor") {
                        save_contributor(key, value).await?;
                    } else if key.starts_with("Work") {
                        save_work(key, value).await?;
                    }
                }
            } else {
                println!("apolloState is not an object");
            }
        },
        None => println!("apolloState not found in page data"),
    }
    
    Ok(())
}
