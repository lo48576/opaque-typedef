//! Easy opaque typedef for Rust.
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

extern crate proc_macro;

use crate::{
    derive::base::{
        gen_base_sized, gen_base_sized_infallible, gen_base_sized_mut, gen_base_unsized,
        gen_base_unsized_infallible, gen_base_unsized_infallible_mut, gen_base_unsized_mut,
    },
    input::Input,
};

pub(crate) mod attr;
pub(crate) mod derive;
pub(crate) mod input;

/// The entrypoint for `#[derive(OpaqueTypedefSized)]`-ed types.
#[proc_macro_derive(OpaqueTypedefSized, attributes(opaque_typedef))]
pub fn opaque_typedef_sized(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse(input).unwrap();
    match Input::new(&input) {
        Ok(input) => gen_base_sized(&input).into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// The entrypoint for `#[derive(OpaqueTypedefSizedInfallible)]`-ed types.
#[proc_macro_derive(OpaqueTypedefSizedInfallible, attributes(opaque_typedef))]
pub fn opaque_typedef_sized_infallible(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse(input).unwrap();
    match Input::new(&input) {
        Ok(input) => gen_base_sized_infallible(&input)
            .unwrap_or_else(|e| e.to_compile_error())
            .into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// The entrypoint for `#[derive(OpaqueTypedefSizedMut)]`-ed types.
#[proc_macro_derive(OpaqueTypedefSizedMut, attributes(opaque_typedef))]
pub fn opaque_typedef_sized_mut(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse(input).unwrap();
    match Input::new(&input) {
        Ok(input) => gen_base_sized_mut(&input).into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// The entrypoint for `#[derive(OpaqueTypedefUnsized)]`-ed types.
#[proc_macro_derive(OpaqueTypedefUnsized, attributes(opaque_typedef))]
pub fn opaque_typedef_unsized(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse(input).unwrap();
    match Input::new(&input) {
        Ok(input) => gen_base_unsized(&input).into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// The entrypoint for `#[derive(OpaqueTypedefUnsizedInfallible)]`-ed types.
#[proc_macro_derive(OpaqueTypedefUnsizedInfallible, attributes(opaque_typedef))]
pub fn opaque_typedef_unsized_infallible(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = syn::parse(input).unwrap();
    match Input::new(&input) {
        Ok(input) => gen_base_unsized_infallible(&input)
            .unwrap_or_else(|e| e.to_compile_error())
            .into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// The entrypoint for `#[derive(OpaqueTypedefUnsizedMut)]`-ed types.
#[proc_macro_derive(OpaqueTypedefUnsizedMut, attributes(opaque_typedef))]
pub fn opaque_typedef_unsized_mut(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse(input).unwrap();
    match Input::new(&input) {
        Ok(input) => gen_base_unsized_mut(&input).into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// The entrypoint for `#[derive(OpaqueTypedefUnsizedInfallibleMut)]`-ed types.
#[proc_macro_derive(OpaqueTypedefUnsizedInfallibleMut, attributes(opaque_typedef))]
pub fn opaque_typedef_unsized_infallible_mut(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = syn::parse(input).unwrap();
    match Input::new(&input) {
        Ok(input) => gen_base_unsized_infallible_mut(&input)
            .unwrap_or_else(|e| e.to_compile_error())
            .into(),
        Err(e) => e.to_compile_error().into(),
    }
}
