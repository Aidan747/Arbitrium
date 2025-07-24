use crate::data::types::TickerDataframe;

pub fn volume_ratio(data: &Vec<TickerDataframe>) -> f32 {
    let mut up_total = 0;
    let mut down_total = 0;

    for el in data {
        if el.open > el.close {
            down_total += el.vol;
        } else {
            up_total += el.vol;
        }
    }

    up_total as f32 / down_total as f32
}

pub fn normalize_series(series: Vec<f32>, cur_value: f32) -> Vec<f32> {
    let ret = series.iter().map(|el| {
        el / cur_value
    }).collect::<Vec<f32>>();

    ret
}