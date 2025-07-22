use std::{error::Error, sync::LazyLock};

use chrono::{Datelike, Days, Duration, NaiveDate};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws}, opt::auth::{Record, Root}, RecordId, Surreal
};

use crate::data::{db_service::etf_tables::{HIST_PRICE_DATA, STOCK, TECHNICALS}, types::*};

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

lazy_static! {
    static ref LAST_WEEKDAY: NaiveDate = {
        let today = chrono::Local::now().date_naive();
        let last_weekday = if today.weekday().num_days_from_monday() > 4 {
            today.checked_sub_days(Days::new(today.weekday().num_days_from_monday() as u64 - 4)).unwrap()
        } else {
            today
        };

        last_weekday
    };
}

mod etf_tables {
    pub const HIST_PRICE_DATA: &str = "historical_price_data";
    pub const TICKER: &str = "Ticker";
    pub const TECHNICALS: &str = "technicals";
    pub const STOCK: &str = "stock";
    pub const TRADE: &str = "trade";
    pub const ETF_HOLDING: &str = "etf_holding";
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct DbHistEntries {
    close: Vec<f32>,
    high: Vec<f32>,
    low: Vec<f32>,
    open: Vec<f32>,
    volume: Vec<i64>,
    volume_weighted: Vec<f32>,
}

pub async fn init_database() -> Result<(), surrealdb::Error> {
    DB.connect::<Ws>("127.0.0.1:8000").await?;

    DB.signin(Root {
        username: "root",
        password: "root",
    }).await?;

    Ok(())
}

pub async fn insert_etf(etf: Etf, data: TickerData) -> Result<(), Box<dyn Error>> {
    // for entry in data.price_data.iter() {
    //     let entry: TickerDataframe = entry.clone();
    //     let res: Option<TickerDataframe> = app.database
    //         .upsert(("PriceData", &entry.t))
    //         .merge(entry)
    //         .await
    //         .unwrap();
    //     println!("{:#?}", res);
    // }

    use etf_tables::*;



    // let data = super::collection::get_ticker_data(
    //     etf.as_ref(),
    //     TickerDatatype::HistOHCL("2016-01-01".to_string(), LAST_WEEKDAY.to_string()),
    //     PointTimeDelta::Day
    // ).await?;

    // println!("{:#?}", data);

    let holdings = super::collection::get_etf_holdings(etf, 500).await?;

    DB.use_ns("ticker_data").use_db("etfs").await?;

    let hist_data = data.price_data;

    let etf_components = holdings
        .iter()
        .map(|el| {
            RecordId::from((STOCK, &el.0))
        })
        .collect();

    let hist_data: Option<DbHistEntries> = DB
        .upsert((HIST_PRICE_DATA, &data.symbol))
        .content(DbHistEntries {
            close: hist_data.iter().map(|el| el.close).collect(),
            high: hist_data.iter().map(|el| el.high).collect(),
            low: hist_data.iter().map(|el| el.low).collect(),
            open: hist_data.iter().map(|el| el.open).collect(),
            volume: hist_data.iter().map(|el| el.vol).collect(),
            volume_weighted: hist_data.iter().map(|el| el.vol_weighted).collect(),
        })
        .await?;

    // println!("{:#?}", hist_data);

    let ticker: Option<EtfTicker> = DB
        .upsert(("Ticker", &data.symbol))
        .merge(EtfTicker {
            symbol: data.symbol.clone(),
            components: etf_components,
            current_market_price: 0.0,
            historical_price_data: RecordId::from((HIST_PRICE_DATA, data.symbol.clone())),
            technicals: RecordId::from((TECHNICALS, data.symbol.clone()))
        })
        .await?;

    // println!("{:#?}", ticker);

    Ok(())
}

pub async fn populate_stock(symbol: String) -> Result<(), Box<dyn Error>> {
    DB.use_ns("ticker_data").use_db("etfs").await?;

    let data = super::collection::get_ticker_data(
        symbol,
        TickerDatatype::HistOHCL("2016-01-01".to_string(), LAST_WEEKDAY.to_string()),
        PointTimeDelta::Day
    ).await?;

    let ticker: Option<StockTicker> = DB
        .upsert((STOCK, &data.symbol))
        .merge(StockTicker {
            symbol: data.symbol.clone(),
            historical_price_data: RecordId::from((HIST_PRICE_DATA, data.symbol.clone())),
            technicals: RecordId::from((TECHNICALS, data.symbol.clone())),
            market_price: 0.0
        }).await?;

    Ok(())
}

pub async fn get_ticker(symbol: String) -> Result<StockTicker, Box<dyn Error>> {
    DB.use_ns("ticker_data").use_db("etfs").await?;

    let mut results = DB.query(format!("
        SELECT * FROM {STOCK}:{symbol}
    ")).await?;

    let ticker: Option<StockTicker> = results.take(0).unwrap();

    Ok(ticker.unwrap())
}

pub async fn get_etf(etf: Etf) -> Result<TickerData, Box<dyn Error>> {
    DB.use_ns("ticker_data").use_db("etfs").await?;

    let symbol = etf.as_ref();

    let mut data = DB.query(format!("
        SELECT historical_price_data FROM Ticker:{symbol}
    "))
    .query(format!("
        SELECT technicals FROM Ticker:{symbol}
    ")).await?;

    println!("{:#?}", data);

    let decoded_price_data: Option<DbHistEntries> = data.take(0)?;

    let price_data = if let Some(hist) = decoded_price_data {
        let len = hist.close.len();
        let mut frames = Vec::with_capacity(len);
        for i in 0..len {
            let date = NaiveDate::from_ymd_opt(2016, 1, 1)
                .unwrap()
                .checked_add_days(Days::new(i as u64))
                .unwrap();
            frames.push(TickerDataframe {
                close: hist.close[i],
                high: hist.high[i],
                low: hist.low[i],
                open: hist.open[i],
                vol: hist.volume[i],
                vol_weighted: hist.volume_weighted[i],
                t: date.to_string(),
            });
        }
        frames
    } else {
        Vec::new()
    };

    let technicals: Option<Vec<Technicals>> = data.take(1)?;


    let ticker_data = TickerData {
        symbol: symbol.to_string(),
        price_data: price_data,
        technicals: technicals.unwrap(),
    };
    
    Ok(ticker_data)
}
