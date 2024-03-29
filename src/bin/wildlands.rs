use generative_art::paintings::wildlands::{algo::wildlands, config::WildlandsConfig};

fn main() {
    let config = WildlandsConfig {
        seed: 0,
        radius: 5.,
        step_size: 2.5,
        size: 1500.0,
        line_count: 1500,
        chaos: 2.2,
        smoothness: 1000.0,
        max_line_length: 500,
        color_rounds: 5,
    };

    let mut svg = wildlands(&config);

    svg.save();
}
