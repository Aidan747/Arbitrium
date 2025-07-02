use std::{error::Error, sync::LazyLock};

use surrealdb::{engine::remote::ws::{Client, Ws, Wss}, opt::auth::Root, Surreal};

mod ui;
mod data;


static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // DB.connect::<Ws>("127.0.0.1:8000").await?;

    // DB.signin(Root {
    //     username: "root",
    //     password: "root",
    // }).await?;

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Arbitrium",
        options,
        Box::new(|_cc| Ok(Box::new(ui::renderer::App::default()))),
    ).unwrap();

    Ok(())
}
