use reqwest::Error;
use serde::Deserialize;
use serde_json; // Add this line

#[derive(Deserialize, Debug)]
pub struct Coin {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub current_price: f64,
}

pub async fn fetch_meme_coins() -> Result<Vec<Coin>, Box<dyn std::error::Error>> {
    let url = "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&category=meme-token";
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "solana_meme_prediction/0.1.0")
        .send()
        .await?;
    let response_text = response.text().await?;
    
    // Print raw response for debugging
    println!("Raw response: {}", response_text);
    
    let coins: Vec<Coin> = serde_json::from_str(&response_text)?;
    Ok(coins)
}
