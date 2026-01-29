use include_docs::include_docs;

fn main() {
    // Pass an integer instead of a string
    let _: &str = include_docs!(123);
}
