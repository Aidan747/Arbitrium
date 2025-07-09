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
    Minute(i32),
    Hour(i32),
    Day,
    Week,
    Month(i32),
}
impl ToString for PointTimeDelta {
    fn to_string(&self) -> String {
        let (val, unit) = match self {
            PointTimeDelta::Minute(val) => (val.clamp(&1, &59), self.as_ref()),
            PointTimeDelta::Hour(val) => (val.clamp(&1, &23), self.as_ref()),
            PointTimeDelta::Day => (&1, self.as_ref()),
            PointTimeDelta::Week => (&1, self.as_ref()),
            PointTimeDelta::Month(val) => {
                let checked_val = if [1,2,3,4,6,12].contains(val) {
                    val
                } else {
                    &1
                };
                (checked_val, self.as_ref())
            },
        };
        let res = format!("{val}{unit}");

        res
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TickerData {
    pub price_data: Vec<TickerDataframe>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct TickerDataframe {
    pub t: String,
    pub open: f32,
    pub high: f32,
    pub close: f32,
    pub low: f32,
    pub vol: i64,
    pub vol_weighted: f32,
}