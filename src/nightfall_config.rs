use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct NightfallConfig {
    #[arg(long, default_value_t = 1500.0)]
    pub size: f64,

    #[arg(long, default_value_t = 5000)]
    pub points: usize,

    #[arg(long, default_value_t = 50.)]
    pub distance: f64,
}

impl NightfallConfig {
    pub fn new() -> NightfallConfig {
        let args = NightfallConfig::parse();

        return NightfallConfig {
            size: args.size,
            points: args.points,
            distance: args.distance,
        };
    }
}

impl Into<String> for NightfallConfig {
    fn into(self) -> String {
        format!(
            "<!-- size={} point_count={} distance={} -->",
            self.size, self.points, self.distance
        )
    }
}
