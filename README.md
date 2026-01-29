# include-docs

[![docs.rs](https://img.shields.io/docsrs/include-docs)][docs.rs]
[![Crates.io](https://img.shields.io/crates/v/include-docs)][crates.io]

Include inner doc comments from submodule files into parent module
documentation. Supports all doc comment formats: `//!`, `/*! */`, and
`#![doc = "..."]`.

## Example

The following documentation for the `fruit` module will be generated from the
three files below.

> ## Fruit functionality
>
> This has a lot of interesting functionality.
>
> ### Apple processing
>
> Green or red, we don't care.
>
> ### Orange processing
>
> Various orange-related code.

### `/src/fruit/mod.rs`

```rust
//! ## Fruit functionality
//!
//! This has a lot of interesting functionality.

#![doc = include_docs::include_docs!(
    "src/fruit/apple.rs",
    "src/fruit/orange.rs"
)]

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

## Macros

- `include_docs!("path1.rs", "path2.rs", ...)` - Include docs from multiple
  files, separated by blank lines.
- `include_module_docs!("path.rs")` - Include docs from a single file.

## Development status

This is in active development. I am open to [suggestions][issues].

## License

Unless otherwise noted, this project is dual-licensed under the Apache 2 and MIT
licenses. You may choose to use either.

  * [Apache License, Version 2.0](LICENSE-APACHE)
  * [MIT license](LICENSE-MIT)

### Contributions

Unless you explicitly state otherwise, any contribution you submit as defined
in the Apache 2.0 license shall be dual licensed as above, without any
additional terms or conditions.

[docs.rs]: https://docs.rs/include-docs/latest/combine_docs/
[crates.io]: https://crates.io/crates/include-docs
[issues]: https://github.com/danielparks/include-docs/issues
