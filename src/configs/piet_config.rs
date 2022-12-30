use clap::Parser;

#[derive(Parser, Debug, Default)]
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

impl From<PietConfig> for String {
    fn from(val: PietConfig) -> Self {
        format!(
            "<!-- size={} rounds={} split-chance={} -->",
            val.size, val.rounds, val.split_chance
        )
    }
}
