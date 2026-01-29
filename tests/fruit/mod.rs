//! # Fruit functionality
//!
//! This has a lot of interesting functionality.

#![doc = include_docs::include_docs!(
    "tests/fruit/apple.rs",
    "tests/fruit/orange.rs"
)]

mod apple;
pub use apple::*;

mod orange;
pub use orange::*;
