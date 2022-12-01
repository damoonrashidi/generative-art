/**
Palettes, colors and other stuff
*/
#[deny(
    dead_code,
    missing_docs,
    unused_variables,
    unused_imports,
    unused_import_braces,
    rustdoc::broken_intra_doc_links,
    missing_debug_implementations,
    unreachable_pub,
    clippy::all
)]
use color::Color;

/// Color
pub mod color;

/// Palettes
pub mod palettes;

/// Simple Palette
pub mod simple_palette;

/// Weighted Palette
pub mod weighted_palette;

/// A list of colors where a single color can be picked randomly ([`SimplePalette`])
/// or more controlled based on some weights ([`WeightedPalette`])
pub trait Palette {
    /// Get a random color from the given palette
    fn get_random_color(&self) -> Option<Color>;
}
