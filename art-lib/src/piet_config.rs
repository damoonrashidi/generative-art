use std::fmt::Display;

use clap::Parser;

#[derive(Debug, Clone, Default, Parser)]
#[command(author, version, about, long_about = None)]
pub struct PietConfig {
    #[arg(long, default_value_t = 1500.0)]
    pub size: f64,

    #[arg(long, default_value_t = 5)]
    pub rounds: usize,

    #[arg(long, default_value_t = 0.7)]
    pub split_chance: f64,
}

impl PietConfig {
    pub fn new() -> PietConfig {
        PietConfig::parse()
    }
}

impl Display for PietConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<!-- size={} rounds={} split_chance={} -->",
            self.size, self.rounds, self.split_chance
        )
    }
}
