# P_M_-_B_4_Memecoins
Prediction Market &amp; Blink for Memecoins

# Solana Meme Price Prediction

Predict the price trend of meme coins over different timeframes using Rust!

## Overview

This project demonstrates how to fetch data from the CoinGecko API, predict the price trends of meme coins, and interactively select coins and timeframes for prediction.

### Features

- Fetches live data of meme coins from CoinGecko API.
- Predicts price trends over timeframes ranging from 24 hours to 1 year.
- Allows user interaction to select coins based on symbol input.

## Setup

To run this project locally, follow these steps:

1. **Clone the repository:**

   ```bash
   git clone https://github.com/your-username/solana-meme-prediction.git
   cd solana-meme-prediction
   ```

2. **Install Rust (if not already installed):**

   Follow the instructions on [rust-lang.org](https://www.rust-lang.org/tools/install) to install Rust and Cargo.

3. **Install dependencies:**

   Ensure you have `cargo` installed. Navigate to the project directory and run:

   ```bash
   cargo build
   ```

4. **Run the project:**

   ```bash
   cargo run
   ```

5. **Input your selection:**

   Follow the prompts to select a meme coin by its symbol and observe the price predictions over different timeframes.

## Dependencies

- `reqwest`: For making HTTP requests to the CoinGecko API.
- `serde`: For serializing and deserializing JSON data.
- `rand`: For generating random selections of meme coins.
- `tokio`: Provides async runtime for handling async operations.

## Usage

1. **Select a meme coin:**

   Enter the symbol of the meme coin when prompted. For example, enter `DOGE` for Dogecoin.

2. **View predictions:**

   The program will predict the price trend of the selected meme coin over various timeframes (24 hours, 1 week, 1 month, 6 months, 1 year).

3. **Interpret the results:**

   Based on the predictions (`true` for price increase, `false` for price decrease), analyze the future price trends of the selected meme coin.

## Contributing

Contributions are welcome! Feel free to open issues or pull requests for any improvements or features you'd like to see in this project.

1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/new-feature`).
3. Commit your changes (`git commit -am 'Add new feature'`).
4. Push to the branch (`git push origin feature/new-feature`).
5. Open a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

