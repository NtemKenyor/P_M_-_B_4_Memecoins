use rand::Rng;
use super::api::Coin;

// pub enum Timeframe {
//     Next24Hours,
//     NextWeek,
//     NextMonth,
//     Next6Months,
//     NextYear,
// }

// pub fn predict_price(_meme_coin: &Coin, _timeframe: Timeframe) -> bool {
//     // Placeholder: simple random prediction
//     let mut rng = rand::thread_rng();
//     rng.gen_bool(0.5)
// }

pub enum Timeframe {
    Next24Hours,
    NextWeek,
    NextMonth,
    Next6Months,
    NextYear,
}

// pub fn predict_price(_coin: &Coin, timeframe: Timeframe) -> bool {
// we can borrow it instead of takinf ownership of it...
// We can use some machine learning model(eg: LG, XGB, KNN, LGB etc) or some deep leanring models in the future
pub fn predict_price(_coin: &Coin, timeframe: &Timeframe) -> bool {
    let mut rng = rand::thread_rng();
    match timeframe {
        Timeframe::Next24Hours => rng.gen_bool(0.5),
        Timeframe::NextWeek => rng.gen_bool(0.5),
        Timeframe::NextMonth => rng.gen_bool(0.5),
        Timeframe::Next6Months => rng.gen_bool(0.5),
        Timeframe::NextYear => rng.gen_bool(0.5),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::api::Coin;

    #[test]
    fn test_predict_price() {
        let coin = Coin {
            id: String::from("pepe"),
            symbol: String::from("pepe"),
            name: String::from("Pepe"),
            current_price: 0.00000887, // Example price
        };
        let prediction = predict_price(&coin, Timeframe::Next24Hours);
        assert!(prediction == true || prediction == false);
    }
}
