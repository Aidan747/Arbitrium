use egui::Ui;
use egui_plot::{Line, Plot, PlotPoints};
use crate::ui::renderer::AppPage;

use super::renderer::App;

pub fn navbar(app: &mut App, ui: &mut Ui) {
    ui.heading("Arbitrium App");

    ui.horizontal(|ui| {
        if ui.button("Home").clicked() {
            app.page = AppPage::Home;
        }
        if ui.button("Data Viewer").clicked() {
            app.page = AppPage::DataViewer;
        }
        if ui.button("Train/Test").clicked() {
            app.page = AppPage::TrainTest;
        }
        if ui.button("Trading Terminal").clicked() {
            app.page = AppPage::TradingTerminal;
        }
    });

    ui.separator();
}


pub fn portfolio_data_viewer(app: &mut App, ui: &mut Ui) {
    ui.allocate_ui_with_layout(
        egui::Vec2 { x: ui.available_width(), y: 350.0 },
        egui::Layout::left_to_right(egui::Align::Min),
        |ui| {
            let points = PlotPoints::from_explicit_callback(|x| x.sin(), 0.0..10.0, 100);
            let line = Line::new("test", points).name("Sine Wave");

            let available_width = ui.available_width() * 0.7;
            Plot::new("placeholder_plot")
                .width(available_width)
                .show(ui, |plot_ui| {
                    plot_ui.line(line);
                }
            );

            portfolio_holdings_table(app, ui);
        }
    );
}


fn portfolio_holdings_table(app: &mut App, ui: &mut Ui) {
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
                        ui.heading("QTY");
                        ui.heading("Price");
                        ui.heading("Weight");
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
                    }
                );
            }
        );
    });
}