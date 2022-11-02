use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct PietConfig {
    #[arg(long, default_value_t = 1500.0)]
    pub size: f64,

    #[arg(long, default_value_t = 5)]
    pub rounds: usize,

    #[arg(long, default_value_t = 1.8)]
    pub split_chance: f64,
}

impl PietConfig {
    pub fn new() -> PietConfig {
        let args = PietConfig::parse();

        return PietConfig {
            size: args.size,
            rounds: args.rounds,
            split_chance: args.split_chance,
        };
    }
}

impl Into<String> for PietConfig {
    fn into(self) -> String {
        format!(
            "<!-- size={} rounds={} split_chance={} -->",
            self.size, self.rounds, self.split_chance
        )
    }
}
