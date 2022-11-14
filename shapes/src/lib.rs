#![warn(rust_2018_idioms)]
#![deny(
    dead_code,
    // NOTE: This is very helpful to include
    //missing_docs,
    unused_variables,
    unused_imports,
    unused_import_braces,
    rustdoc::broken_intra_doc_links,
    missing_debug_implementations,
    unreachable_pub
)]

pub mod blob;
pub mod circle;
pub mod path;
pub mod point;
pub mod pointmap;
pub mod rectangle;
pub mod shape;
