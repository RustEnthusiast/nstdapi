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
        let mut input = syn::parse(item).expect("failed to parse an item with `nstdapi`");
        match &mut input {
            Item::Fn(f) => {
                f.attrs.push(parse_quote!(#[no_mangle]));
                f.sig.abi = parse_quote!(extern "C");
            }
            Item::Static(s) => s.attrs.push(parse_quote!(#[no_mangle])),
            Item::Struct(s) => s.attrs.push(parse_quote!(#[repr(C)])),
            Item::Enum(e) => e.attrs.push(parse_quote!(#[repr(C)])),
            Item::Union(u) => u.attrs.push(parse_quote!(#[repr(C)])),
            _ => {}
        }
        input.into_token_stream().into()
    }
}
