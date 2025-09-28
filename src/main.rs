#![feature(result_option_map_or_default)]

mod app;
mod code_page;
mod views;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Lcf explorer",
        native_options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
    .unwrap();
}
