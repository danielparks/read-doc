//! Tests that verify the actual output of the macros.

use include_docs::{include_docs, include_module_docs};

/// Test that include_module_docs extracts the expected content.
#[test]
fn test_include_module_docs_output() {
    let docs: &str = include_module_docs!("tests/fruit/apple.rs");
    assert_eq!(docs, " ## Apple processing\n\n Green or red, we don't care.");
}

/// Test that include_docs combines multiple files correctly.
#[test]
fn test_include_docs_combines_files() {
    let docs: &str = include_docs!(
        "tests/fruit/apple.rs",
        "tests/fruit/orange.rs"
    );

    // Should contain both modules' docs separated by a blank line
    assert!(docs.contains("Apple processing"), "Missing apple docs");
    assert!(docs.contains("Orange processing"), "Missing orange docs");
    assert!(docs.contains("Green or red"), "Missing apple content");
    assert!(docs.contains("Various orange-related"), "Missing orange content");

    // Verify the exact format
    let expected = " ## Apple processing\n\n Green or red, we don't care.\n\n ## Orange processing\n\n Various orange-related code.";
    assert_eq!(docs, expected);
}

/// Test block-style doc comments.
#[test]
fn test_block_docs_output() {
    let docs: &str = include_module_docs!("tests/doc_formats/block_docs.rs");
    assert!(docs.contains("Block-style docs"), "Missing block header");
    assert!(docs.contains("/*! */"), "Missing block comment reference");
}

/// Test attribute-style doc comments.
#[test]
fn test_attr_docs_output() {
    let docs: &str = include_module_docs!("tests/doc_formats/attr_docs.rs");
    assert_eq!(
        docs,
        "## Attribute-style docs\n\nThese use `#![doc = ...]` attributes."
    );
}

/// Test that all three doc formats produce output.
#[test]
fn test_all_doc_formats() {
    let line_docs: &str = include_module_docs!("tests/doc_formats/line_docs.rs");
    let block_docs: &str = include_module_docs!("tests/doc_formats/block_docs.rs");
    let attr_docs: &str = include_module_docs!("tests/doc_formats/attr_docs.rs");

    assert!(!line_docs.is_empty(), "Line docs should not be empty");
    assert!(!block_docs.is_empty(), "Block docs should not be empty");
    assert!(!attr_docs.is_empty(), "Attr docs should not be empty");
}

/// Test that empty files produce empty output.
#[test]
fn test_empty_docs() {
    // Create a module with no inner docs - just outer docs
    let docs: &str = include_module_docs!("tests/no_inner_docs.rs");
    assert_eq!(docs, "");
}
