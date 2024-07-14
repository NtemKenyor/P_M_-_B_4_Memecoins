use crate::api::{fetch_meme_coins, Coin};
use std::io::{self, Write}; // Add Write for flushing stdout
use crate::prediction::{predict_price, Timeframe};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub async fn run_demo() {
    // Fetch the list of coins from CoinGecko API
    let coins = fetch_meme_coins().await.unwrap();

    // Select a coin at random
    // let mut rng = thread_rng();
    // let selected_coin = coins.choose(&mut rng).unwrap();

    // We can also ask the user to sselect
    // Ask the user to select a coin
    println!("Enter meme coin symbol: ");
    let mut symbol = String::new();
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately
    io::stdin().read_line(&mut symbol).unwrap(); // Await here for async read

    let symbol = symbol.trim();
    let selected_coin = coins.iter().find(|&coin| coin.symbol == symbol)
        .expect("Coin not found");

    // Define the timeframes for demonstration
    let timeframes = vec![
        Timeframe::Next24Hours,
        Timeframe::NextWeek,
        Timeframe::NextMonth,
        Timeframe::Next6Months,
        Timeframe::NextYear,
    ];

    // Run predictions for each timeframe and print the results
    for timeframe in &timeframes {
        let prediction = predict_price(selected_coin, timeframe);
        let timeframe_str = match timeframe {
            Timeframe::Next24Hours => "Next 24 hours",
            Timeframe::NextWeek => "Next week",
            Timeframe::NextMonth => "Next month",
            Timeframe::Next6Months => "Next 6 months",
            Timeframe::NextYear => "Next year",
        };
        println!("Prediction for {} over {}: {}", selected_coin.name, timeframe_str, prediction);
    }
}
