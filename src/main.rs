/* use std::io::{self, Write}; // Add Write for flushing stdout
mod api;
mod prediction;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let coins = api::fetch_meme_coins().await?;
    println!("Fetched meme coins data: {:?}", coins);

    println!("Enter meme coin symbol: ");
    let mut symbol = String::new();
    io::stdin().read_line(&mut symbol)?;
    let symbol = symbol.trim();

    let selected_coin = coins.iter().find(|&coin| coin.symbol == symbol).expect("Coin not found");

    // Display timeframe options
    println!("Select prediction timeframe:");
    println!("1. Next 24 hours");
    println!("2. Next week");
    println!("3. Next month");
    println!("4. Next 6 months");
    println!("5. Next year");

    // Read user input for timeframe
    let mut timeframe_input = String::new();
    io::stdin().read_line(&mut timeframe_input)?;
    let timeframe_input = timeframe_input.trim();

    let timeframe = match timeframe_input {
        "1" => prediction::Timeframe::Next24Hours,
        "2" => prediction::Timeframe::NextWeek,
        "3" => prediction::Timeframe::NextMonth,
        "4" => prediction::Timeframe::Next6Months,
        "5" => prediction::Timeframe::NextYear,
        _ => {
            println!("Invalid selection, defaulting to Next 24 Hours.");
            prediction::Timeframe::Next24Hours
        }
    };

    let prediction = prediction::predict_price(selected_coin, timeframe);
    println!("Prediction for {}: {}", selected_coin.name, prediction);

    Ok(())
}



#[tokio::main]
async fn main() {
    run_demo().await;
}


 */

 // Using Blink Now...

mod api;
mod prediction;
mod blink;

#[tokio::main]
async fn main() {
    // Optionally, you can run the blink demo from here
    blink::run_demo().await;
}
