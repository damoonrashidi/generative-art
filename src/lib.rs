pub mod circle;
pub mod group;
pub mod palette;
pub mod path;
pub mod point;
pub mod pointmap;
pub mod rectangle;
use chrono::{Datelike, Utc};
use group::Group;

use std::{fs::File, io::Write};

use self::point::Point;

pub struct SVG<'a> {
    pub name: &'a str,
    pub width: f64,
    pub height: f64,
    document: String,
}

pub trait Shape {
    fn as_svg(&self) -> String;
    fn contains(&self, point: Point) -> bool;
}

impl SVG<'static> {
    pub fn new(name: &str, width: f64, height: f64) -> SVG {
        SVG {
            name,
            width,
            height,
            document: format!(
                "<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
                width, height
            ),
        }
    }

    pub fn add_shape(&mut self, shape: Box<dyn Shape>) -> () {
        self.document.push_str(&shape.as_svg());
    }

    pub fn add_group(&mut self, group: Box<Group>) -> () {
        self.document.push_str(&group.as_svg());
    }

    pub fn save(&mut self) -> () {
        let _ = &self.document.push_str("</svg>");

        let now = Utc::now();
        let time = format!(
            "{}-{}-{}-{}",
            now.year(),
            now.month(),
            now.day(),
            now.timestamp_millis()
        );

        let path = format!(
            "./output/{}/{}-{}.svg",
            &self.name.to_lowercase(),
            &self.name,
            time
        );

        let _result = std::fs::create_dir(format!("./output/{}", self.name.to_lowercase()));

        let mut f = File::create(&path).expect("could not open file for writing");

        let _result = f
            .write_all(self.document.as_bytes())
            .expect("Could not write to file");
    }
}
