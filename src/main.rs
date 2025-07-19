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
    use crate::data::{collection, types::{TickerData, TickerDataframe}};

    use super::*;

    #[tokio::test]
    pub async fn test_option_chain_fetch() {
        let ret = collection::get_options_chain("NVDA", None).await.unwrap();

        println!("{:#?}", ret);
    }
}

