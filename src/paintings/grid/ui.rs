use std::{fmt::Debug, fs::File, io::Write, sync::Arc};

use eframe::egui::Slider;

use super::{algo::generate_grid, config::GridConfig};

pub struct GridApp {
    config: GridConfig,
    svg: egui_extras::RetainedImage,
    svg_str: String,
}

impl Debug for GridApp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Default for GridApp {
    fn default() -> Self {
        GridApp {
            config: GridConfig {
                size: 1200.,
                max_dots: 40,
            },
            svg: egui_extras::RetainedImage::from_svg_str(
                "default",
                r#"<svg viewBox="0 0 500 500" xmlns="http://www.w3.org/2000/svg"></svg>"#,
            )
            .unwrap(),
            svg_str: "".into(),
        }
    }
}

impl GridApp {
    fn generate(&self) -> String {
        let svg = generate_grid(Arc::new(self.config));

        svg.generate()
    }

    pub fn set_new_svg(&mut self) {
        let svg_str = self.generate();
        self.svg_str = svg_str.clone();
        self.svg = egui_extras::RetainedImage::from_svg_str("Grid", svg_str.as_str()).unwrap();
    }
}

impl eframe::App for GridApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(Slider::new(&mut self.config.size, 800.0..=2500.).text("Size"));
            ui.add(Slider::new(&mut self.config.max_dots, 5..=60).text("Max dots"));

            ui.horizontal(|ui| {
                if ui.button("Generate").clicked() {
                    self.set_new_svg();
                }

                if ui.button("Save").clicked() {
                    let mut f = File::create("./output/grid/grid-ui.svg")
                        .expect("could not open file for writing");

                    f.write_all(self.svg_str.as_bytes())
                        .expect("Could not write to file");
                }
            });

            self.svg.show_size(ui, ui.available_size());
        });
    }
}
