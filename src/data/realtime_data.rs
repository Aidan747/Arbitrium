use lazy_static::lazy_static;

lazy_static! {
    static ref ALPACA_API_KEY: String = {
        dotenv::dotenv().ok();
        let key = dotenv::var("ALPACA_API_KEY");
        match key {
            Ok(k) => k,
            Err(e) => {
                eprintln!("[ERROR] Alpaca API key not found, check the .env file and try again");
                std::process::exit(1)
            }
        }
    };

    static ref ALPACA_SECRET_KEY: String = {
        dotenv::dotenv().ok();
        let key = dotenv::var("ALPACA_SECRET_KEY");
        match key {
            Ok(k) => k,
            Err(_) => {
                eprintln!("[ERROR] Alpaca Secret Key not found, check the .env file and try again");
                std::process::exit(1)
            }
        }
    };
}


pub async fn stream_market_data() {
    
}