use crate::data::types::{TickerDataframe, PointTimeDelta};

pub fn sma_on_series(series: Vec<TickerDataframe>, sma_period_days: i32) -> Vec<TickerDataframe> {
    let mut ret = Vec::new();

    let recip_k = 1.0 / sma_period_days as f32;
    let n = series.len();
    let frames = n / sma_period_days as usize;

    if series.len() < sma_period_days as usize {
        let avg: f32 = series.iter()
            .map(|el| el.close)
            .sum();
        let total_vol: i64 = series.iter()
            .map(|el| {
                el.vol
            })
            .sum();
        ret.push(
            TickerDataframe {
                close: avg,
                vol: total_vol,
                ..series.get(0).unwrap().clone() 
            }
        );
        return ret;
    }

    let pd0 = recip_k * ((n - series.len() + 1)..n).into_iter().for_each(|i| {
        series.get(i).unwrap().close
    });



    ret
}