use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ForcesConfig {
    #[arg(long, default_value_t = 1500.0)]
    pub size: f64,

    #[arg(long, default_value_t = 5000)]
    pub density: usize,

    #[arg(long, default_value_t = 1.8)]
    pub distort: f64,

    #[arg(long, default_value_t = 1500.0)]
    pub zoom: f64,

    #[arg(long, default_value_t = 1)]
    pub seed: u32,
}

impl ForcesConfig {
    pub fn new() -> ForcesConfig {
        let args = ForcesConfig::parse();

        return ForcesConfig {
            density: args.density,
            distort: args.distort,
            seed: args.seed,
            zoom: args.zoom,
            size: args.size,
        };
    }
}

impl Into<String> for ForcesConfig {
    fn into(self) -> String {
        format!(
            "<!-- size={} density={} distort={} zoom={} seed={} -->",
            self.size, self.density, self.distort, self.zoom, self.seed
        )
    }
}
