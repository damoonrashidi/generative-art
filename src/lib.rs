pub mod circle;
pub mod line;
pub mod point;
pub mod rectangle;
use chrono::{Datelike, Utc};

use std::{fs::File, io::Write};

use self::point::Point;

pub struct SVG<'a> {
    pub name: &'a str,
    pub width: f64,
    pub height: f64,
    pub document: String,
}

pub trait Shape {
    fn as_svg(&self) -> String;
    fn contains(&self, point: &Point) -> bool;
}

impl SVG<'static> {
    pub fn create_document(&mut self) -> () {
        self.document = format!(
            "<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
            self.width, self.height
        );
    }

    pub fn add(&mut self, shape: Box<dyn Shape>) -> () {
        self.document.push_str(&shape.as_svg());
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
        println!("Wrote image to {}", &path);
    }
}
