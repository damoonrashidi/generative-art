use generative_art::paintings::nightfall::{algo::generate_nightfall, config::NightfallConfig};

fn main() {
    let config = NightfallConfig::new();
    let mut svg = generate_nightfall(std::rc::Rc::new(&config));

    svg.save();
}
