use ndarray::{Array3, Shape};

use crate::data::{collection, types::{OptionChain, TickerData, TickerDataframe}};

pub fn ticker_volatility_n_series(data: &Vec<TickerDataframe>) -> f32 {
    let closing_prices = data.iter().map(|el| {
        el.close
    }).collect::<Vec<_>>();

    let avg = closing_prices.iter().sum::<f32>() / closing_prices.len() as f32;

    let squared_delta = closing_prices.iter().map(|el| {
        (el - avg).powf(2.0)
    }).collect::<Vec<_>>();

    let variance = squared_delta.iter().sum::<f32>() / closing_prices.len() as f32;

    let std_dev = variance.sqrt();

    std_dev
}

pub fn get_vix_along_data(data: &TickerData) -> Vec<TickerDataframe> {
    use crate::data::collection::*;
    let dataframes = &data.price_data;

    let mut vol_series: Vec<TickerDataframe> = Vec::new();

    futures::executor::block_on(async {
        let resp = get_ticker_data(
            "VIX",
            crate::data::types::TickerDatatype::HistPrice(dataframes.get(0).unwrap().t.clone(), dataframes.last().unwrap().t.clone()),
            crate::data::types::PointTimeDelta::Day
        ).await.unwrap();

        vol_series = resp.price_data;
    });
    
    vol_series.clone()
}

pub fn calculate_volatility_surface(options: &OptionChain) -> Array3<f32> {
    let mut surface = Array3::<f32>::zeros((3, 1, 1));

    let ticker = options.data.get(0).unwrap().contract_id
        .chars()
        .take_while(|c| !c.is_ascii_digit() || c.is_ascii_digit() && c == &'0')
        .collect::<String>();

    let ivs = options.data
        .iter()
        .map(|el| el.implied_volatility)
        .collect::<Vec<_>>();
    
    let time_to_exp = options.data
        .iter()
        .map(|el| {
            let expiration = el.expiry_date;
            let delta = expiration.signed_duration_since(expiration);
            delta.num_days()
        })
        .collect::<Vec<_>>();

    let moneyness = options.data
        .iter()
        .map(|el| {
            // let market_price = 
            0.0
        })
        .collect::<Vec<_>>();

    surface
}

