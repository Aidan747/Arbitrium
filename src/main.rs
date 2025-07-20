use std::{error::Error, sync::LazyLock};

use surrealdb::{engine::remote::ws::{Client, Ws, Wss}, opt::auth::Root, Surreal};

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
    use crate::data::{collection, db_service, types::{TickerData, TickerDataframe}};

    use super::*;

    #[tokio::test]
    pub async fn test_option_chain_fetch() {
        let ret = collection::get_options_chain("NVDA", None).await.unwrap();

        println!("{:#?}", ret);
    }

    #[tokio::test]
    pub async fn test_etf_db_fetch() {
        db_service::init_database().await.unwrap();
        let ret = db_service::get_etf(data::types::Etf::SPY).await.unwrap();

        println!("{:#?}", ret);
    }
}

