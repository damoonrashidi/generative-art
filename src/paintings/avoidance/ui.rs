use std::fmt::Debug;

use super::{algo::avoidance, config::AvoidanceConfig};

pub struct AvoidanceUi {
    svg: egui_extras::RetainedImage,
    config: AvoidanceConfig,
    svg_str: String,
}

impl Debug for AvoidanceUi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Default for AvoidanceUi {
    fn default() -> Self {
        AvoidanceUi {
            svg: egui_extras::RetainedImage::from_svg_str(
                "default",
                r#"<svg viewBox="0 0 500 500" xmlns="http://www.w3.org/2000/svg"></svg>"#,
            )
            .unwrap(),
            config: AvoidanceConfig::default(),
            svg_str: "".into(),
        }
    }
}

impl AvoidanceUi {
    fn generate(&self) -> String {
        avoidance(&self.config)
    }

    pub fn set_new_svg(&mut self) {
        let svg_str = self.generate();
        self.svg_str = svg_str.clone();
        self.svg = egui_extras::RetainedImage::from_svg_str("Avoidance", svg_str.as_str()).unwrap();
    }
}

impl eframe::App for AvoidanceUi {
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
                    eframe::egui::Slider::new(&mut self.config.scan_distance, 10.0..=50.0)
                        .text("Scan distance"),
                )
                .changed()
            {
                self.set_new_svg();
            }

            if ui
                .add(
                    eframe::egui::Slider::new(&mut self.config.scan_angle, -1.0..=1.0)
                        .text("scan angle"),
                )
                .changed()
            {
                self.set_new_svg();
            }

            self.svg.show_size(ui, ui.available_size());
        });
    }
}
