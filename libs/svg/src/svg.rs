use std::{fs::File, io::Write};

use chrono::{Datelike, Utc};
use shapes::{rectangle::Rectangle, shape::Shape};

use crate::group::Group;

#[derive(Debug)]
pub struct SVG<'a> {
    pub name: &'a str,
    bounds: Rectangle,
    document: String,
}

impl<'a> SVG<'static> {
    pub fn new(name: &'static str, bounds: Rectangle) -> SVG<'a> {
        SVG {
            name,
            bounds,
            document: format!(
                "<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
                bounds.width, bounds.height
            ),
        }
    }

    pub fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.document.push_str(&shape.as_svg());
    }

    pub fn add_group(&mut self, group: Group) {
        self.document.push_str(&group.as_svg());
    }

    pub fn get_bounds(&self) -> Rectangle {
        self.bounds
    }

    pub fn save(&mut self, config: Option<String>) {
        self.document.push_str("</svg>");

        if let Some(comment) = config {
            self.document = format!("{}\n{}", self.document, comment);
        }

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
            self.name.to_lowercase(),
            self.name,
            time
        );

        let _result = std::fs::create_dir(format!("./output/{}", self.name.to_lowercase()));

        let mut f = File::create(&path).expect("could not open file for writing");

        f.write_all(self.document.as_bytes())
            .expect("Could not write to file");
    }
}
