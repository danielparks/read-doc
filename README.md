# Macro to read module documentation from Rust source files

[![docs.rs](https://img.shields.io/docsrs/read-doc)][docs.rs]
[![Crates.io](https://img.shields.io/crates/v/read-doc)][crates.io]
![Rust version 1.88+](https://img.shields.io/badge/Rust%20version-1.88%2B-success)

```rust
//! # Overall module documentation
#![doc = read_doc::module!("submodule1.rs", "submodule2.rs")]

mod submodule1;
mod submodule2;
```

The [`read_doc::module!("path1.rs", ...)`][macro] macro reads module
documentation from Rust source files as a string.

This is useful if you want to arrange your code logically in private submodules
along with summary module documentation, then re-export the items and overview
documentation in the public parent module.

## Example

The following documentation for the `fruit` module will be generated from the
three files below.

```markdown
## Fruit functionality

This has a lot of interesting functionality.

### Apple processing

Green or red, we don't care.

### Orange processing

Various orange-related code.
```

### `/src/fruit/mod.rs`

```rust
//! ## Fruit functionality
//!
//! This has a lot of interesting functionality.
#![doc = read_doc::module!("apple.rs", "orange.rs")]

mod apple;
pub use apple::*;

mod orange;
pub use orange::*;
```

### `/src/fruit/apple.rs`

```rust
//! ### Apple processing
//!
//! Green or red, we don't care.

/// Sweet or tart.
pub struct Apple;
```

### `/src/fruit/orange.rs`

```rust
//! ### Orange processing
//!
//! Various orange-related code.

/// A round fruit.
pub struct Orange;
```

## Similar crates

I found three crates that allow access to doc comments on _types_ at runtime. I
have not tried them, but here they are, listed in order of most to least simple:

  1. [documented](https://docs.rs/documented/latest/documented/)
  2. [doc_for](https://docs.rs/doc_for/latest/doc_for/)
  3. [user_doc](https://docs.rs/user_doc/latest/user_doc/)

## Minimum supported Rust version

Currently the minimum supported Rust version (MSRV) is **1.88**. Future
increases in the MSRV will require a major version bump.

## Development

This is in active development. I am open to [suggestions][issues].

### LLM use

LLM produced code must be marked in the commit message. I typically ask the LLM
to make its own commits, then I make my own changes after careful review.

Example: 62ab2857d8d17fdf2d7db1f902b6281732854648 was written by Claude; I
cleaned up that commit in 63e1a97c17f4c9452d59309c1e26a226f1155009.

### License

Unless otherwise noted, this project is dual-licensed under the Apache 2 and MIT
licenses. You may choose to use either.

  * [Apache License, Version 2.0](LICENSE-APACHE)
  * [MIT license](LICENSE-MIT)

### Contributions

Unless you explicitly state otherwise, any contribution you submit as defined
in the Apache 2.0 license shall be dual licensed as above, without any
additional terms or conditions.

[macro]: https://docs.rs/read-doc/latest/read_doc/macro.module.html
[docs.rs]: https://docs.rs/read-doc/latest/read_doc/
[crates.io]: https://crates.io/crates/read-doc
[issues]: https://github.com/danielparks/read-doc/issues
