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

    series.iter()
        .as_slice()
        .windows(sma_period_days as usize)
        .for_each(|el| {
            let window_sum: f32 = el.iter().map(|v| v.close).sum();
            ret.push(TickerDataframe {
                close: window_sum * recip_k,
                ..el[0].clone()
            });
        });

    ret
}

