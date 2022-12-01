#[deny(
    dead_code,
    unused_variables,
    missing_docs,
    unused_imports,
    unused_import_braces,
    rustdoc::all,
    missing_debug_implementations,
    unreachable_pub,
    clippy::all
)]

/// Generate a random number in a range, weighted towards the start of the range
pub mod gen_weighted;

/// Map a value from one range to another range
pub mod map;
