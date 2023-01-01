use std::rc::Rc;

use generative_art::paintings::piet::{algo::piet, config::PietConfig};

fn main() {
    let config = PietConfig::new();

    let mut svg = piet(Rc::new(&config));

    svg.save();
}
