use serde::{Deserialize, Serialize};
use strum::AsRefStr;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct EtfHolding {
    pub symbol: String,
    pub weight: f32
}

pub enum TickerDatatype {
    HistPrice(String, String),
    HistVolume(String, String),
    HistOHCL(String, String),
}
#[derive(AsRefStr, Debug, Clone, Copy)]
pub enum Etf {
    SPY,
    QQQ,
    QQQM,
    DIA,
}

impl FromStr for Etf {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SPY" => Ok(Etf::SPY),
            "QQQ" => Ok(Etf::QQQ),
            "QQQM" => Ok(Etf::QQQM),
            "DIA" => Ok(Etf::DIA),
            _ => Err(()),
        }
    }
}

#[derive(AsRefStr)]
pub enum PointTimeDelta {
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Quarter,
    Year,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TickerData {
    pub price_data: Vec<TickerDataframe>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TickerDataframe {
    pub open: f32,
    pub high: f32,
    pub close: f32,
    pub low: f32,
    pub vol: i64,
    pub vol_weighted: f32,
}