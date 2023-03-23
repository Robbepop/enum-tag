use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[macro_use]
mod error;
mod derive;

/// Proc. macro to derive the `EnumTag` trait for the given Rust `enum`.
///
/// This proc. macro only works on Rust `enum` types and creates a C-like
/// `enum` with the same variants as the input Rust `enum` without all the
/// associated data.
/// Also it derives an implementation of `EnumTag` for the Rust `enum` that
/// makes it possible to create instances of the generated C-like `enum` as
/// well as link to its identifier via `<RustEnum as EnumTag>::Tag`.
///
/// # Example
///
/// ```
/// use ::enum_tag::EnumTag;
///
/// #[derive(EnumTag)]
/// enum Foo {
///     A,
///     B(i32),
///     C(i32, i64),
///     D { a: i32 },
///     E { a: i32, b: i64 },
/// }
///
/// /// This is how we can access the generated C-like enum type and name it.
/// type FooTag = <Foo as EnumTag>::Tag;
///
/// assert_eq!(Foo::A.tag(), FooTag::A);
/// assert_eq!(Foo::B(1).tag(), FooTag::B);
/// assert_eq!(Foo::C(2, 3).tag(), FooTag::C);
/// assert_eq!(Foo::D { a: 4 }.tag(), FooTag::D);
/// assert_eq!(Foo::E { a: 5, b: 6 }.tag(), FooTag::E);
/// ```
#[proc_macro_derive(EnumTag)]
pub fn enum_tag(input: TokenStream) -> TokenStream {
    derive::enum_tag(parse_macro_input!(input as DeriveInput)).into()
}