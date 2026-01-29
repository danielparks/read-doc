use include_docs::include_docs;

fn main() {
    let _: &str = include_docs!("nonesuch.rs");
}
