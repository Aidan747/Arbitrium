use std::sync::LazyLock;

use surrealdb::{engine::remote::ws::Client, Surreal};

mod ui;
mod data;


static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Arbitrium",
        options,
        Box::new(|_cc| Ok(Box::new(ui::renderer::App::default()))),
    ).unwrap();
}
