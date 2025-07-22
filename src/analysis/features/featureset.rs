use std::str::FromStr;

use chrono::{Datelike, Days, Duration, NaiveDate};
use holiday::Month::{January, June, March, May};
use holiday::{holiday, Holiday, HolidayDate};

use crate::data::types::{TickerData, TickerDataframe};
use crate::analysis::*;
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
}

