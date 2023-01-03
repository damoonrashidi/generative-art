use std::{fmt::Debug, rc::Rc};

use super::{algo::piet, config::PietConfig};

pub struct PietUi {
    svg: egui_extras::RetainedImage,
    config: PietConfig,
    svg_str: String,
}

impl Debug for PietUi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Default for PietUi {
    fn default() -> Self {
        PietUi {
            svg: egui_extras::RetainedImage::from_svg_str(
                "default",
                r#"<svg viewBox="0 0 500 500" xmlns="http://www.w3.org/2000/svg"></svg>"#,
            )
            .unwrap(),
            config: PietConfig {
                size: 1500.,
                rounds: 5,
                padding: 16.0,
                split_chance: 0.1,
            },
            svg_str: "".into(),
        }
    }
}

impl PietUi {
    fn generate(&self) -> String {
        let svg = piet(Rc::new(&self.config));
        svg.generate()
    }

    pub fn set_new_svg(&mut self) {
        let svg_str = self.generate();
        self.svg_str = svg_str.clone();
        self.svg = egui_extras::RetainedImage::from_svg_str("Piet", svg_str.as_str()).unwrap();
    }
}

impl eframe::App for PietUi {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            if ui
                .add(eframe::egui::Slider::new(&mut self.config.size, 500.0..=3000.0).text("Size"))
                .changed()
            {
                self.set_new_svg();
            }

            if ui
                .add(
                    eframe::egui::Slider::new(&mut self.config.padding, 0.0..=50.0).text("Padding"),
                )
                .changed()
            {
                self.set_new_svg();
            }

            if ui
                .add(eframe::egui::Slider::new(&mut self.config.rounds, 2..=10).text("Rounds"))
                .changed()
            {
                self.set_new_svg();
            }

            if ui
                .add(
                    eframe::egui::Slider::new(&mut self.config.split_chance, 0.1..=1.0)
                        .text("Split chance"),
                )
                .changed()
            {
                self.set_new_svg();
            }

            self.svg.show_size(ui, ui.available_size());
        });
    }
}
