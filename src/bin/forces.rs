use generative_art::paintings::forces::{algo::generate_forces, config::ForcesConfig};

fn main() -> Result<(), std::fmt::Error> {
    let config = ForcesConfig::new();
    let mut svg = generate_forces(std::rc::Rc::new(&config));
    svg.save();

    Ok(())
}
