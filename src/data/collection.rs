use reqwest::{header::{HeaderMap, HeaderValue}, Request};
use ::serde::{Deserialize, Serialize};
use lazy_static::lazy_static;
use std::result::Result;
use super::types::*;


lazy_static! {
    static ref POLYGON_API_KEY: String = {
        dotenv::dotenv().ok();
        let key = dotenv::var("POLYGON_API_KEY");
        match key {
            Ok(k) => k,
            Err(_) => {
                eprintln!("[ERROR] Polygon API Key not found, check the .env file and try again");
                std::process::exit(1)
            }
        }
    };

    static ref ALPHA_VANTAGE_API_KEY: String = {
        dotenv::dotenv().ok();
        let key = dotenv::var("ALPHA_VANTAGE_API_KEY");
        match key {
            Ok(k) => k,
            Err(_) => {
                eprintln!("[ERROR] Alpha Vantage API Key not found, check the .env file and try again");
                std::process::exit(1)
            }
        }
    };

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


pub async fn get_ticker_data(ticker: impl ToString, datatype: TickerDatatype, point_time_delta: PointTimeDelta,) -> Result<TickerData, reqwest::Error> {
    let (from, to) = match datatype {
        TickerDatatype::HistPrice(from, to) => (from, to),
        TickerDatatype::HistVolume(from, to) => (from, to),
        TickerDatatype::HistOHCL(from, to) => (from, to),
    };

    let url = format!("https://data.alpaca.markets/v2/stocks/bars");

    let client = reqwest::Client::new();
    
    let mut all_bars = Vec::new();
    let mut next_page_token: Option<String> = None;

    loop {
        let mut req = client
            .get(&url)
            .header("APCA-API-KEY-ID", ALPACA_API_KEY.as_str())
            .header("APCA-API-SECRET-KEY", ALPACA_SECRET_KEY.as_str())
            .query(&[
                ("symbols", ticker.to_string()),
                ("timeframe", point_time_delta.to_string()),
                ("start", from.clone()),
                ("end", to.clone()),
            ]);

        if let Some(ref token) = next_page_token {
            req = req.query(&[("page_token", token.clone())]);
        }

        let resp_text = req.send().await?.text().await?;
        let resp_json: serde_json::Value = serde_json::from_str(&resp_text).unwrap();

        let bars = resp_json["bars"][ticker.to_string()]
            .as_array()
            .unwrap_or(&vec![])
            .clone();

        all_bars.extend(bars);

        next_page_token = resp_json["next_page_token"].as_str().map(|s| s.to_string());
        if next_page_token.is_none() {
            break;
        }
    }

    // Build a fake JSON response with all bars for downstream code
    let mut resp = serde_json::Map::new();
    let mut bars_map = serde_json::Map::new();
    bars_map.insert(ticker.to_string(), serde_json::Value::Array(all_bars));
    resp.insert("bars".to_string(), serde_json::Value::Object(bars_map));
    let resp = serde_json::Value::Object(resp).to_string();

    let resp: serde_json::Value = serde_json::from_str(&resp).unwrap();

    let price_data = resp["bars"][ticker.to_string()]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|item| TickerDataframe {
            t: item["t"].as_str().unwrap_or("").to_string(),
            open: item["o"].as_f64().unwrap_or(0.0) as f32,
            high: item["h"].as_f64().unwrap_or(0.0) as f32,
            close: item["c"].as_f64().unwrap_or(0.0) as f32,
            low: item["l"].as_f64().unwrap_or(0.0) as f32,
            vol: if let Some(v) = item["v"].as_f64() {
                v as i64
            } else if let Some(v) = item["v"].as_str() {
                v.parse::<f64>().map(|num| num as i64).unwrap_or(0)
            } else {
                0
            },
            vol_weighted: item["vw"].as_f64().unwrap_or(0.0) as f32,
        })
        .collect();

    // println!("{}", resp);

    Ok(TickerData { price_data })
}

pub async fn get_etf_holdings(etf: Etf, n: i32) -> Result<Vec<(String, f32)>, reqwest::Error> {
    let url = format!("https://www.alphavantage.co/query?function=ETF_PROFILE&symbol={}&apikey={}", etf.as_ref(), *ALPHA_VANTAGE_API_KEY);

    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .send()
        .await?
        .text()
        .await?;

    let json: serde_json::Value = serde_json::from_str(&response).unwrap_or_default();

    let symbols = json["holdings"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|holding| {
            let symbol = holding["symbol"].as_str()?;
            let weight = holding["weight"].as_str()?.parse::<f32>().unwrap();
            Some((symbol.to_string(), weight))
        })
        .take(n as usize)
        .collect();

    Ok(symbols)
}
