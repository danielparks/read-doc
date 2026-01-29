//! Ensure that the code itself compiles correctly.
#![allow(clippy::no_effect_underscore_binding, reason = "testing")]

mod doc_formats;
mod fruit;

#[test]
fn doc_formats() {
    let _ = doc_formats::LineDocs;
    let _ = doc_formats::BlockDocs;
    let _ = doc_formats::AttrDocs;
    let _ = doc_formats::NoDocs;
}

#[test]
fn fruit() {
    let _ = fruit::Apple;
    let _ = fruit::Orange;
}
