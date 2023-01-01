use std::rc::Rc;

use generative_art::{configs::piet_config::PietConfig, paintings::piet::algo::piet};

fn main() {
    let config = PietConfig::new();

    let mut svg = piet(Rc::new(&config));

    svg.save();
}
