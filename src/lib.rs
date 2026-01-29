//! Include docstrings from submodules.
//!
//! This crate provides macros that extract inner doc comments from Rust source
//! files and combine them into a string literal for use with `#[doc = ...]`.
//!
//! All doc comment formats are supported:
//! - `//!` line comments
//! - `/*! */` block comments
//! - `#![doc = "..."]` attributes
//!
//! # Example
//!
//! ```ignore
//! //! # Parent module docs
//!
//! #![doc = include_docs::include_docs!("src/child.rs")]
//!
//! mod child;
//! pub use child::*;
//! ```

// Lint configuration in Cargo.toml isn't supported by cargo-geiger.
#![forbid(unsafe_code)]
// Enable doc_cfg on docsrs so that we get feature markers.
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use include_docs_macro::{include_docs, include_module_docs};
