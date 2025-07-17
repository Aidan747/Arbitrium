use std::{error::Error, sync::LazyLock};

use surrealdb::{engine::remote::ws::{Client, Ws, Wss}, opt::auth::Root, Surreal};

mod ui;
mod data;
mod analysis;

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    DB.connect::<Ws>("127.0.0.1:8000").await?;

    DB.signin(Root {
        username: "root",
        password: "root",
    }).await?;

    // analysis::hmm::train_hmm_on_symbol(&DB, "SPY".to_string());

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
    use crate::data::types::{TickerData, TickerDataframe};

    use super::*;

    #[tokio::test]
    async fn test_hmm_training() {
        // let test_data = data::collection::get_ticker_data(
        //     "SPY",
        //     data::types::TickerDatatype::HistOHCL(String::from("2021-01-01"), String::from("2025-01-01")),
        //     data::types::PointTimeDelta::Day
        // ).await.unwrap().price_data;

        // let ticker_train_data: Vec<TickerDataframe> = test_data.iter().take(test_data.len() - 10).cloned().collect();
        // let validation_data = test_data.get((test_data.len() - 10)..).unwrap();

        // let model = analysis::hmm::StockHMM::new(&ticker_train_data, 100);

        // model.print_model_info();

        // let prediction_data = model.predict_next_movements(9);

        // let validation_changes = analysis::hmm::StockHMM::calculate_price_changes(validation_data);
        // let validation_data = analysis::hmm::StockHMM::discretize_price_changes(&validation_changes);

        // assert_eq!(prediction_data.len(), validation_data.len());
        // assert_eq!(prediction_data, validation_data);

    }
}

