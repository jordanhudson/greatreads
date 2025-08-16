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

async fn test_scrape(book_id: u64) -> Result<(), Box<dyn std::error::Error>> {
    let html = scrape_book_page(book_id).await?;
    let json = extract_next_data(&html)?;
    println!("{}", serde_json::to_string_pretty(&json)?);
    Ok(())
}
