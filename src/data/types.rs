use chrono::{DateTime, NaiveDate, Offset, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use strum::AsRefStr;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct EtfHolding {
    pub symbol: String,
    pub weight: f32
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
    pub symbol: String,
    pub price_data: Vec<TickerDataframe>,
    pub technicals: Vec<Technicals>,
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
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Technicals {
    pub volatility_at_t: f32,
    pub sma: f32,
    pub rsi: f32,
    pub analyst_target: f32,
}
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct OptionChain {
    pub data: Vec<StockOption>
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct StockOption {
    #[serde(rename = "contractID")]
    pub contract_id: String,
    #[serde(rename = "expiration")]
    pub expiry_date: NaiveDate,
    #[serde(rename = "strike")]
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub strike_price: f32,
    #[serde(rename = "type")]
    pub option_type: OptionType,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub bid: f32,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub bid_size: f32,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub ask: f32,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub ask_size: f32,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub open_interest: f32,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub volume: f32,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub implied_volatility: f32,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub delta: f32,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub gamma: f32,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub theta: f32,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub vega: f32,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub rho: f32,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub enum OptionType {
    #[default]
    #[serde(rename = "call")]
    Call = 0,
    #[serde(rename = "put")]
    Put = 1,
}