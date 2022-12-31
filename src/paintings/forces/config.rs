use std::fmt::Display;

use clap::Parser;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForcesPalette {
    PeachesAndCream,
    OrangeAutumn,
    SpringBreak,
    RedWhiteBlack,
}

impl Default for ForcesPalette {
    fn default() -> Self {
        ForcesPalette::OrangeAutumn
    }
}

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
pub struct ForcesParams {
    /// Set the size of the final SVG output
    #[arg(long, default_value_t = 1500.0)]
    pub size: f64,

    /// Number of lines to attempt to fill the image with
    #[arg(long, default_value_t = 5000)]
    pub line_count: usize,

    /// Minimum length for a line, all lines that would be shorter will be discarded
    #[arg(long, default_value_t = 80.0)]
    pub min_line_length: f64,

    /// Maximum length for a line
    #[arg(long, default_value_t = 2500.0)]
    pub max_line_length: f64,

    /// Color palette to use for each line and background, some are weighted some are not.
    #[arg(long, default_value_t = String::from("peaches_and_cream"))]
    pub palette: String,

    /// How much each turn in a line is exaggerated, the higher the number the higher the more chaotic the output
    #[arg(long, default_value_t = 1.8)]
    pub chaos: f64,

    /// How much to smooth out the line curves, the higher the number the smoother the lines
    #[arg(long, default_value_t = 1200.0)]
    pub smoothness: f64,

    /// Seed for the RNG
    #[arg(long)]
    pub seed: u32,

    /// The probability that a line has more than one color
    #[arg(long, default_value_t = 0.0)]
    pub split_line_chance: f64,

    /// If line is split into several should there be a gap between new lines
    #[arg(long, default_value_t = false)]
    pub split_with_gap: bool,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ForcesConfig {
    pub size: f64,
    pub line_count: usize,
    pub min_line_length: f64,
    pub max_line_length: f64,
    pub palette: ForcesPalette,
    pub chaos: f64,
    pub smoothness: f64,
    pub seed: u32,
    pub split_line_chance: f64,
    pub split_with_gap: bool,
}

impl ForcesConfig {
    pub fn new() -> ForcesConfig {
        let args = ForcesParams::parse();

        println!("{}", args.palette);

        let palette = match args.palette.to_ascii_lowercase().as_str() {
            "peaches_and_cream" => ForcesPalette::PeachesAndCream,
            "orange_autumn" => ForcesPalette::OrangeAutumn,
            "spring_break" => ForcesPalette::SpringBreak,
            "red_white_black" => ForcesPalette::RedWhiteBlack,
            _ => panic!(
                "{} is not a valid palette, valid values are peaches_and_cream, orange_autumn, spring_break, red_white_black",
                args.palette
            ),
        };

        ForcesConfig {
            line_count: args.line_count,
            min_line_length: args.min_line_length,
            max_line_length: args.max_line_length,
            palette,
            chaos: args.chaos,
            seed: args.seed,
            smoothness: args.smoothness,
            split_line_chance: args.split_line_chance,
            split_with_gap: args.split_with_gap,
            size: args.size,
        }
    }
}

impl Display for ForcesConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (
            size,
            line_count,
            min_line_length,
            max_line_length,
            chaos,
            smoothness,
            seed,
            split_line_chance,
            split_with_gap,
        ) = (
            self.size,
            self.line_count,
            self.min_line_length,
            self.max_line_length,
            self.chaos,
            self.smoothness,
            self.seed,
            self.split_line_chance,
            self.split_with_gap,
        );

        write!(
            f,
            r#"
            <!--
            size={size}
            line-count={line_count}
            min-line-length={min_line_length} 
            max-line-length={max_line_length}
            chaos={chaos}
            smoothness={smoothness}
            seed={seed}
            split-line-chance={split_line_chance}
            split-with-gap={split_with_gap}
            -->"#,
        )
    }
}
