use std::{fs::File, io::Write};

use chrono::{Datelike, Utc};

use crate::shapes::{rectangle::Rectangle, shape::Shape};

use super::group::Group;

/// The SVG struct contains all the SVG information for a generated artwork.
/// It is the common interface for all shapes when they are finally rendered.
#[derive(Debug)]
pub struct Document<'a> {
    /// Name of the generated art piece, will be used to control both
    /// output folder and file name.
    pub name: &'a str,

    /// The size of the svg element
    pub bounds: Rectangle,

    /// Internal storage for all the shapes added to this SVG document.
    document: String,
}

impl<'a> Document<'static> {
    /**
    Create a new SVG document, this will be in-memory until it is saved.

    Example

    ```

    use generative_art::{
        shapes::{point::Point, rectangle::Rectangle, circle::Circle},
        svg::document::Document,
    };

    // Create the document
    let bounds = Rectangle::new(Point(0.0, 0.0), 1000.0, 1000.0);
    let mut svg = Document::new("my-art", bounds);

    // Draw some art
    svg.add_shape(Box::new(Circle::new(Point(500.0, 500.0), 200.0)));

    // Save the document to disk
    svg.save();
    ```
    */

    pub fn new(name: &'static str, bounds: Rectangle) -> Document<'a> {
        Document {
            name,
            bounds,
            document: format!(
                "<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
                bounds.width, bounds.height
            ),
        }
    }

    /// Add a shape to the SVG document
    pub fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.document.push_str(&shape.as_svg());
    }

    /// Add a group to the document. Groups can hold other shapes but share styles between the shapes.
    /// Nice to have to keep the final svg file size down if there are a lot of shared style between shapes.
    pub fn add_group(&mut self, group: Group) {
        self.document.push_str(&group.as_svg());
    }

    /// Save the SVG document to disk. Optionally a configuration string can be passed, which will be appended
    /// as an SVG comment in the file, to be able to recreate a given painting.
    pub fn save(&mut self) {
        self.document.push_str("</svg>");

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

        let mut f = File::create(path).expect("could not open file for writing");

        f.write_all(self.document.as_bytes())
            .expect("Could not write to file");
    }

    /// Generate the svg in memory
    pub fn generate(&self) -> String {
        format!("{}</svg>", self.document)
    }
}
