use std::fmt::Debug;

use super::{algo::wildlands, config::WildlandsConfig};

pub struct WildlandsUi {
    svg: egui_extras::RetainedImage,
    config: WildlandsConfig,
    svg_str: String,
}

impl Debug for WildlandsUi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Default for WildlandsUi {
    fn default() -> Self {
        WildlandsUi {
            svg: egui_extras::RetainedImage::from_svg_str(
                "default",
                r#"<svg viewBox="0 0 500 500" xmlns="http://www.w3.org/2000/svg"></svg>"#,
            )
            .unwrap(),
            config: WildlandsConfig {
                line_count: 1500,
                size: 1400.0,
                chaos: 0.5,
                smoothness: 900.0,
                max_line_length: 500,
            },
            svg_str: "".into(),
        }
    }
}

impl WildlandsUi {
    fn generate(&self) -> String {
        let svg = wildlands(&self.config);
        svg.generate()
    }

    pub fn set_new_svg(&mut self) {
        let svg_str = self.generate();
        self.svg_str = svg_str.clone();
        self.svg = egui_extras::RetainedImage::from_svg_str("Wildlands", svg_str.as_str()).unwrap();
    }
}

impl eframe::App for WildlandsUi {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            if ui
                .add(eframe::egui::Slider::new(&mut self.config.size, 1000.0..=3000.0).text("Size"))
                .changed()
            {
                self.set_new_svg();
            }

            if ui
                .add(
                    eframe::egui::Slider::new(&mut self.config.line_count, 800..=15_000)
                        .text("Line count"),
                )
                .changed()
            {
                self.set_new_svg();
            }

            if ui
                .add(
                    eframe::egui::Slider::new(&mut self.config.chaos, 0.2..=10.0)
                        .text("Chaos")
                        .step_by(0.2),
                )
                .changed()
            {
                self.set_new_svg();
            }

            if ui
                .add(
                    eframe::egui::Slider::new(&mut self.config.smoothness, 100.0..=2000.0)
                        .text("Smoothness"),
                )
                .changed()
            {
                self.set_new_svg();
            }

            if ui
                .add(
                    eframe::egui::Slider::new(
                        &mut self.config.max_line_length,
                        100..=self.config.size as usize,
                    )
                    .text("Max line length"),
                )
                .changed()
            {
                self.set_new_svg();
            }

            self.svg.show_size(ui, ui.available_size());
        });
    }
}
