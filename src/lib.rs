//! Read module documentation from Rust source files.

// Lint configuration in Cargo.toml isn't supported by cargo-geiger.
#![forbid(unsafe_code)]
// Enable doc_cfg on docsrs so that we get feature markers.
#![cfg_attr(docsrs, feature(doc_cfg))]

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::path::{Path, PathBuf};
use syn::{
    LitStr, Meta, Token, parse::Parse, parse::ParseStream, parse_macro_input,
};

/// Input for `include_docs!` macro.
struct IncludeDocsInput {
    /// Paths to the files, relative to the directory of the calling file.
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

/// Read module documentation from files.
///
/// This macro extracts inner doc comments (`//!`, `/*! */`, `#![doc = "..."]`)
/// from the passed Rust source files and combines them into a single string
/// literal that can be used with `#[doc = ...]`.
///
/// Each file’s module documentation will be separated by a blank line.
///
/// Paths are relative to the directory containing the calling file.
///
/// # Example
///
/// ```ignore
/// //! # Fruit functionality
/// //!
/// //! This has a lot of interesting functionality.
/// #[doc = include_docs!("apple.rs", "orange.rs")]
///
/// mod apple;
/// pub use apple::*;
///
/// mod orange;
/// pub use orange::*;
/// ```
#[proc_macro]
pub fn include_docs(input: TokenStream) -> TokenStream {
    fn inner(input: &IncludeDocsInput) -> syn::Result<TokenStream> {
        let base_dir = get_source_dir()?;

        let mut all_docs = Vec::new();

        for path_lit in &input.paths {
            let path_str = path_lit.value();
            let full_path = Path::new(&base_dir).join(&path_str);

            let content =
                std::fs::read_to_string(&full_path).map_err(|error| {
                    syn::Error::new(
                        path_lit.span(),
                        format!("Failed to read {full_path:?}: {error}"),
                    )
                })?;

            let docs = extract_inner_docs(&content).map_err(|error| {
                syn::Error::new(
                    path_lit.span(),
                    format!("Failed to parse {full_path:?}: {error}"),
                )
            })?;

            if !docs.is_empty() {
                if !all_docs.is_empty() {
                    all_docs.push(String::new()); // blank line separator
                }
                all_docs.push(docs);
            }
        }

        let combined = all_docs.join("\n");
        let lit = LitStr::new(&combined, Span::call_site());

        Ok(quote! { #lit }.into())
    }

    match inner(&parse_macro_input!(input as IncludeDocsInput)) {
        Ok(stream) => stream,
        Err(error) => error.to_compile_error().into(),
    }
}

/// Extract inner doc comments from Rust source.
fn extract_inner_docs(content: &str) -> syn::Result<String> {
    Ok(syn::parse_file(content)?
        .attrs
        .into_iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc")
                && let Meta::NameValue(meta) = &attr.meta
                && let syn::Expr::Lit(expr_lit) = &meta.value
                && let syn::Lit::Str(lit_str) = &expr_lit.lit
            {
                Some(lit_str.value())
            } else {
                // Skip attributes other than a doc attributes with a value.
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n"))
}

/// Get the directory containing the source file that called the macro.
///
/// # Errors
///
/// Returns and error if source didn’t have a path, or if we couldn’t get the
/// parent of that path.
fn get_source_dir() -> Result<PathBuf, syn::Error> {
    match Span::call_site()
        .local_file()
        .and_then(|path| path.parent().map(Path::to_path_buf))
    {
        Some(path) => Ok(path),
        None => Err(syn::Error::new(
            Span::call_site(),
            "Could not get path to source file",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::assert;

    #[test]
    fn line_doc_comments() {
        assert!(
            extract_inner_docs(
                r"
//! First line
//! Second line

fn foo() {}
"
            )
            .unwrap()
                == " First line\n Second line"
        );
    }

    #[test]
    fn mixed_attrs() {
        assert!(
            extract_inner_docs(
                r"
//! First line
#![forbid(unsafe_code)]
//! Second line

fn foo() {}
"
            )
            .unwrap()
                == " First line\n Second line"
        );
    }

    #[test]
    fn block_doc_comments() {
        assert!(
            extract_inner_docs(
                r"
/*! Block doc comment
with multiple lines
*/

fn foo() {}
"
            )
            .unwrap()
                == " Block doc comment\nwith multiple lines\n"
        );
    }

    #[test]
    fn doc_attributes() {
        assert!(
            extract_inner_docs(
                r#"
#![doc = "First line"]
#![doc = "Second line"]

fn foo() {}
"#
            )
            .unwrap()
                == "First line\nSecond line"
        );
    }

    #[test]
    fn mixed_doc_styles() {
        assert!(
            extract_inner_docs(
                r#"
//! Line comment
#![doc = "Attribute doc"]

fn foo() {}
"#
            )
            .unwrap()
                == " Line comment\nAttribute doc"
        );
    }

    #[test]
    fn no_docs() {
        assert!(extract_inner_docs("fn foo() {}\n").unwrap() == "");
    }

    #[test]
    fn only_outer_docs_ignored() {
        assert!(
            extract_inner_docs(
                r"
/// This is an outer doc comment
fn foo() {}
"
            )
            .unwrap()
                == ""
        );
    }

    #[test]
    fn realistic_module() {
        assert!(
            extract_inner_docs(
                r"
//! # Module Title
//!
//! This module does things.

use std::io;

/// Function doc
pub fn do_thing() {}
"
            )
            .unwrap()
                == " # Module Title\n\n This module does things."
        );
    }

    #[test]
    fn empty_doc_lines() {
        assert!(
            extract_inner_docs(
                r"
//! First
//!
//! Third

fn foo() {}
"
            )
            .unwrap()
                == " First\n\n Third"
        );
    }
}
