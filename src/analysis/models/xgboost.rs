use std::sync::LazyLock;

use gbdt::config::Config;
use surrealdb::{engine::remote::ws::Client, Surreal};

pub async fn fit_decision_trees(db: LazyLock<Surreal<Client>>) {
    db.use_ns("TickerData").use_db("ETFs").await;
    
}