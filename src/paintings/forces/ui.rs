use crate::{paintings::forces::config::ForcesParams, shapes::point::Point};
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;

use rand::prelude::*;

use super::algo::generate_forces;
use super::config::ForcesConfig;

pub struct ForcesApp {
    config: ForcesParams,
    svg: egui_extras::RetainedImage,
    svg_str: String,
}

impl Debug for ForcesApp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl ForcesApp {
    pub fn set_new_image(&mut self) {
        let config = ForcesConfig {
            size: self.config.size,
            line_count: self.config.line_count,
            min_line_length: self.config.min_line_length,
            max_line_length: self.config.max_line_length,
            palette: super::config::ForcesPalette::OrangeAutumn,
            chaos: self.config.chaos,
            smoothness: self.config.smoothness,
            seed: self.config.seed,
            split_line_chance: self.config.split_line_chance,
            split_with_gap: self.config.split_with_gap,
        };
        let svg_str = generate_forces(std::rc::Rc::new(&config)).generate();
        self.svg = egui_extras::RetainedImage::from_svg_str("Forces", svg_str.as_str()).unwrap();
    }

    #[allow(unused)]
    fn split_line(line: Vec<Point>, use_gap: bool) -> Vec<Vec<Point>> {
        let mut rng = thread_rng();
        let mut lines = vec![];
        let mut last_split = 1;
        for i in 1..line.len() - 1 {
            if rng.gen_bool(0.2) {
                if use_gap {
                    lines.push(line[last_split..i].into());
                } else {
                    lines.push(line[last_split - 1..i + 1].into());
                }
                last_split = i
            }
        }

        lines
    }
}

impl Default for ForcesApp {
    fn default() -> Self {
        ForcesApp {
            svg: egui_extras::RetainedImage::from_svg_str(
                "default",
                r#"<svg viewBox="0 0 500 500" xmlns="http://www.w3.org/2000/svg"></svg>"#,
            )
            .unwrap(),
            svg_str: "".into(),
            config: ForcesParams {
                line_count: 500,
                size: 2500.,
                chaos: 0.5,
                smoothness: 800.0,
                min_line_length: 50.0,
                max_line_length: 1500.0,
                palette: "forces_palette".to_string(),
                seed: 0,
                split_line_chance: 0.0,
                split_with_gap: false,
            },
        }
    }
}

impl eframe::App for ForcesApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            if ui
                .add(eframe::egui::Slider::new(&mut self.config.seed, 0..=10000).text("Seed"))
                .changed()
            {
                self.set_new_image();
            }

            if ui
                .add(
                    eframe::egui::Slider::new(&mut self.config.line_count, 50..=3000)
                        .text("Line Count"),
                )
                .changed()
            {
                self.set_new_image();
            }

            // Min/Max length
            ui.horizontal(|ui| {
                if ui
                    .add(
                        eframe::egui::Slider::new(
                            &mut self.config.min_line_length,
                            40.0..=self.config.max_line_length,
                        )
                        .text("Min line length"),
                    )
                    .changed()
                {
                    self.set_new_image();
                };

                if ui
                    .add(
                        eframe::egui::Slider::new(
                            &mut self.config.max_line_length,
                            self.config.min_line_length..=self.config.size * 1.4,
                        )
                        .text("Max line length"),
                    )
                    .changed()
                {
                    self.set_new_image();
                }
            });

            // Line behaviour
            ui.horizontal(|ui| {
                if ui
                    .add(
                        eframe::egui::Slider::new(&mut self.config.chaos, 0.5..=16.0).text("Chaos"),
                    )
                    .changed()
                {
                    self.set_new_image();
                }

                if ui
                    .add(
                        eframe::egui::Slider::new(&mut self.config.smoothness, 400.0..=10000.0)
                            .text("Smoothness"),
                    )
                    .changed()
                {
                    self.set_new_image();
                }
            });

            // Line splitting
            ui.horizontal(|ui| {
                if ui
                    .add(
                        eframe::egui::Slider::new(&mut self.config.split_line_chance, 0.0..=1.0)
                            .text("Split chance"),
                    )
                    .changed()
                {
                    self.set_new_image();
                }

                if ui
                    .add(eframe::egui::Checkbox::new(
                        &mut self.config.split_with_gap,
                        "Split with gap",
                    ))
                    .changed()
                {
                    self.set_new_image()
                }
            });

            if ui.button("Export").clicked() {
                let mut f = File::create("./output/forces/forces-live.svg")
                    .expect("could not open file for writing");

                f.write_all(self.svg_str.as_bytes())
                    .expect("Could not write to file");
            }

            self.svg.show_size(ui, ui.available_size());
        });
    }
}
