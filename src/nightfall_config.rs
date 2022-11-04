use std::fmt::Display;

use clap::Parser;

pub enum ForceMethod {
    Distort,
    Push,
    Pull,
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

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct NightfallParams {
    #[arg(long, default_value_t = 1500.0)]
    pub size: f64,

    #[arg(long, default_value_t = 5000)]
    pub points: usize,

    #[arg(long, default_value_t = 50.)]
    pub distance: f64,

    #[arg(long, default_value_t = String::from("distort"))]
    force: String,
}

pub struct NightfallConfig {
    pub size: f64,
    pub points: usize,
    pub distance: f64,
    pub force: ForceMethod,
}

impl NightfallConfig {
    pub fn new() -> NightfallConfig {
        let args = NightfallParams::parse();

        let force = match args.force.to_ascii_lowercase().as_str() {
            "distort" => ForceMethod::Distort,
            "push" => ForceMethod::Push,
            "pull" => ForceMethod::Pull,
            _ => panic!(
                "{} is not a valid force method, valid values are: distort, push, pull",
                args.force
            ),
        };

        return NightfallConfig {
            size: args.size,
            points: args.points,
            distance: args.distance,
            force,
        };
    }
}

impl Into<String> for NightfallConfig {
    fn into(self) -> String {
        format!(
            "<!-- size={} point_count={} distance={} force={} -->",
            self.size, self.points, self.distance, self.force
        )
    }
}
