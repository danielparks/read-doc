//! Test the actual output of the macro.

use assert2::assert;

#[test]
fn read_doc_zero_files() {
    assert!(read_doc::module!() == "");
}

#[test]
fn read_doc_one_file() {
    assert!(
        read_doc::module!("fruit/apple.rs")
            == " ## Apple processing\n\n \
            Green or red, we don't care."
    );
}

#[test]
fn read_doc_two_files() {
    assert!(
        read_doc::module!("fruit/apple.rs", "fruit/orange.rs")
            == " ## Apple processing\n\n \
            Green or red, we don't care.\n\n \
            ## Orange processing\n\n \
            Various orange-related code."
    );
}

#[test]
fn read_block_docs() {
    assert!(
        read_doc::module!("doc_formats/block_docs.rs")
            == " ## Block-style docs\n\n\
            These use `/*! */` comments.\n"
    );
}

#[test]
fn read_doc_attribute() {
    assert!(
        read_doc::module!("doc_formats/attr_docs.rs")
            == "## Attribute-style docs\n\n\
            These use `#![doc = ...]` attributes."
    );
}

#[test]
fn read_line_docs() {
    assert!(
        read_doc::module!("doc_formats/line_docs.rs")
            == " ## Line-style docs\n\n \
            These use `//!` comments."
    );
}

#[test]
fn read_no_docs() {
    assert!(read_doc::module!("doc_formats/no_docs.rs") == "");
}
