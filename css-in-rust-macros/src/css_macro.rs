use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::{
    css_class_impl_internal, css_if_impl_internal, css_impl_internal, css_multi_if_impl_internal,
};

/// CSS宏实现
pub fn css_impl(input: TokenStream) -> TokenStream {
    let input2 = TokenStream2::from(input);
    match css_impl_internal(input2) {
        Ok(tokens) => TokenStream::from(tokens),
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}

/// CSS条件宏实现
pub fn css_if_impl(input: TokenStream) -> TokenStream {
    let input2 = TokenStream2::from(input);
    match css_if_impl_internal(input2) {
        Ok(tokens) => TokenStream::from(tokens),
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}

/// CSS类宏实现
pub fn css_class_impl(input: TokenStream) -> TokenStream {
    let input2 = TokenStream2::from(input);
    match css_class_impl_internal(input2) {
        Ok(tokens) => TokenStream::from(tokens),
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}

/// CSS多条件宏实现
pub fn css_multi_if_impl(input: TokenStream) -> TokenStream {
    let input2 = TokenStream2::from(input);
    match css_multi_if_impl_internal(input2) {
        Ok(tokens) => TokenStream::from(tokens),
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}
