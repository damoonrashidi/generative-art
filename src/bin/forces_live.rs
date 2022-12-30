use std::fmt::Error;

use generative_art::{
    configs::forces_config::ForcesParams,
    palette::{color::Color, palettes::Palettes},
    shapes::{
        blob::Blob,
        circle::Circle,
        path::{Path, PathStyle},
        point::Point,
        pointmap::PointMap,
        rectangle::Rectangle,
        shape::Shape,
    },
    svg::svg::SVG,
};

use noise::{NoiseFn, Seedable, SuperSimplex};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

fn main() -> Result<(), Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(eframe::egui::vec2(1000.0, 1400.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Forces",
        options,
        Box::new(|_| Box::new(ForcesApp::default())),
    );

    Ok(())
}

struct ForcesApp {
    config: ForcesParams,
    svg: egui_extras::RetainedImage,
}

impl ForcesApp {
    pub fn set_new_image(&mut self) {
        self.svg = egui_extras::RetainedImage::from_svg_str(
            "Forces",
            self.generate_svg(&self.config).as_str(),
        )
        .unwrap();
    }

    fn generate_svg(&self, config: &ForcesParams) -> String {
        let mut bounds = Rectangle::new(Point { x: 0.0, y: 0.0 }, config.size, config.size * 1.4);
        let (background, palette) = Palettes::orange_autumn();

        bounds.set_color(background);
        let inner_bounds = bounds.scale(0.9);

        let mut svg = SVG::new("Forces", bounds);
        svg.add_shape(Box::new(bounds));
        let mut rng = ChaCha20Rng::from_entropy();

        let mut color_bounds: Vec<Blob> = vec![];

        for _ in 0..20 {
            let x = rng.gen_range(bounds.x_range());
            let y = rng.gen_range(bounds.y_range());
            let r = rng.gen_range((bounds.width / 10.0)..(bounds.width / 7.));
            let color = palette.get_random_color();

            let blob = Blob::new(Point { x, y }, r, color);

            color_bounds.push(blob);
        }

        let mut point_map: PointMap<'_, Circle> = PointMap::new(&bounds, 20);
        let noise = SuperSimplex::new().set_seed(config.seed);

        for i in 0..config.line_count {
            let mut x: f64 = rng.gen_range(inner_bounds.x_range());
            let mut y: f64 = rng.gen_range(inner_bounds.y_range());

            let line_color: Option<Color> = match color_bounds
                .iter()
                .find(|region| region.contains(&Point { x, y }))
            {
                Some(region) => region.color,
                _ => palette.get_random_color(),
            };

            let mut r = 65.0;
            let mut step_size = 50.0;

            if rng.gen_bool(0.7) && i < 5 {
                r = 200.;
                step_size = 250.;
            } else if rng.gen_bool(0.1) {
                r = 40.;
                step_size = 30.;
            }

            let mut line = Path {
                points: vec![],
                style: PathStyle {
                    stroke_weight: Some(r),
                    ..Default::default()
                },
            };

            while inner_bounds.contains(&Point { x, y }) && line.length() < config.max_line_length {
                let n = noise.get([x / config.smoothness, y / config.smoothness]);
                x += (config.chaos * n).cos() * step_size;
                y += (config.chaos * n).sin() * step_size;
                let circle = Circle::new(Point { x, y }, r);

                if let Ok(neighbors) = point_map.get_neighbors(&circle, None) {
                    if neighbors
                        .iter()
                        .any(|neighbor| neighbor.distance(&circle) < r / 2.)
                    {
                        break;
                    }
                } else {
                    break;
                }

                line.add_point(Point { x, y });
            }

            if line.length() > config.min_line_length {
                for point in line.points.iter() {
                    let circle = Circle::new(*point, r);
                    let _ = point_map.insert(circle);
                }

                if config.split_line_chance > 0.0 && rng.gen_bool(config.split_line_chance) {
                    for points in ForcesApp::split_line(line.points, config.split_with_gap) {
                        let path = Path::new(
                            points,
                            PathStyle {
                                stroke_weight: Some(r),
                                stroke: palette.get_random_color(),
                                color: None,
                            },
                        );
                        svg.add_shape(Box::new(path));
                    }
                } else {
                    line.style = PathStyle {
                        stroke_weight: Some(r),
                        stroke: line_color,
                        color: None,
                    };
                    svg.add_shape(Box::new(line));
                }
            }
        }

        svg.generate()
    }

    #[allow(unused)]
    fn split_line(line: Vec<Point>, use_gap: bool) -> Vec<Vec<Point>> {
        let mut rng = thread_rng();
        let mut lines = vec![];
        let mut last_split = 1;
        for i in 1..line.len() - 1 {
            if rng.gen_bool(0.5) {
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
            config: ForcesParams {
                line_count: 500,
                size: 2500.,
                chaos: 1.5,
                smoothness: 1500.0,
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
                    eframe::egui::Slider::new(&mut self.config.line_count, 50..=20000)
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
                            self.config.min_line_length..=self.config.min_line_length + 1000.,
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
                    .add(eframe::egui::Slider::new(&mut self.config.chaos, 0.5..=8.0).text("Chaos"))
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

            self.svg.show_size(ui, ui.available_size());
        });
    }
}
