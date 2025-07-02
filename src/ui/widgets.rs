
use egui::Ui;
use egui_plot::{Line, Plot, PlotPoints};
use crate::{data::{self, types::TickerDatatype}, ui::renderer::{AppPage, DataPageState}};

use super::renderer::App;
use chrono::NaiveDate;




pub fn navbar(app: &mut App, ui: &mut Ui) {
    ui.heading("Arbitrium App");

    ui.horizontal(|ui| {
        if ui.button("Home").clicked() {
            app.page = AppPage::Home;
        }
        if ui.button("Data Viewer").clicked() {
            app.page = AppPage::DataViewer(DataPageState::default());
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


pub fn data_controller_widget(app: &mut App, state: &mut DataPageState, ui: &mut Ui) {

    // let mut from_date = NaiveDate::default();
    // let mut to_date = NaiveDate::default();

    let mut fetch_data_clicked: bool = false;
    let mut read_data_clicked: bool = false; 

    ui.heading("Select Historical Data to Import:");
    ui.add_space(7.5);

    let text = match state.symbol_is_etf {
        true => {"ETF"},
        false => {"Common Stock"},
        _ => {""}
    };

    ui.horizontal(|ui| {
        egui::containers::ComboBox::from_label("Symbol Type").selected_text(text).show_ui(ui, |box_ui| {
            box_ui.selectable_value(&mut state.symbol_is_etf, true, "ETF" );
            box_ui.selectable_value( &mut state.symbol_is_etf, false, "Common Stock" );
        });

        ui.label("From: ");
        ui.add(egui_extras::DatePickerButton::new(&mut state.from_date).id_salt("from_date_picker"));

        ui.label("To: ");
        ui.add(egui_extras::DatePickerButton::new(&mut state.to_date).id_salt("to_date_picker"));

        ui.label("For Symbol: ");
        ui.text_edit_singleline(&mut app.input);
    });

    ui.add_space(5.0);

    ui.horizontal(|ui| {
        fetch_data_clicked = ui.button("Fetch Data from Remote").clicked();
        read_data_clicked = ui.button("Read Data").clicked();
    });

    ui.separator();

    if fetch_data_clicked {

        // println!("{:#?}", state.from_date);
        // println!("{:#?}", state.to_date);
        // println!("{:#?}", app.input.clone());

        let data = futures::executor::block_on(
            data::collection::get_ticker_data(app.input.clone(), TickerDatatype::HistOHCL(state.from_date.to_string(), state.to_date.to_string()), data::types::PointTimeDelta::Day)
        ).unwrap();
        
        // println!("{:#?}", data);
    }
    
}


