use std::fmt::Display;

use clap::Parser;

#[derive(Debug, Clone, Copy, Default)]
pub enum ForceMethod {
    #[default]
    Distort,
    Push,
    Pull,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct NightfallConfig {
    #[arg(long, default_value_t = 1500.0)]
    pub size: f64,

    #[arg(long, default_value_t = 5000)]
    pub points: usize,

    #[arg(long, default_value_t = 50.)]
    pub distance: f64,

    #[arg(long, default_value_t = ForceMethod::default())]
    force: ForceMethod,
}

impl NightfallConfig {
    pub fn new() -> NightfallConfig {
        NightfallConfig::parse()
    }
}

impl Display for NightfallConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<!-- size={} point_count={} distance={} force={} -->",
            self.size, self.points, self.distance, self.force
        )
    }
}

impl Display for ForceMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ForceMethod::Distort => write!(f, "distort"),
            ForceMethod::Push => write!(f, "push"),
            ForceMethod::Pull => write!(f, "pull"),
        }
    }
}

impl From<String> for ForceMethod {
    fn from(s: String) -> Self {
        match s.as_str() {
            "distort" => ForceMethod::Distort,
            "push" => ForceMethod::Push,
            "pull" => ForceMethod::Pull,
            _ => panic!("{s} is not a valid force method, valid values are: distort, push, pull",),
        }
    }
}
