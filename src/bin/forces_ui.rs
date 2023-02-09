use std::fmt::Error;



fn main() -> Result<(), Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(eframe::egui::vec2(1000.0, 1400.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Forces",
        options,
        Box::new(|_| Box::<generative_art::paintings::forces::ui::ForcesApp>::default()),
    );

    Ok(())
}
