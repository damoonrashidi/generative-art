use generative_art::{
    configs::grid_config::GridConfig,
    palette::color::Color,
    shapes::{circle::Circle, point::Point, rectangle::Rectangle},
    svg::svg::SVG,
};
use rand::{thread_rng, Rng};
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

fn main() -> Result<(), std::fmt::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(eframe::egui::vec2(1000.0, 1400.0)),
        ..Default::default()
    };

    eframe::run_native("Grid", options, Box::new(|_| Box::new(App::default())));

    Ok(())
}

struct App {
    svg: egui_extras::RetainedImage,
    svg_str: String,
}

impl Default for App {
    fn default() -> Self {
        App {
            svg: egui_extras::RetainedImage::from_svg_str(
                "default",
                r#"<svg viewBox="0 0 500 500" xmlns="http://www.w3.org/2000/svg"></svg>"#,
            )
            .unwrap(),
            svg_str: "".into(),
        }
    }
}

impl App {
    fn generate(&self) -> String {
        let config = GridConfig {
            size: 1500.0,
            max_dots: 50,
        };

        let bounds = Rectangle {
            position: Point { x: 0.0, y: 0.0 },
            width: config.size,
            height: config.size * 1.4,
            color: Some(Color::Hex("#fff")),
        };

        let inner_bounds = bounds.scale(0.9);
        let mut rects: Vec<Rectangle> = vec![];
        let mut document = SVG::new("Grid", bounds);
        let mut rng = rand::thread_rng();

        let mut x: f64 = inner_bounds.position.x;

        document.add_shape(Box::new(bounds));

        while inner_bounds.x_range().contains(&x) {
            let block_width = rng.gen_range(bounds.width * 0.003..bounds.width * 0.04);
            let mut y = inner_bounds.position.y;

            while inner_bounds.y_range().contains(&y) {
                let block_height = if rng.gen_bool(0.2) {
                    bounds.height * rng.gen_range(0.03..0.045)
                } else {
                    bounds.height * rng.gen_range(0.002..0.01)
                };

                let rect = Rectangle::new(Point { x, y }, block_width, block_height);
                rects.push(rect);
                y += block_height;
            }
            x += block_width;
        }

        let count = rects.len();
        let pool = ThreadPool::new(count);
        let (sender, receiver) = channel::<Vec<Circle>>();
        for rect in rects {
            let sender = sender.clone();
            pool.execute(move || {
                let mut thread_rng = thread_rng();
                let mut points: Vec<Circle> = vec![];
                let dots = App::get_dot_count(&rect, bounds.height, config.max_dots);
                for _ in 0..dots {
                    let mut circle = Circle::new(
                        Point {
                            x: thread_rng.gen_range(rect.x_range()),
                            y: thread_rng.gen_range(rect.y_range()),
                        },
                        0.5,
                    );

                    circle.set_color(Color::Hex("#111"));

                    points.push(circle);
                }
                sender.send(points).expect("error");
            });
        }

        receiver.iter().take(count).for_each(|circles| {
            for circle in circles {
                document.add_shape(Box::new(circle));
            }
        });

        document.generate()
    }

    pub fn set_new_svg(&mut self) {
        let svg_str = self.generate();
        self.svg_str = svg_str.clone();
        self.svg = egui_extras::RetainedImage::from_svg_str("Forces", svg_str.as_str()).unwrap();
    }

    fn get_dot_count(rect: &Rectangle, render_height: f64, max_count: usize) -> usize {
        let area_str = format!("{}", rect.area());

        let max_str_len = std::cmp::min(area_str.len(), 4);

        let normalized_area = area_str
            .get(0..max_str_len)
            .unwrap_or("0.0")
            .parse::<f64>()
            .unwrap_or(0.);

        let mut rng = rand::thread_rng();
        let count = (render_height - rect.position.y) * rng.gen_range(2.0..4.0) + normalized_area;

        (count as usize).min(max_count)
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("generate").clicked() {
                self.set_new_svg();
            }

            self.svg.show_size(ui, ui.available_size());
        });
    }
}
