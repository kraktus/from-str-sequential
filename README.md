# from-str-sequential

[![crates.io](https://img.shields.io/crates/v/from-str-sequential.svg)](https://crates.io/crates/from-str-sequential)
[![docs.rs](https://docs.rs/from-str-sequential/badge.svg)](https://docs.rs/from-str-sequential)

A utility crate which implement a simple `FromStrSequential` trait similar to `FromStr`. Used on enums with unit and un-named variants, and try to convert the string to each variant sequentially (from top to bottom variant). For unit variant,
the string must be the variant name (case-insentive). For un-named variants, the string must match the `FromStr` implementation of the un-named type.

This crate was initially released to allow multiple input formats for `clap::Command`

## Example

```rust
use from_str_sequential::FromStrSequential;

#[derive(FromStrSequential)]
enum Foo {
	Bar,
	Baz(usize),
}

assert_eq!(Foo::Bar, Foo::from_str_sequential("bar"));
assert_eq!(Foo::Bar, Foo::from_str_sequential("BaR"));
assert_eq!(Foo::Baz(100), Foo::from_str_sequential("100"));

#[derive(clap::Args, Debug)]
pub struct Cli {
    #[arg(value_parser = Foo::from_str_sequential)]
    foo: Foo
}
```