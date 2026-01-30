//! Read module documentation from Rust source files.
//!
//! # Minimum supported Rust version
//!
//! Currently the minimum supported Rust version (MSRV) is **1.88**. Future
//! increases in the MSRV will require a major version bump.

// Lint configuration in Cargo.toml isn't supported by cargo-geiger.
#![forbid(unsafe_code)]
// Enable doc_cfg on docsrs so that we get feature markers.
#![cfg_attr(docsrs, feature(doc_cfg))]

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::fs;
use std::path::{Path, PathBuf};
use syn::{
    LitStr, Meta, Token, parse::Parse, parse::ParseStream, parse_macro_input,
};

/// Input for `module!` macro.
struct ModuleInput {
    /// Paths to the files, relative to the directory of the calling file.
    paths: Vec<LitStr>,
}

impl Parse for ModuleInput {
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

/// # Read module documentation from Rust source files.
///
/// ```ignore
/// //! # Overall module documentation
/// #![doc = read_doc::module!("submodule1.rs", "submodule2.rs")]
///
/// mod submodule1;
/// mod submodule2;
/// ```
///
/// This macro extracts inner doc comments from the passed Rust source files and
/// combines them into a string that can be used with `#[doc = ...]`.
///
/// Each file’s module documentation will be separated by a blank line.
///
/// Paths are relative to the directory containing the calling file.
///
/// # Example
///
/// Given the source files below, `cargo doc` will produce the following
/// documentation:
///
/// ```Markdown
/// # Fruit functionality
///
/// This has a lot of interesting functionality.
///
/// ### Apple processing
///
/// Green or red, we don't care.
///
/// ### Orange processing
///
/// Various orange-related code.
/// ```
///
/// ### `/src/fruit/mod.rs`
///
/// ```rust,ignore
/// //! # Fruit functionality
/// //!
/// //! This has a lot of interesting functionality.
/// #![doc = read_doc::module!("apple.rs", "orange.rs")]
///
/// mod apple;
/// pub use apple::*;
///
/// mod orange;
/// pub use orange::*;
/// ```
///
/// ### `/src/fruit/apple.rs`
///
/// ```rust
/// //! ### Apple processing
/// //!
/// //! Green or red, we don't care.
///
/// /// Sweet or tart.
/// pub struct Apple;
/// ```
///
/// ### `/src/fruit/orange.rs`
///
/// ```rust
/// //! ### Orange processing
/// //!
/// //! Various orange-related code.
///
/// /// A round fruit.
/// pub struct Orange;
/// ```
#[proc_macro]
pub fn module(input: TokenStream) -> TokenStream {
    fn inner(input: &ModuleInput) -> syn::Result<TokenStream> {
        let base_dir = get_source_dir()?;

        let docs = input
            .paths
            .iter()
            .filter_map(|path_lit| {
                let path = Path::new(&base_dir).join(path_lit.value());
                fs::read_to_string(&path)
                    .map_err(|error| error.to_string())
                    .and_then(|content| extract_inner_docs(&content))
                    .map(|docs| if docs.is_empty() { None } else { Some(docs) })
                    .map_err(|error| {
                        syn::Error::new(
                            path_lit.span(),
                            format!("Failed to read {path:?}: {error}"),
                        )
                    })
                    .transpose()
            })
            .collect::<syn::Result<Vec<String>>>()?
            .join("\n\n"); // FIXME all errors

        let lit = LitStr::new(&docs, Span::call_site());
        Ok(quote! { #lit }.into())
    }

    match inner(&parse_macro_input!(input as ModuleInput)) {
        Ok(stream) => stream,
        Err(error) => error.to_compile_error().into(),
    }
}

/// Extract inner doc comments from Rust source.
///
/// # Errors
///
/// Returns an error if there was a problem parsing the file.
fn extract_inner_docs(content: &str) -> Result<String, String> {
    Ok(syn::parse_file(content)
        .map_err(|error| error.to_string())?
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
