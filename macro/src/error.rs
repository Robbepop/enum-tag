//! Utilies for proc. macro error reporting.

macro_rules! bail_spanned {
    ($tokens:expr, $($msg:tt)*) => {
        return ::core::result::Result::Err(
            ::syn::Error::new_spanned(
                &$tokens,
                format_args!($($msg)*)
            )
        )
    }
}
