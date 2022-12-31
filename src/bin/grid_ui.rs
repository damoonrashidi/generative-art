use generative_art::paintings::grid::ui::GridApp;

fn main() -> Result<(), std::fmt::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(eframe::egui::vec2(1000.0, 1400.0)),
        ..Default::default()
    };

    eframe::run_native("Grid", options, Box::new(|_| Box::new(GridApp::default())));

    Ok(())
}
