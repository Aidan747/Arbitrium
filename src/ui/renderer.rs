use std::{default, sync::LazyLock};

use chrono::NaiveDate;
use egui::TextStyle;
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{data, ui::widgets::{self, data_controller_widget}};
use egui_plot::{Line, Plot, PlotBounds, PlotPoints};

#[derive(Debug, Clone, Copy)]
pub struct DataPageState {
    pub symbol_is_etf: bool,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
}

impl Default for DataPageState {
    fn default() -> Self {
        Self { 
            symbol_is_etf: false,
            from_date: NaiveDate::default(),
            to_date: NaiveDate::default(),
        }
    }
}

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
    DataViewer(DataPageState),
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
            let visuals = egui::Visuals {
                override_text_color: Some(egui::Color32::WHITE),
                ..egui::Visuals::dark()
            };
            ctx.set_visuals(visuals);
            
            widgets::navbar(self, ui);

            let page = std::mem::replace(&mut self.page, AppPage::Home);
            
            match page {
                AppPage::Home => {
                    ui.heading(format!("Total Value: ${}", 500));

                    ui.add_space(2.5);
            
                    widgets::portfolio_data_viewer(self, ui);
                    self.page = AppPage::Home;
                }
                AppPage::DataViewer(mut data) => {
                    data_controller_widget(self, &mut data, ui);
                    self.page = AppPage::DataViewer(data);
                }
                AppPage::TrainTest => {
                    ui.label("Train/Test page");
                    self.page = AppPage::TrainTest;
                }
                AppPage::TradingTerminal => {
                    ui.label("Trading Terminal page");
                    self.page = AppPage::TradingTerminal;
                }
            }

            // ui.separator();
        });
    }
}
