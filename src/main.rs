use std::error::Error;


mod ui;
mod data;
mod analysis;



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // analysis::hmm::train_hmm_on_symbol(&DB, "SPY".to_string());

    data::db_service::init_database().await?;

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Arbitrium",
        options,
        Box::new(|_cc| Ok(Box::new(ui::renderer::App::default()))),
    ).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use core::num;
    use std::{thread::sleep, time::Duration};

    use linfa::{prelude::ToConfusionMatrix, traits::Predict};

    use crate::{analysis::features::featureset::{self, PriceDirection}, data::{collection, db_service, types::{TickerData, TickerDataframe}}};

    use super::*;

    #[tokio::test]
    async fn test_model_training_spy() {
        use crate::data::types::*;

        data::db_service::init_database().await.unwrap();

        let train_data = db_service::get_etf(Etf::SPY).await.unwrap().price_data.iter().take(10000).cloned().collect::<Vec<TickerDataframe>>();
        let test_data = collection::get_ticker_data(
            "SPY",
            TickerDatatype::HistOHCL("2025-01-02".to_string(), "2025-05-30".to_string()),
            PointTimeDelta::Day
        ).await.unwrap();

        let train_features = analysis::features::featureset::calculate_featureset(&TickerData { symbol: "SPY".to_string(), price_data: train_data, technicals: Vec::new() });
        let test_features = analysis::features::featureset::calculate_featureset(&test_data);

        let model = analysis::strategies::gradient_trees::train(&train_features).unwrap();

        let (test_feats, test_targs) = analysis::strategies::gradient_trees::prepare_dataset(&test_features);

        let test_dataset = linfa::Dataset::new(test_feats, test_targs).with_feature_names(vec![
            "close/high ratio",
            "close/low ratio",
            "daily return",
            "volatility 5d",
            "volatility 20d",
            "volume ratio 5d",
            "volume ratio 20d",
            "rsi 14",
            "sma 5",
            "sma 20",
            "macd line",
            "macd signal",
            "macd histogram",
        ]);

        let output = model.predict(&test_dataset);

        let con_matrix = output.confusion_matrix(&test_dataset.targets).unwrap();
        let accuracy = con_matrix.accuracy();
        let corr_with_p_val = test_dataset.pearson_correlation_with_p_value(100);

        println!("accuracy: {accuracy}");
        println!();
        println!("P-Value on test_data: \n {corr_with_p_val}");
        println!("{:#?}", con_matrix);

        println!("------------------------------------");
        let price_changes: Vec<f32> = test_data.price_data
            .iter()
            .map(|el| ((el.open - el.close) / el.close) * 100.0)
            .collect();

        let predictions = output.into_raw_vec();

        let price_strs: Vec<String> = price_changes.iter().map(|v| format!("{:.2}", v)).collect();
        let pred_strs: Vec<String> = predictions.iter().map(|v| format!("{}", v)).collect();

        let max_width = price_strs.iter()
            .chain(pred_strs.iter())
            .map(|s| s.len())
            .max()
            .unwrap_or(0);

        let mut idx = 0;
        let total = price_strs.len();
        while idx < total {
            let end = usize::min(idx + 10, total);

            // Print price changes centered
            for s in &price_strs[idx..end] {
                print!("{:^width$} ", s, width = max_width);
            }
            println!();

            // Print predictions centered - fix: don't subtract 1
            // print!("{:^width$} ", "", width = max_width);
            for i in idx..end {
                if i == 0 {
                    if idx == 0 {
                        print!("{:^width$} ", "", width = max_width);
                    } // Empty space for first position
                    continue;
                }
                let pred_str = &pred_strs[i-1];
                print!("{:^width$} ", pred_str, width = max_width);
            }
            println!();

            // Print validation marks
            // print!("{:^width$} ", "", width = max_width);
            for i in idx..end {
                let price_change = price_changes[i];
                if i == 0 {
                    if idx == 0 {
                        print!("{:^width$} ", "", width = max_width);
                    }
                     // Empty space for first position
                    continue;
                }
                let el = &pred_strs[i-1];
                let matches = if price_change > 0.5 && el == "Up" {
                    "✓"
                } else if price_change < -0.5 && el == "Down" {
                    "✓"
                } else if price_change < 0.5 && price_change > -0.5 && el == "None" {
                    "✓"
                } else {
                    "✗"
                };
                print!("{:^width$} ", matches, width = max_width);
            }
            println!();
            println!();

            idx += 10;
        }
        println!();
        println!("------------------------------------");
        println!();

        // for i in 0..output.into_raw_vec().len() {
        //     let price_el = test_data.price_data.get(i).unwrap();

        //     print!("  {}  \n  {}", ((price_el.open - price_el.close) / price_el.close) * 100.0);
        // }          

    }

    #[tokio::test]
    async fn test_ensemble_trees_spy() {
        use crate::data::types::*;

        data::db_service::init_database().await.unwrap();

        let train_data = db_service::get_etf(Etf::SPY).await.unwrap().price_data.iter().take(10000).cloned().collect::<Vec<TickerDataframe>>();
        let test_data = collection::get_ticker_data(
            "SPY",
            TickerDatatype::HistOHCL("2025-01-02".to_string(), "2025-05-30".to_string()),
            PointTimeDelta::Day
        ).await.unwrap();

        let train_features = analysis::features::featureset::calculate_featureset(&TickerData { symbol: "SPY".to_string(), price_data: train_data, technicals: Vec::new() });
        let test_features = analysis::features::featureset::calculate_featureset(&test_data);
        let model = analysis::strategies::gradient_trees::train_ensemble(&train_features).unwrap();
        let (test_feats, test_targs) = analysis::strategies::gradient_trees::prepare_dataset(&test_features);

        let test_dataset = linfa::Dataset::new(test_feats, test_targs).with_feature_names(vec![
            "close/high ratio",
            "close/low ratio",
            "daily return",
            "volatility 5d",
            "volatility 20d",
            "volume ratio 5d",
            "volume ratio 20d",
            "rsi 14",
            "sma 5",
            "sma 20",
            "macd line",
            "macd signal",
            "macd histogram",
        ]);
        
        let output = model.predict(&test_dataset);

        println!("Acuracy: {}", output.confusion_matrix(&test_dataset.targets).unwrap().accuracy())
    }
}

