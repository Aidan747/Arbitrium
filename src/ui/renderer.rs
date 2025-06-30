use std::{default, sync::LazyLock};

use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::data;
use egui_plot::{Line, Plot, PlotPoints};

#[derive(Debug, Serialize, Deserialize)]
pub enum QueuedOperation {
    CLEAR,
    EXIT,
    NOOP,
    SYMBOL(Vec<data::types::TickerDataframe>)
}
#[derive(Clone, Debug)]
pub enum AppPage {
    Home,
    DataViewer,
    TrainTest,
    TradingTerminal,
}

#[derive(Debug)]
pub struct App {
    /// Current value of the input box
    pub input: String,
    
    /// History of recorded messages
    pub messages: Vec<(String, Result<(), String>)>,

    pub queued_operation: QueuedOperation,

    pub page: AppPage,

    pub database: &'static LazyLock<Surreal<Client>>
}

impl Default for App {
    fn default() -> Self {
        Self { 
            input: String::new(),
            messages: Vec::new(),
            queued_operation: QueuedOperation::NOOP,
            page: AppPage::Home,
            database: &crate::DB
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Arbitrium App");

            ui.horizontal(|ui| {
                if ui.button("Home").clicked() {
                    self.page = AppPage::Home;
                }
                if ui.button("Data Viewer").clicked() {
                    self.page = AppPage::DataViewer;
                }
                if ui.button("Train/Test").clicked() {
                    self.page = AppPage::TrainTest;
                }
                if ui.button("Trading Terminal").clicked() {
                    self.page = AppPage::TradingTerminal;
                }
            });

            ui.separator();

            match self.page {
                AppPage::Home => {
                    ui.label("Portfolio overview");
                }
                AppPage::DataViewer => {
                    ui.label("Data Viewer page");
                }
                AppPage::TrainTest => {
                    ui.label("Train/Test page");
                }
                AppPage::TradingTerminal => {
                    ui.label("Trading Terminal page");
                }
            }

            ui.separator();

            ui.heading(format!("Total Value: ${}", 500));

            ui.horizontal(|ui| {
                // Placeholder graph using egui's plot widget

                let points = PlotPoints::from_explicit_callback(|x| x.sin(), 0.0..10.0, 100);
                let line = Line::new("test",points).name("Sine Wave");

                let available_width = ui.available_width() * 0.7;
                Plot::new("placeholder_plot")
                    .width(available_width)
                    .show(ui, |plot_ui| {
                        plot_ui.line(line);
                    });

                ui.vertical(|ui| {
                    ui.label("Portfolio Holdings");
                    ui.allocate_ui_with_layout(
                        ui.available_size() * egui::vec2(0.3, 1.0),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            egui::Grid::new("portfolio_table")
                                .striped(true)
                                .show(ui, |ui| {
                                    ui.heading("Symbol");
                                    ui.heading("Shares");
                                    ui.heading("Price");
                                    ui.heading("Value");
                                    ui.end_row();

                                    // Example static data; replace with real data as needed
                                    let holdings = vec![
                                        ("AAPL", 10, 190.0),
                                        ("GOOG", 5, 2800.0),
                                        ("TSLA", 2, 700.0),
                                    ];

                                    for (symbol, shares, price) in holdings {
                                        let value = shares as f64 * price;
                                        ui.label(symbol);
                                        ui.label(shares.to_string());
                                        ui.label(format!("{:.2}", price));
                                        ui.label(format!("{:.2}", value));
                                        ui.end_row();
                                    }
                                });
                        }
                    )
                });
            });
        });
    }
}
