//! `OpaqueTypedefUnsizedInfallible` codegen.

use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

use crate::input::Input;

/// Generate impl for `OpaqueTypedefUnsizedInfallible`.
pub fn gen_base_unsized_infallible(input: &Input) -> syn::Result<TokenStream> {
    if let Some(validator) = input.validator() {
        // A validator is specified and it may fail.
        return Err(syn::Error::new(
            validator.span(),
            "Validator and `OpaqueTypedefUnsizedInfallible` cannot be specified at the same time",
        ));
    }
    input.ensure_acceptable_unsized_repr_or_panic();

    let ty = input.ident();
    let (generics_impl, generics_ty, generics_where) = input.generics().split_for_impl();
    // This `unsafe` is safe if all the conditions below are met.
    //
    // * The type has just one unsized field and zero or more zero-sized field.
    //     + Rustc ensures "if there are unsized fields, it should be just one and all other fields
    //       are zero-sized types.
    //     + Currently, proc macro cannot check if a field has unsized type or not.
    //       Therefore, **library users are responsible to guarantee that**.
    // * The type has `#[repr(transparent)]` or `#[repr(C)]`.
    //     + This is already checked by `input.ensure_acceptable_unsized_repr_or_panic()`.
    //
    // This `unsafe` is necessary to convert an unsized type to a wrapper type.
    let expr_from_inner = quote!(unsafe { &*(__inner as *const Self::Inner as *const Self) });
    let base_impl_attrs = input.base_impl_attrs();

    Ok(quote! {
        #base_impl_attrs
        impl #generics_impl opaque_typedef::OpaqueTypedefUnsizedInfallible for #ty #generics_ty #generics_where {
            fn from_inner(__inner: &Self::Inner) -> &Self {
                #expr_from_inner
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_tuple_for_each_repr() {
        for repr in &[quote!(C), quote!(transparent)] {
            let input = syn::parse_quote! {
                #[derive(OpaqueTypedefUnsized, OpaqueTypedefUnsizedInfallible)]
                #[repr(#repr)]
                pub struct Simple<T>(T);
            };
            let toks = gen_base_unsized_infallible(&Input::new(&input).unwrap()).unwrap();
            let expected = quote! {
                impl<T> opaque_typedef::OpaqueTypedefUnsizedInfallible for Simple<T> {
                    fn from_inner(__inner: &Self::Inner) -> &Self {
                        unsafe { &*(__inner as *const Self::Inner as *const Self) }
                    }
                }
            };
            assert_eq!(toks.to_string(), expected.to_string());
        }
    }

    #[test]
    fn simple_struct() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefUnsized, OpaqueTypedefUnsizedInfallible)]
            #[repr(transparent)]
            pub struct Simple<T> {
                /// Inner data.
                inner: T,
            }
        };
        let toks = gen_base_unsized_infallible(&Input::new(&input).unwrap()).unwrap();
        let expected = quote! {
            impl<T> opaque_typedef::OpaqueTypedefUnsizedInfallible for Simple<T> {
                fn from_inner(__inner: &Self::Inner) -> &Self {
                    unsafe { &*(__inner as *const Self::Inner as *const Self) }
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn simple_tuple_trait_bound() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefUnsized, OpaqueTypedefUnsizedInfallible)]
            #[repr(transparent)]
            pub struct Simple<T: Debug>(T);
        };
        let toks = gen_base_unsized_infallible(&Input::new(&input).unwrap()).unwrap();
        let expected = quote! {
            impl<T: Debug> opaque_typedef::OpaqueTypedefUnsizedInfallible for Simple<T> {
                fn from_inner(__inner: &Self::Inner) -> &Self {
                    unsafe { &*(__inner as *const Self::Inner as *const Self) }
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn multi_field_struct() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefUnsized, OpaqueTypedefUnsizedInfallible)]
            #[repr(transparent)]
            pub struct Tagged<T, Tag> {
                /// Inner data.
                #[opaque_typedef::inner]
                inner: T,
                /// Tag.
                tag: Tag,
            }
        };
        let toks = gen_base_unsized_infallible(&Input::new(&input).unwrap()).unwrap();
        let expected = quote! {
            impl<T, Tag> opaque_typedef::OpaqueTypedefUnsizedInfallible for Tagged<T, Tag> {
                fn from_inner(__inner: &Self::Inner) -> &Self {
                    unsafe { &*(__inner as *const Self::Inner as *const Self) }
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn simple_tuple_explicit_inner() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefUnsized, OpaqueTypedefUnsizedInfallible)]
            #[repr(transparent)]
            pub struct Simple<T>(#[opaque_typedef(inner)] T);
        };
        let toks = gen_base_unsized_infallible(&Input::new(&input).unwrap()).unwrap();
        let expected = quote! {
            impl<T> opaque_typedef::OpaqueTypedefUnsizedInfallible for Simple<T> {
                fn from_inner(__inner: &Self::Inner) -> &Self {
                    unsafe { &*(__inner as *const Self::Inner as *const Self) }
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn simple_tuple_hide_impl_docs() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefUnsized, OpaqueTypedefUnsizedInfallible)]
            #[repr(transparent)]
            #[opaque_typedef(hide_base_impl_docs)]
            pub struct Simple<T>(pub T);
        };
        let toks = gen_base_unsized_infallible(&Input::new(&input).unwrap()).unwrap();
        let expected = quote! {
            #[doc(hidden)]
            impl<T> opaque_typedef::OpaqueTypedefUnsizedInfallible for Simple<T> {
                fn from_inner(__inner: &Self::Inner) -> &Self {
                    unsafe { &*(__inner as *const Self::Inner as *const Self) }
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    /// Types without `#[repr(C)]` and `#[repr(transparent)]` should be rejected.
    #[test]
    #[should_panic]
    fn no_repr() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefUnsized, OpaqueTypedefUnsizedInfallible)]
            struct MyStr(str);
        };
        let _ = gen_base_unsized_infallible(&Input::new(&input).unwrap()).unwrap();
    }

    #[test]
    fn with_validator() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefUnsized, OpaqueTypedefUnsizedInfallible)]
            #[opaque_typedef(validate(error = "Error", validator = "validate"))]
            pub struct Simple<T>(T);
        };
        let toks = gen_base_unsized_infallible(&Input::new(&input).unwrap());
        assert!(toks.is_err());
    }
}
