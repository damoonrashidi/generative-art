use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
pub struct GridConfig {
    /// Set the size of the final SVG output
    #[arg(long, default_value_t = 1500.0)]
    pub size: f64,

    /// Number of lines to attempt to fill the image with
    #[arg(long, default_value_t = 900)]
    pub max_dots: usize,
}

impl GridConfig {
    pub fn new() -> GridConfig {
        GridConfig::parse()
    }
}

impl From<GridConfig> for String {
    fn from(config: GridConfig) -> Self {
        format!("<!-- size={} max_dots={} -->", config.size, config.max_dots)
    }
}
