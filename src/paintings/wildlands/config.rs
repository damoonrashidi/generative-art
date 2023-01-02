#[derive(Debug)]
pub struct WildlandsConfig {
    pub seed: u32,
    pub size: f64,
    pub line_count: usize,
    pub chaos: f64,
    pub smoothness: f64,
    pub max_line_length: usize,
    pub radius: f64,
    pub step_size: f64,
    pub color_rounds: u8,
}
