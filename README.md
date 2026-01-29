# Macro to read module documentation from Rust source files

[![docs.rs](https://img.shields.io/docsrs/include-docs)][docs.rs]
[![Crates.io](https://img.shields.io/crates/v/include-docs)][crates.io]
![Rust version 1.88+](https://img.shields.io/badge/Rust%20version-1.88%2B-success)

This provides the [`include_docs!("path1.rs", "path2.rs", ...)`][include_docs]
macro, which reads module documentation from Rust source files.

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
#![doc = include_docs::include_docs!("apple.rs", "orange.rs")]

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

## Development status

This is in active development. I am open to [suggestions][issues].

## Minimum supported Rust version

Currently the minimum supported Rust version (MSRV) is **1.88**. Future
increases in the MSRV will require a major version bump.

## License

Unless otherwise noted, this project is dual-licensed under the Apache 2 and MIT
licenses. You may choose to use either.

  * [Apache License, Version 2.0](LICENSE-APACHE)
  * [MIT license](LICENSE-MIT)

### Contributions

Unless you explicitly state otherwise, any contribution you submit as defined
in the Apache 2.0 license shall be dual licensed as above, without any
additional terms or conditions.

[include_docs]: https://docs.rs/include-docs/latest/include_docs/macro.include_docs.html
[docs.rs]: https://docs.rs/include-docs/latest/include_docs/
[crates.io]: https://crates.io/crates/include-docs
[issues]: https://github.com/danielparks/include-docs/issues
