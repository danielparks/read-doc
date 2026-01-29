//! Include docstrings from submodules.
//!
//! This crate provides the [`include_docs`] attribute macro, which extracts
//! `//!` documentation comments from a submodule file and appends them to the
//! parent module's documentation.
//!
//! # Example
//!
//! ```ignore
//! //! # Parent module docs
//!
//! #[include_docs]
//! mod child;
//! pub use child::*;
//! ```

// Lint configuration in Cargo.toml isn't supported by cargo-geiger.
#![forbid(unsafe_code)]
// Enable doc_cfg on docsrs so that we get feature markers.
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use include_docs_macro::{include_docs, include_module_docs};
