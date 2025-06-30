use std::{default, sync::LazyLock};

use egui::TextStyle;
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{data, ui::widgets::{self, data_controller_widget}};
use egui_plot::{Line, Plot, PlotBounds, PlotPoints};

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
            let visuals = egui::Visuals {
                override_text_color: Some(egui::Color32::WHITE),
                ..egui::Visuals::dark()
            };
            ctx.set_visuals(visuals);
            
            widgets::navbar(self, ui);

            match self.page {
                AppPage::Home => {
                    ui.heading(format!("Total Value: ${}", 500));

                    ui.add_space(2.5);
            
                    widgets::portfolio_data_viewer(self, ui);
                }
                AppPage::DataViewer => {
                    data_controller_widget(self, ui);
                }
                AppPage::TrainTest => {
                    ui.label("Train/Test page");
                }
                AppPage::TradingTerminal => {
                    ui.label("Trading Terminal page");
                }
            }

            // ui.separator();
        });
    }
}
