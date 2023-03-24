| Continuous Integration |  Documentation   |      Crates.io       |
|:----------------------:|:----------------:|:--------------------:|
| [![ci][1]][2]          | [![docs][3]][4] | [![crates][5]][6]  |

[1]: https://github.com/Robbepop/enum-tag/actions/workflows/rust.yml/badge.svg
[2]: https://github.com/Robbepop/enum-tag/actions/workflows/rust.yml
[3]: https://docs.rs/enum-tag/badge.svg
[4]: https://docs.rs/enum-tag
[5]: https://img.shields.io/crates/v/enum-tag.svg
[6]: https://crates.io/crates/enum-tag

# `#[derive(EnumTag)]`

This crate provides a proc. macro to derive the `EnumTag` trait for the given Rust `enum`.
The `#derive(EnumTag)` proc. macro only works on Rust `enum` types and generates both

- a C-like `enum` type with the same variants as the input Rust `enum`
  but without all the associated data.
- a derived implementation of the `EnumTag` trait for the Rust `enum`

The derived `EnumTag` trait makes it possible to create instances of the generated
C-like `enum` type as well as link to its definition via `<RustEnum as EnumTag>::Tag`.

## When is this useful?

This is mostly useful for crates that profit from having a distinct `enum` tag type
while at the same time hosting Rust `enum` types with lots of variants which would
make it burdensome to maintain the mirroring between both `enum` type and `enum` tag type.

The motivation for this crate was a Wasm interpreter that represents its instructions
as an enum but also wants to access the opcodes of the instructions without their data.
In this example the opcodes are the instruction `enum` tag.

## Example

```rust
use ::enum_tag::EnumTag;

#[derive(EnumTag)]
enum Foo {
    A = 42,
    B(i32),
    C(i32, i64),
    D { a: i32 },
    E { a: i32, b: i64 },
}

/// This is how we can access the generated C-like enum type and name it.
type FooTag = <Foo as EnumTag>::Tag;

assert_eq!(Foo::A.tag(), FooTag::A);
assert_eq!(Foo::B(1).tag(), FooTag::B);
assert_eq!(Foo::C(2, 3).tag(), FooTag::C);
assert_eq!(Foo::D { a: 4 }.tag(), FooTag::D);
assert_eq!(Foo::E { a: 5, b: 6 }.tag(), FooTag::E);
```

The above `#[derive(EnumTag)]` generates the following Rust code:

```rust
const _: () = {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::core::cmp::PartialEq,
        ::core::cmp::Eq,
        ::core::cmp::PartialOrd,
        ::core::cmp::Ord,
        ::core::hash::Hash,
    )]
    pub enum FooTag {
        A = 42,
        B,
        C,
        D,
        E,
    }

    impl ::enum_tag::EnumTag for Foo {
        type Tag = FooTag;

        fn tag(&self) -> Self::Tag {
            match self {
                Self::A { .. } => Self::Tag::A,
                Self::B { .. } => Self::Tag::B,
                Self::C { .. } => Self::Tag::C,
                Self::D { .. } => Self::Tag::D,
                Self::E { .. } => Self::Tag::E,
            }
        }
    }
};
```
