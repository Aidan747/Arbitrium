use ndarray::{Array3, Shape};

use crate::data::types::{TickerData, TickerDataframe};

pub fn ticker_volatility_series(data: TickerData) -> f32 {
    let closing_prices = data.price_data.iter().map(|el| {
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

pub fn get_vix_along_data(data: TickerData) -> Vec<TickerDataframe> {
    use crate::data::collection::*;
    let dataframes = data.price_data;

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

pub fn calculate_volatility_surface(data: TickerData) -> Array3<f32> {
    let mut surface = Array3::<f32>::zeros((3, 1, 1));

    let ticker = data.symbol;
    


    surface
}