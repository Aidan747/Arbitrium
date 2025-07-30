
use crate::data::types::TickerDataframe;

pub struct MacdPoint {
    pub signal: f32,
    pub macd: f32,
}


pub fn sma_on_series(series: &Vec<TickerDataframe>, sma_period_days: i32) -> Vec<f32> {
    let mut ret = Vec::new();

    let recip_k = 1.0 / sma_period_days as f32;
    let n = series.len();
    let frames = n / sma_period_days as usize;

    // if series.len() < sma_period_days as usize {
    //     let sum: f32 = series.iter()
    //         .map(|el| el.close)
    //         .sum();
    //     ret.push(
    //         sum / sma_period_days as f32
    //     );
    //     return ret;
    // }

    for i in 0..sma_period_days - 1 {
        let sum: f32 = series.iter().take(i as usize).map(|el| el.close).sum();
        ret.push(
            sum / i as f32
        )
    }
    if series.len() < (sma_period_days - 1) as usize {
        return ret;
    }
    series.iter()
        .as_slice()
        .windows(sma_period_days as usize)
        .for_each(|el| {
            let window_sum: f32 = el.iter().map(|v| v.close).sum();
            ret.push(window_sum * recip_k);
        });

    ret
}

pub fn macd_on_series(series: &Vec<TickerDataframe>, period_short: i32, period_long: i32) -> Vec<MacdPoint> {
    let smoothing_short = 2.0 / (period_short as f32 + 1.0);
    let smoothing_long = 2.0 / (period_long as f32 + 1.0);
    let smoothing_signal = 2.0 / (9.0 + 1.0);

    let n = series.len();
    let mut macd_series = Vec::with_capacity(n);

    // Fill initial values with zeros (or some default value)
    for i in 0..period_long as usize - 1 {
        macd_series.push(MacdPoint { macd: 0.0, signal: 0.0 });
    }

    if n < period_long as usize {
        return macd_series;
    }

    // Calculate initial SMA for short and long periods
    let initial_sma_short: f32 = series.iter()
        .take(period_short as usize)
        .map(|df| df.close)
        .sum::<f32>() / period_short as f32;

    let initial_sma_long: f32 = series.iter()
        .take(period_long as usize)
        .map(|df| df.close)
        .sum::<f32>() / period_long as f32;

    // EMA vectors for short and long
    let mut ema_short = Vec::with_capacity(n);
    let mut ema_long = Vec::with_capacity(n);

    for i in 0..n {
        let current_price = series[i].close;

        // Short EMA
        let ema_short_value = if i < period_short as usize - 1 {
            0.0
        } else if i == period_short as usize - 1 {
            initial_sma_short
        } else {
            (current_price * smoothing_short) + (ema_short[i - 1] * (1.0 - smoothing_short))
        };
        ema_short.push(ema_short_value);

        // Long EMA
        let ema_long_value = if i < period_long as usize - 1 {
            0.0
        } else if i == period_long as usize - 1 {
            initial_sma_long
        } else {
            (current_price * smoothing_long) + (ema_long[i - 1] * (1.0 - smoothing_long))
        };
        ema_long.push(ema_long_value);
    }

    // Calculate MACD line
    let mut macd_line = Vec::with_capacity(n);
    for i in 0..n {
        let macd_value = ema_short[i] - ema_long[i];
        macd_line.push(macd_value);
    }

    // Calculate initial signal line as SMA of first 9 MACD values
    let mut signal_line = Vec::with_capacity(n);
    for i in 0..8 {
        signal_line.push(0.0);
    }
    if n >= 9 {
        let initial_signal = macd_line.iter().take(9).sum::<f32>() / 9.0;
        signal_line.push(initial_signal);

        for i in 9..n {
            let signal_val = (macd_line[i] * smoothing_signal) + (signal_line[i - 1] * (1.0 - smoothing_signal));
            signal_line.push(signal_val);
        }
    }

    // Fill output vector to match input length
    for i in 0..n {
        let macd = macd_line[i];
        let signal = if i < signal_line.len() { signal_line[i] } else { 0.0 };
        if i >= macd_series.len() {
            macd_series.push(MacdPoint { macd, signal });
        } else {
            macd_series[i] = MacdPoint { macd, signal };
        }
    }

    macd_series
}

pub fn rsi_on_series(series: &Vec<TickerDataframe>, period: usize) -> Vec<f32> {
    let mut rsis = Vec::new();
    
    if series.len() < 2 {
        return rsis;
    }

    // Calculate price differences
    let price_changes: Vec<f32> = series.windows(2)
        .map(|window| window[1].close - window[0].close)
        .collect();

    // Initialize with first possible value
    let mut avg_gain = price_changes.iter()
        .take(period)
        .map(|&x| if x > 0.0 { x } else { 0.0 })
        .sum::<f32>() / period as f32;

    let mut avg_loss = price_changes.iter()
        .take(period)
        .map(|&x| if x < 0.0 { -x } else { 0.0 })
        .sum::<f32>() / period as f32;

    // Fill initial values with None
    for _ in 0..period {
        rsis.push(50.0);
    }

    // Calculate first RSI
    if avg_loss == 0.0 {
        rsis.push(100.0);
    } else {
        let rs = avg_gain / avg_loss;
        rsis.push(100.0 - (100.0 / (1.0 + rs)));
    }

    // Calculate remaining RSI values
    for i in period..price_changes.len() {
        let gain = if price_changes[i] > 0.0 { price_changes[i] } else { 0.0 };
        let loss = if price_changes[i] < 0.0 { -price_changes[i] } else { 0.0 };

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
