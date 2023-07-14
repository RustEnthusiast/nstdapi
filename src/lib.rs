//! A helper crate for `nstd` providing manipulation of an item.
#![no_std]
use cfg_if::cfg_if;
use proc_macro::TokenStream;

/// Manipulates an item based on the crate's features.
#[proc_macro_attribute]
pub fn nstdapi(_: TokenStream, item: TokenStream) -> TokenStream {
    cfg_if! {
        if #[cfg(any(feature = "capi", feature = "link"))] {
            use quote::ToTokens;
            use syn::{
                parse_quote, FnArg, ForeignItem, ForeignItemFn, ForeignItemStatic, Item,
                ItemForeignMod, Pat,
            };
            let mut input = syn::parse(item).expect("failed to parse an item with `nstdapi`");
            if cfg!(feature = "capi") {
                match &mut input {
                    Item::Struct(s) => s.attrs.push(parse_quote!(#[repr(C)])),
                    Item::Enum(e) => e.attrs.push(parse_quote!(#[repr(C)])),
                    Item::Union(u) => u.attrs.push(parse_quote!(#[repr(C)])),
                    _ => {}
                }
            }
            if cfg!(feature = "link") {
                input = match input {
                    Item::Fn(f) => {
                        let mut sig = f.sig;
                        sig.unsafety = None;
                        sig.constness = None;
                        for arg in &mut sig.inputs {
                            match arg {
                                FnArg::Receiver(arg) => {
                                    arg.mutability = None;
                                }
                                FnArg::Typed(pat) => {
                                    if let Pat::Ident(arg) = &mut *pat.pat {
                                        arg.mutability = None;
                                    }
                                }
                            }
                        }
                        Item::ForeignMod(ItemForeignMod {
                            attrs: [parse_quote!(#[allow(unused_attributes)])].into(),
                            unsafety: None,
                            #[cfg(not(feature = "capi"))]
                            abi: parse_quote!(extern "Rust"),
                            #[cfg(feature = "capi")]
                            abi: parse_quote!(extern "C"),
                            brace_token: Default::default(),
                            items: [ForeignItem::Fn(ForeignItemFn {
                                attrs: f.attrs,
                                vis: f.vis,
                                sig,
                                semi_token: Default::default(),
                            })].into(),
                        })
                    }
                    Item::Static(s) => Item::ForeignMod(ItemForeignMod {
                        attrs: [parse_quote!(#[allow(unused_attributes)])].into(),
                        unsafety: None,
                        #[cfg(not(feature = "capi"))]
                        abi: parse_quote!(extern "Rust"),
                        #[cfg(feature = "capi")]
                        abi: parse_quote!(extern "C"),
                        brace_token: Default::default(),
                        items: [ForeignItem::Static(ForeignItemStatic {
                            attrs: s.attrs,
                            vis: s.vis,
                            static_token: Default::default(),
                            mutability: s.mutability,
                            ident: s.ident,
                            colon_token: Default::default(),
                            ty: s.ty,
                            semi_token: Default::default(),
                        })].into(),
                    }),
                    input => input
                };
            } else if cfg!(feature = "capi") {
                match &mut input {
                    Item::Fn(f) => {
                        f.attrs.push(parse_quote!(#[no_mangle]));
                        f.sig.abi = parse_quote!(extern "C");
                    }
                    Item::Static(s) => s.attrs.push(parse_quote!(#[no_mangle])),
                    _ => {}
                }
            }
            input.into_token_stream().into()
        } else {
            item
        }
    }
}
