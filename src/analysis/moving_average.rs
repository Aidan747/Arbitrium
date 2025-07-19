use chrono::NaiveDate;

use crate::data::types::{TickerDataframe, PointTimeDelta};

pub struct MacdPoint {
    pub signal: f32,
    pub macd: f32,
}


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

pub fn macd_on_series(series: Vec<TickerDataframe>, period_short: i32, period_long: i32) -> Vec<MacdPoint> {
    let smoothing_short = 2.0 / (period_short as f32 + 1.0);
    let smoothing_long = 2.0 / (period_long as f32 + 1.0);
    let smoothing_signal = 2.0 / (9.0 + 1.0);

    if series.len() < period_long as usize {
        return Vec::new();
    }

    let mut ema_short = Vec::new();
    let mut ema_long = Vec::new();
    let mut signal = Vec::new();
    let mut macd_series = Vec::new();

    let intial_signal: f32 = series.iter()
        .take(9)
        .map(|df| df.close)
        .sum::<f32>() / 9 as f32;

    let initial_sma_short: f32 = series.iter()
        .take(period_short as usize)
        .map(|df| df.close)
        .sum::<f32>() / period_short as f32;

    let initial_sma_long: f32 = series.iter()
        .take(period_long as usize)
        .map(|df| df.close)
        .sum::<f32>() / period_long as f32;

    for (i, dataframe) in series.iter().enumerate() {
        let current_price = dataframe.close;

        let ema_short_value = if i < period_short as usize {
            if i == period_short as usize - 1 {
                initial_sma_short
            } else {
                continue;
            }
        } else {
            (current_price * smoothing_short) + (*ema_short.last().unwrap() * (1.0 - smoothing_short))
        };
        ema_short.push(ema_short_value);

        let ema_long_value = if i < period_long as usize {
            if i == period_long as usize - 1 {
                initial_sma_long
            } else {
                continue;
            }
        } else {
            (current_price * smoothing_long) + (*ema_long.last().unwrap() * (1.0 - smoothing_long))
        };
        ema_long.push(ema_long_value);

        let signal_val = if i < 9 as usize {
            if i == 9 as usize - 1 {
                intial_signal
            } else {
                continue;
            }
        } else {
            (current_price * smoothing_signal) + (*signal.last().unwrap() * (1.0 - smoothing_signal))
        };
        signal.push(signal_val);

        if ema_short.len() > 0 && ema_long.len() > 0 {
            let macd_value = ema_short_value - ema_long_value;
            macd_series.push(MacdPoint {
                macd: macd_value,
                signal: signal_val,
            });
        }
    }

    macd_series
}

pub fn rsi_on_series(series: Vec<TickerDataframe>) -> Vec<f32> {
    let period = 14;
    let mut rsis = Vec::new();

    if series.len() <= period {
        return rsis;
    }

    let mut gains = 0.0;
    let mut losses = 0.0;

    for i in 1..=period {
        let diff = series[i].close - series[i - 1].close;
        if diff > 0.0 {
            gains += diff;
        } else {
            losses -= diff;
        }
    }
    let mut avg_gain = gains / period as f32;
    let mut avg_loss = losses / period as f32;

    if avg_loss == 0.0 {
        rsis.push(100.0);
    } else {
        let rs = avg_gain / avg_loss;
        rsis.push(100.0 - (100.0 / (1.0 + rs)));
    }

    for i in (period + 1)..series.len() {
        let diff = series[i].close - series[i - 1].close;
        let gain = if diff > 0.0 { diff } else { 0.0 };
        let loss = if diff < 0.0 { -diff } else { 0.0 };

        avg_gain = (avg_gain * (period as f32 - 1.0) + gain) / period as f32;
        avg_loss = (avg_loss * (period as f32 - 1.0) + loss) / period as f32;

        if avg_loss == 0.0 {
            rsis.push(100.0);
        } else {
            let rs = avg_gain / avg_loss;
            rsis.push(100.0 - (100.0 / (1.0 + rs)));
        }
    }

    rsis
}
