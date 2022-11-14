use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ForcesConfig {
    /// Set the size of the final SVG output
    #[arg(long, default_value_t = 1500.0)]
    pub size: f64,

    /// Number of lines to attempt to fill the image with
    #[arg(long, default_value_t = 5000)]
    pub line_count: usize,

    /// How much each turn in a line is exaggerated, the higher the number the higher the more chaotic the output
    #[arg(long, default_value_t = 1.8)]
    pub chaos: f64,

    /// How much to smooth out the line curves, the higher the number the smoother the lines
    #[arg(long, default_value_t = 1200.0)]
    pub smoothness: f64,

    /// Seed for the RNG
    #[arg(long, default_value_t = 1)]
    pub seed: u32,
}

impl ForcesConfig {
    pub fn new() -> ForcesConfig {
        let args = ForcesConfig::parse();

        return ForcesConfig {
            line_count: args.line_count,
            chaos: args.chaos,
            seed: args.seed,
            smoothness: args.smoothness,
            size: args.size,
        };
    }
}

impl Into<String> for ForcesConfig {
    fn into(self) -> String {
        format!(
            "<!-- size={} density={} distort={} zoom={} seed={} -->",
            self.size, self.line_count, self.chaos, self.smoothness, self.seed
        )
    }
}