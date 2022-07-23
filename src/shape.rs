mod circle;

use std::{fs::File, io::Write};

struct SVG {
    document: String,
}

impl SVG {
    fn create(mut self: &SVG) -> () {
        self.document
            .push_str("<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">");
    }

    fn write(self: &SVG) -> () {
        let time: String = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        let mut f = File::create(format!("./output/image{}.svg", time))
            .expect("could not open file for writing");

        let _result = f
            .write_all(self.document.as_bytes())
            .expect("Could not write to file");
        println!("Wrote image to ./output/image{}.svg", time);
    }
}
