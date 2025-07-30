
use chrono::{Datelike, Days, Duration, NaiveDate};
use holiday::{holiday, Holiday, HolidayDate};

use crate::data::types::{TickerData, TickerDataframe};
use crate::analysis::{self, *};
use crate::data::*;
#[derive(Debug, Clone)]
pub struct DirectionClassificationFeatures {
    // Price-based features
    pub prev_close_to_high_ratio: f32,
    pub prev_close_to_low_ratio: f32,
    pub daily_return: f32,
    pub volatility_5d: f32,
    pub volatility_20d: f32,
    
    // Volume features
    pub volume_ratio_5d: f32,   // vs 5-day average
    pub volume_ratio_20d: f32,  // vs 20-day average
    // pub volume_price_trend: f32,
    
    // Technical indicators
    pub rsi_14: f32,
    pub sma_5: f32,
    pub sma_20: f32,

    pub macd_line: f32,
    pub macd_signal: f32,
    pub macd_histogram: f32,

    pub direction: PriceDirection
}

#[derive(Debug, Clone, PartialEq)]
pub enum PriceDirection {
    Up,    // Gap up > 1%
    Down,  // Gap down < -1%
    None,  // Normal opening ±1%
}

impl PriceDirection {
    pub fn to_usize(&self) -> usize {
        match self {
            PriceDirection::Up => 2,
            PriceDirection::Down => 0,
            PriceDirection::None => 1,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            PriceDirection::Up => String::from("Up"),
            PriceDirection::Down => String::from("Down"),
            PriceDirection::None => String::from("None"),
        }
    }
    pub fn calc(open: f32, close: f32) -> Self {
        let percent_change = ((open - close) / close) * 100.0;
        
        if percent_change > 0.5 {
            return Self::Up;
        } else if percent_change < -0.5 {
            return Self::Down;
        }

        Self::None
    }
}

pub fn calculate_featureset(data: &TickerData) -> Vec<DirectionClassificationFeatures> {
    let mut features = Vec::new();

    let raw_rsi_14 = analysis::moving_average::rsi_on_series(&data.price_data, 14);
    let raw_sma_5 = analysis::moving_average::sma_on_series(&data.price_data, 5);
    let raw_sma_20 = analysis::moving_average::sma_on_series(&data.price_data, 20);

    let rsi_14 = analysis::normalization::normalize_series(raw_rsi_14, data.price_data.last().unwrap().close);
    let sma_5 = analysis::normalization::normalize_series(raw_sma_5, data.price_data.last().unwrap().close);
    let sma_20 = analysis::normalization::normalize_series(raw_sma_20, data.price_data.last().unwrap().close);
    
    let macd = analysis::moving_average::macd_on_series(&data.price_data, 12, 26);

    println!("sma 20: {}", sma_20.len());
    println!("sma 5: {}", sma_5.len());
    println!("rsi 14: {}", rsi_14.len());
    println!("macd: {}", macd.len());
    println!("data: {}", data.price_data.len());

    for (idx, el) in data.price_data.iter().enumerate() {
        if idx == 0 {continue;}

        let prev_5d = if idx > 5 {
            data.price_data.get(idx-5..).unwrap()
        } else {
            data.price_data.get(..).unwrap()
        }.to_vec();

        let prev_20d = if idx > 20 {
            data.price_data.get(idx-20..).unwrap()
        } else {
            data.price_data.get(..).unwrap()
        }.to_vec();

        let prev_frame = data.price_data.get(idx - 1).unwrap();

        let prev_chr = prev_frame.close / prev_frame.high;
        let prev_clr = prev_frame.close / prev_frame.low;

        let daily_ret = (prev_frame.close - el.close) / el.close;

        let volatility_5d = analysis::volatility::ticker_volatility_n_series(&prev_5d);
        let volatility_20d = analysis::volatility::ticker_volatility_n_series(&prev_20d);

        let vol_ratio_5d = analysis::normalization::volume_ratio(&prev_5d);
        let vol_ratio_20d = analysis::normalization::volume_ratio(&prev_20d);

        features.push(DirectionClassificationFeatures {
            prev_close_to_high_ratio: prev_chr,
            prev_close_to_low_ratio: prev_clr,
            daily_return: daily_ret,
            volatility_5d: volatility_5d,
            volatility_20d: volatility_20d,
            volume_ratio_5d: vol_ratio_5d,
            volume_ratio_20d: vol_ratio_20d,
            sma_5: *sma_5.get(idx).unwrap(),
            rsi_14: *rsi_14.get(idx).unwrap(),
            sma_20: *sma_20.get(idx).unwrap(),
            macd_line: macd.get(idx).unwrap().macd,
            macd_signal: macd.get(idx).unwrap().signal,
            macd_histogram: macd.get(idx).unwrap().macd - macd.get(idx).unwrap().signal,
            direction: PriceDirection::calc(el.open, el.close),
        });
    }

    features
}

