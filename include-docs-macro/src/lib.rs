//! Proc macro implementation for include-docs.
//!
//! Provides [`include_module_docs!`] which extracts inner doc comments from
//! a Rust source file at compile time.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::path::Path;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, Attribute, LitStr, Meta, Token};

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

/// Extract module-level documentation from a Rust source file.
///
/// This macro reads a Rust source file at compile time and extracts all inner
/// doc comments (`//!`, `/*! */`, `#![doc = "..."]`) from it, returning them
/// as a string literal suitable for use with `#[doc = ...]`.
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
            return syn::Error::new(input.path.span(), "CARGO_MANIFEST_DIR not set")
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

    let docs = match extract_inner_docs(&content) {
        Ok(d) => d,
        Err(e) => {
            return syn::Error::new(
                input.path.span(),
                format!("Failed to parse '{}': {e}", full_path.display()),
            )
            .to_compile_error()
            .into();
        }
    };

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
/// This macro extracts inner doc comments (`//!`, `/*! */`, `#![doc = "..."]`)
/// from multiple Rust source files and combines them into a single string
/// literal, suitable for use with `#[doc = ...]`.
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

        let docs = match extract_inner_docs(&content) {
            Ok(d) => d,
            Err(e) => {
                return syn::Error::new(
                    path_lit.span(),
                    format!("Failed to parse '{}': {e}", full_path.display()),
                )
                .to_compile_error()
                .into();
            }
        };

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

/// Extract inner doc comments from Rust source content using syn.
///
/// This handles all forms of inner documentation:
/// - `//! line doc`
/// - `/*! block doc */`
/// - `#![doc = "string"]`
fn extract_inner_docs(content: &str) -> Result<String, syn::Error> {
    // Parse as a file to get all the inner attributes
    let file = syn::parse_file(content)?;

    let mut docs = Vec::new();

    for attr in &file.attrs {
        if let Some(doc) = extract_doc_from_attr(attr) {
            docs.push(doc);
        }
    }

    Ok(docs.join("\n"))
}

/// Extract the doc string from a #[doc = "..."] attribute.
fn extract_doc_from_attr(attr: &Attribute) -> Option<String> {
    // Check if this is a doc attribute
    if !attr.path().is_ident("doc") {
        return None;
    }

    // Extract the value from #[doc = "value"]
    if let Meta::NameValue(meta) = &attr.meta
        && let syn::Expr::Lit(expr_lit) = &meta.value
        && let syn::Lit::Str(lit_str) = &expr_lit.lit
    {
        return Some(lit_str.value());
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_doc_comments() {
        let source = r#"
//! First line
//! Second line

fn foo() {}
"#;
        let result = extract_inner_docs(source).unwrap();
        assert_eq!(result, " First line\n Second line");
    }

    #[test]
    fn test_block_doc_comments() {
        let source = r#"
/*! Block doc comment
with multiple lines
*/

fn foo() {}
"#;
        let result = extract_inner_docs(source).unwrap();
        assert_eq!(result, " Block doc comment\nwith multiple lines\n");
    }

    #[test]
    fn test_doc_attributes() {
        let source = r#"
#![doc = "First line"]
#![doc = "Second line"]

fn foo() {}
"#;
        let result = extract_inner_docs(source).unwrap();
        assert_eq!(result, "First line\nSecond line");
    }

    #[test]
    fn test_mixed_doc_styles() {
        let source = r#"
//! Line comment
#![doc = "Attribute doc"]

fn foo() {}
"#;
        let result = extract_inner_docs(source).unwrap();
        assert_eq!(result, " Line comment\nAttribute doc");
    }

    #[test]
    fn test_no_docs() {
        let source = r#"
fn foo() {}
"#;
        let result = extract_inner_docs(source).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_only_outer_docs_ignored() {
        let source = r#"
/// This is an outer doc comment
fn foo() {}
"#;
        let result = extract_inner_docs(source).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_realistic_module() {
        let source = r#"
//! # Module Title
//!
//! This module does things.

use std::io;

/// Function doc
pub fn do_thing() {}
"#;
        let result = extract_inner_docs(source).unwrap();
        assert_eq!(result, " # Module Title\n\n This module does things.");
    }

    #[test]
    fn test_empty_doc_lines() {
        let source = r#"
//! First
//!
//! Third

fn foo() {}
"#;
        let result = extract_inner_docs(source).unwrap();
        assert_eq!(result, " First\n\n Third");
    }
}
