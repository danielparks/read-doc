//! # Fruit functionality
//!
//! This has a lot of interesting functionality.

#![doc = include_docs::module!(
    "apple.rs",
    "orange.rs"
)]

mod apple;
pub use apple::*;

mod orange;
pub use orange::*;
