use generative_art::paintings::forces::{algo::forces, config::ForcesConfig};

fn main() -> Result<(), std::fmt::Error> {
    let config = ForcesConfig::new();
    let mut svg = forces(std::rc::Rc::new(&config));
    svg.save();

    Ok(())
}
