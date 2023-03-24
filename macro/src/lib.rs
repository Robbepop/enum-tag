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
/// #[repr(u8)] // Rust needs this for `B = 42`
/// enum Foo {
///     A,
///     B = 42,
///     C(i32),
///     D(i32, i64),
///     E { a: i32 },
///     F { a: i32, b: i64 },
/// }
///
/// /// This is how we can access the generated C-like enum type and name it.
/// type FooTag = <Foo as EnumTag>::Tag;
///
/// assert_eq!(FooTag::A, Foo::A.tag());
/// assert_eq!(FooTag::B, Foo::B.tag());
/// assert_eq!(FooTag::C, Foo::C(1).tag());
/// assert_eq!(FooTag::D, Foo::D(2, 3).tag());
/// assert_eq!(FooTag::E, Foo::E { a: 4 }.tag());
/// assert_eq!(FooTag::F, Foo::F { a: 5, b: 6 }.tag());
///
/// assert_eq!(FooTag::B as u8, 42);
/// ```
#[proc_macro_derive(EnumTag)]
pub fn enum_tag(input: TokenStream) -> TokenStream {
    derive::enum_tag(parse_macro_input!(input as DeriveInput)).into()
}
