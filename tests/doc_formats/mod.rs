//! # Various doc formats
//!
//! Testing different documentation styles.

#![doc = include_docs::include_docs!(
    "tests/doc_formats/line_docs.rs",
    "tests/doc_formats/block_docs.rs",
    "tests/doc_formats/attr_docs.rs"
)]

mod line_docs;
pub use line_docs::*;

mod block_docs;
pub use block_docs::*;

mod attr_docs;
pub use attr_docs::*;
