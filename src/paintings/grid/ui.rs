use std::{fmt::Debug, sync::Arc};

use super::{algo::generate_grid, config::GridConfig};

pub struct GridApp {
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
        let config = GridConfig {
            size: 1500.0,
            max_dots: 50,
        };

        let svg = generate_grid(Arc::new(config));

        svg.generate()
    }

    pub fn set_new_svg(&mut self) {
        let svg_str = self.generate();
        self.svg_str = svg_str.clone();
        self.svg = egui_extras::RetainedImage::from_svg_str("Forces", svg_str.as_str()).unwrap();
    }
}

impl eframe::App for GridApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("generate").clicked() {
                self.set_new_svg();
            }

            self.svg.show_size(ui, ui.available_size());
        });
    }
}
