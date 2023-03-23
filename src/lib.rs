#![no_std]

pub use enum_tag_macro::EnumTag;

/// Trait implemented by `enum` types.
///
/// This trait usually is implemented via `#[derive(EnumTag)]`.
pub trait EnumTag {
    /// The type of the `enum`'s tag.
    type Tag;

    /// Returns the tag of `self` where `Self` is a Rust `enum` type.
    fn tag(&self) -> Self::Tag;
}
