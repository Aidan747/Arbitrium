
use std::str::FromStr;

use egui::Ui;
use egui_extras::{Column, TableBuilder};
use egui_plot::{Line, Plot, PlotPoints};
use futures::future;
use crate::{analysis, data::{self, db_service, types::{Etf, TickerData, TickerDataframe, TickerDatatype}}, ui::renderer::{AppPage, DataPageState, TrainTestPageState}};

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
            app.page = AppPage::TrainTest(TrainTestPageState::default());
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
    let mut write_data_clicked: bool = false;

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
        write_data_clicked = ui.button("Send Data to DB").clicked();
    });

    if state.symbol_is_etf {
        ui.add_space(5.0);
        if ui.button("Recursively Populate Current ETF Holdings").clicked() {
            tokio::task::spawn(async {
                
            });
        }
    }

    ui.separator();

    if fetch_data_clicked {

        let data = futures::executor::block_on(
            data::collection::get_ticker_data(app.input.clone(), TickerDatatype::HistOHCL(state.from_date.to_string(), state.to_date.to_string()), data::types::PointTimeDelta::Day)
        ).unwrap();
        
        // println!("{:#?}", data);

        state.ticker_data = Some(data);
    }
    data_table_widget(app, state, ui);

    if write_data_clicked {
        let input = app.input.clone();
        let is_etf = state.symbol_is_etf.clone() &&  Etf::from_str(&input).is_ok();
        tokio::task::spawn(async move {
            if is_etf {
                db_service::insert_etf(Etf::from_str(&input).unwrap()).await.unwrap();
            }
        });
    }
    
}
fn data_table_widget(app: &mut App, state: &mut DataPageState, ui: &mut Ui) {
    let available_width = ui.available_width();
    let available_height= ui.available_height();

    ui.horizontal(|ui| {
        ui.set_min_height(available_height);
        // Left: Table (25% width)
        ui.vertical(|table_ui| {
            let table_width = available_width * 0.25;
            table_ui.set_max_width(table_width);

            table_ui.heading("Data Points: ");
            table_ui.add_space(5.0);

            egui::ScrollArea::vertical()
                .show(table_ui, |ui| {
                    if state.ticker_data.is_none() {
                        ui.label("No Data Loaded...");
                        return;
                    }
                    TableBuilder::new(ui)
                        .column(Column::auto().at_most(table_width / 3.0).clip(true).resizable(true))
                        .columns(Column::remainder().clip(true), 5)
                        .striped(true)
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.heading("Date");
                            });
                            header.col(|ui| {
                                ui.heading("Open");
                            });
                            header.col(|ui| {
                                ui.heading("High");
                            });
                            header.col(|ui| {
                                ui.heading("Low");
                            });
                            header.col(|ui| {
                                ui.heading("Close");
                            });
                            header.col(|ui| {
                                ui.heading("Volume");
                            });                            
                        })
                        .body(|mut body| {
                            if let Some(ticker_data) = &state.ticker_data {
                                body.rows(20.0, ticker_data.price_data.len(), |mut row| {
                                    let val = ticker_data.price_data.get(row.index()).unwrap();
                                    row.col(|ui| {
                                        ui.label(format!("{}", val.t));
                                    });
                                    row.col(|ui| {
                                        ui.label(format!("{:.2}", val.open));
                                    });
                                    row.col(|ui| {
                                        ui.label(format!("{:.2}", val.high));
                                    });
                                    row.col(|ui| {
                                        ui.label(format!("{:.2}", val.low));
                                    });
                                    row.col(|ui| {
                                        ui.label(format!("{:.2}", val.close));
                                    });
                                    row.col(|ui| {
                                        ui.label(format!("{:.2}", val.vol));
                                    });
                                });
                            }
                        }
                    );
                }
            );
            
        });

        // Right: Plot (75% width)
        ui.vertical(|graph_ui| {
            let mut datapoints = PlotPoints::new(vec![]);
            let mut sma_points = PlotPoints::new(vec![]);

            if let Some(ticker_data) = &state.ticker_data {
                datapoints = PlotPoints::from_iter({
                    ticker_data.price_data
                        .iter()
                        .enumerate()
                        .map(|(date, data)| {
                            [date as f64, data.close as f64]
                        })
                    }
                );
                sma_points = PlotPoints::from_iter(analysis::moving_average::sma_on_series(ticker_data.price_data.clone(), 50).iter().enumerate().map(|(date, data)| {
                    [date as f64, data.close as f64]
                }));
            }

            
            
            let line = Line::new("ticker_price_over_time", datapoints);
            let sma = Line::new("sma_lin", sma_points);
            Plot::new("ticker_data_plot").show(graph_ui, |graph_inner_ui| {
                graph_inner_ui.line(line);
                graph_inner_ui.line(sma);
            });


        });
    });
}

pub fn train_test_widget(app: &mut App, ui: &mut Ui, state: &mut TrainTestPageState) {
    ui.heading("Train/Test Configuration");

    ui.horizontal(|ui| {
        ui.checkbox(&mut state.use_stored_data, "Use Stored DB Data");
        ui.checkbox(&mut state.train_time_series_corr, "Find Correlations");
        // ui.checkbox(checked, text)
    });
}
