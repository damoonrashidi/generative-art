use std::fmt::Error;

use generative_art::paintings::avoidance::ui::AvoidanceUi;

fn main() -> Result<(), Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(eframe::egui::vec2(1000.0, 1400.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Avoidance",
        options,
        Box::new(|_| Box::new(AvoidanceUi::default())),
    );

    Ok(())
}
