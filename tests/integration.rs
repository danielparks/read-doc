//! Ensure that the code itself compile correctly.
#![allow(clippy::no_effect_underscore_binding, reason = "testing")]

mod doc_formats;
mod fruit;

#[test]
fn test_fruit_types_accessible() {
    let _apple = fruit::Apple;
    let _orange = fruit::Orange;
}

#[test]
fn test_doc_formats_accessible() {
    let _line = doc_formats::LineDocs;
    let _block = doc_formats::BlockDocs;
    let _attr = doc_formats::AttrDocs;
}
