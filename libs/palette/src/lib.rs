#![warn(rust_2018_idioms)]
#![deny(
    dead_code,
    unused_variables,
    unused_imports,
    unused_import_braces,
    rustdoc::broken_intra_doc_links,
    missing_debug_implementations,
    unreachable_pub
)]

use color::Color;

pub mod color;
pub mod palettes;
pub mod simple_palette;
pub mod weighted_palette;

/// A list of colors where a single color can be picked randomly ([`SimplePalette`])
/// more controlled based on some weights ([`WeightedPalette`])
pub trait Palette {
    /// Get a random color from the given palette
    fn get_random_color(&self) -> Option<Color>;
}
