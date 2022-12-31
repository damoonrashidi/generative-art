use std::sync::Arc;

use generative_art::paintings::grid::{algo::generate_grid, config::GridConfig};

fn main() {
    let config = GridConfig::new();
    let mut svg = generate_grid(Arc::new(config));
    svg.save();
}
