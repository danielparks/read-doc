//! # Various doc formats
//!
//! Testing different documentation styles.

#![doc = include_docs::include_docs!(
    "line_docs.rs",
    "block_docs.rs",
    "attr_docs.rs"
)]

mod line_docs;
pub use line_docs::*;

mod block_docs;
pub use block_docs::*;

mod attr_docs;
pub use attr_docs::*;
