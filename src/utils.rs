use std::io::{self, Write};

pub fn get_user_input() -> String {
    print!("Enter meme coin symbol: ");
    io::stdout().flush().unwrap();

    let mut symbol = String::new();
    io::stdin().read_line(&mut symbol).expect("Failed to read line");
    symbol.trim().to_string()
}
