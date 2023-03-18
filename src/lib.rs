//! A helper crate for `nstd` providing manipulation of an item's ABI attributes.
#![no_std]
use proc_macro::TokenStream;

/// Provides an item with the appropriate attributes and ABI based on the crate's features.
#[proc_macro_attribute]
pub fn nstdapi(_: TokenStream, item: TokenStream) -> TokenStream {
    #[cfg(not(feature = "capi"))]
    return item;
    #[cfg(feature = "capi")]
    {
        use quote::ToTokens;
        use syn::{parse_quote, Item};
        let input = syn::parse(item).expect("failed to parse an item with `nstdapi`");
        match input {
            Item::Fn(mut f) => {
                f.attrs.push(parse_quote!(#[no_mangle]));
                f.sig.abi = parse_quote!(extern "C");
                f.into_token_stream()
            }
            Item::Static(mut s) => {
                s.attrs.push(parse_quote!(#[no_mangle]));
                s.into_token_stream()
            }
            Item::Struct(mut s) => {
                s.attrs.push(parse_quote!(#[repr(C)]));
                s.into_token_stream()
            }
            Item::Enum(mut e) => {
                e.attrs.push(parse_quote!(#[repr(C)]));
                e.into_token_stream()
            }
            Item::Union(mut u) => {
                u.attrs.push(parse_quote!(#[repr(C)]));
                u.into_token_stream()
            }
            input => input.into_token_stream(),
        }
        .into()
    }
}
