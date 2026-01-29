//! Proc macro implementation for include-docs.
//!
//! Provides [`include_module_docs!`] which extracts `//!` documentation from
//! a Rust source file at compile time.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::path::Path;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, LitStr, Token};

/// Input for the `include_module_docs!` macro.
struct IncludeModuleDocsInput {
    /// Path to the file, relative to CARGO_MANIFEST_DIR.
    path: LitStr,
}

impl Parse for IncludeModuleDocsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            path: input.parse()?,
        })
    }
}

/// Extract module-level documentation (`//!` comments) from a Rust source file.
///
/// This macro reads a Rust source file at compile time and extracts all the
/// `//!` documentation comments from it, returning them as a string literal
/// suitable for use with `#[doc = ...]`.
///
/// # Example
///
/// ```ignore
/// //! # Parent module docs
///
/// #[doc = include_module_docs!("src/fruit/child.rs")]
/// mod child;
/// pub use child::*;
/// ```
///
/// The path is relative to `CARGO_MANIFEST_DIR` (typically the crate root).
#[proc_macro]
pub fn include_module_docs(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as IncludeModuleDocsInput);
    let path_str = input.path.value();

    let base_dir = match std::env::var("CARGO_MANIFEST_DIR") {
        Ok(dir) => dir,
        Err(_) => {
            return syn::Error::new(
                input.path.span(),
                "CARGO_MANIFEST_DIR not set",
            )
            .to_compile_error()
            .into();
        }
    };

    let full_path = Path::new(&base_dir).join(&path_str);

    let content = match std::fs::read_to_string(&full_path) {
        Ok(c) => c,
        Err(e) => {
            return syn::Error::new(
                input.path.span(),
                format!("Failed to read '{}': {e}", full_path.display()),
            )
            .to_compile_error()
            .into();
        }
    };

    let docs = extract_module_docs(&content);
    let lit = LitStr::new(&docs, Span::call_site());

    quote! { #lit }.into()
}

/// Input for `include_docs!` macro.
struct IncludeDocsInput {
    /// Paths to the files, relative to CARGO_MANIFEST_DIR.
    paths: Vec<LitStr>,
}

impl Parse for IncludeDocsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut paths = Vec::new();
        while !input.is_empty() {
            paths.push(input.parse()?);
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(Self { paths })
    }
}

/// Include module documentation from multiple files.
///
/// This macro extracts `//!` documentation comments from multiple Rust source
/// files and combines them into a single string literal, suitable for use with
/// `#[doc = ...]`.
///
/// # Example
///
/// ```ignore
/// //! # Fruit functionality
/// //!
/// //! This has a lot of interesting functionality.
///
/// #[doc = include_docs!("src/fruit/apple.rs", "src/fruit/orange.rs")]
/// mod fruit {
///     mod apple;
///     pub use apple::*;
///
///     mod orange;
///     pub use orange::*;
/// }
/// ```
///
/// Paths are relative to `CARGO_MANIFEST_DIR` (typically the crate root).
#[proc_macro]
pub fn include_docs(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as IncludeDocsInput);

    if input.paths.is_empty() {
        return syn::Error::new(Span::call_site(), "Expected at least one file path")
            .to_compile_error()
            .into();
    }

    let base_dir = match std::env::var("CARGO_MANIFEST_DIR") {
        Ok(dir) => dir,
        Err(_) => {
            return syn::Error::new(Span::call_site(), "CARGO_MANIFEST_DIR not set")
                .to_compile_error()
                .into();
        }
    };

    let mut all_docs = Vec::new();

    for path_lit in &input.paths {
        let path_str = path_lit.value();
        let full_path = Path::new(&base_dir).join(&path_str);

        let content = match std::fs::read_to_string(&full_path) {
            Ok(c) => c,
            Err(e) => {
                return syn::Error::new(
                    path_lit.span(),
                    format!("Failed to read '{}': {e}", full_path.display()),
                )
                .to_compile_error()
                .into();
            }
        };

        let docs = extract_module_docs(&content);
        if !docs.is_empty() {
            if !all_docs.is_empty() {
                all_docs.push(String::new()); // blank line separator
            }
            all_docs.push(docs);
        }
    }

    let combined = all_docs.join("\n");
    let lit = LitStr::new(&combined, Span::call_site());

    quote! { #lit }.into()
}

/// Extract `//!` doc comments from Rust source content.
fn extract_module_docs(content: &str) -> String {
    let mut docs = Vec::new();
    let mut in_doc_block = false;

    for line in content.lines() {
        let trimmed = line.trim_start();
        if let Some(doc) = trimmed.strip_prefix("//!") {
            in_doc_block = true;
            // Remove the leading space if present (standard formatting)
            let doc_content = doc.strip_prefix(' ').unwrap_or(doc);
            docs.push(doc_content.to_string());
        } else if in_doc_block {
            // Once we've started reading docs, stop at non-doc content
            if !trimmed.is_empty() && !trimmed.starts_with("//") {
                break;
            }
            // Allow blank lines within doc block
            if trimmed.is_empty() {
                docs.push(String::new());
            }
        }
    }

    // Trim trailing empty lines
    while docs.last().is_some_and(String::is_empty) {
        docs.pop();
    }

    docs.join("\n")
}
