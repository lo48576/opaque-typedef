//! `OpaqueTypedefUnsizedInfallibleMut` codegen.

use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

use crate::input::Input;

/// Generate impl for `OpaqueTypedefUnsizedInfallibleMut`.
pub fn gen_base_unsized_infallible_mut(input: &Input) -> syn::Result<TokenStream> {
    if let Some(validator) = input.validator() {
        // A validator is specified and it may fail.
        return Err(syn::Error::new(
            validator.span(),
            "Validator and `OpaqueTypedefUnsizedInfallibleMut` cannot be specified at the same time",
        ));
    }
    input.ensure_acceptable_unsized_repr_or_panic();

    let ty = input.ident();
    let (generics_impl, generics_ty, generics_where) = input.generics().split_for_impl();
    let expr_from_inner = quote!(unsafe { &mut *(__inner as *mut Self::Inner as *mut Self) });
    let base_impl_attrs = input.base_impl_attrs();

    Ok(quote! {
        #base_impl_attrs
        impl #generics_impl opaque_typedef::OpaqueTypedefUnsizedInfallibleMut for #ty #generics_ty #generics_where {
            fn from_inner_mut(__inner: &mut Self::Inner) -> &mut Self {
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
                #[derive(
                    OpaqueTypedefUnsized,
                    OpaqueTypedefUnsizedInfallible,
                    OpaqueTypedefUnsizedMut,
                    OpaqueTypedefUnsizedInfallibleMut
                )]
                #[repr(#repr)]
                pub struct Simple<T>(T);
            };
            let toks = gen_base_unsized_infallible_mut(&Input::new(&input).unwrap()).unwrap();
            let expected = quote! {
                impl<T> opaque_typedef::OpaqueTypedefUnsizedInfallibleMut for Simple<T> {
                    fn from_inner_mut(__inner: &mut Self::Inner) -> &mut Self {
                        unsafe { &mut *(__inner as *mut Self::Inner as *mut Self) }
                    }
                }
            };
            assert_eq!(toks.to_string(), expected.to_string());
        }
    }

    #[test]
    fn simple_struct() {
        let input = syn::parse_quote! {
            #[derive(
                OpaqueTypedefUnsized,
                OpaqueTypedefUnsizedInfallible,
                OpaqueTypedefUnsizedMut,
                OpaqueTypedefUnsizedInfallibleMut
            )]
            #[repr(transparent)]
            pub struct Simple<T> {
                /// Inner data.
                inner: T,
            }
        };
        let toks = gen_base_unsized_infallible_mut(&Input::new(&input).unwrap()).unwrap();
        let expected = quote! {
            impl<T> opaque_typedef::OpaqueTypedefUnsizedInfallibleMut for Simple<T> {
                fn from_inner_mut(__inner: &mut Self::Inner) -> &mut Self {
                    unsafe { &mut *(__inner as *mut Self::Inner as *mut Self) }
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn simple_tuple_trait_bound() {
        let input = syn::parse_quote! {
            #[derive(
                OpaqueTypedefUnsized,
                OpaqueTypedefUnsizedInfallible,
                OpaqueTypedefUnsizedMut,
                OpaqueTypedefUnsizedInfallibleMut
            )]
            #[repr(transparent)]
            pub struct Simple<T: Debug>(T);
        };
        let toks = gen_base_unsized_infallible_mut(&Input::new(&input).unwrap()).unwrap();
        let expected = quote! {
            impl<T: Debug> opaque_typedef::OpaqueTypedefUnsizedInfallibleMut for Simple<T> {
                fn from_inner_mut(__inner: &mut Self::Inner) -> &mut Self {
                    unsafe { &mut *(__inner as *mut Self::Inner as *mut Self) }
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn multi_field_struct() {
        let input = syn::parse_quote! {
            #[derive(
                OpaqueTypedefUnsized,
                OpaqueTypedefUnsizedInfallible,
                OpaqueTypedefUnsizedMut,
                OpaqueTypedefUnsizedInfallibleMut
            )]
            #[repr(transparent)]
            pub struct Tagged<T, Tag> {
                /// Inner data.
                #[opaque_typedef::inner]
                inner: T,
                /// Tag.
                tag: Tag,
            }
        };
        let toks = gen_base_unsized_infallible_mut(&Input::new(&input).unwrap()).unwrap();
        let expected = quote! {
            impl<T, Tag> opaque_typedef::OpaqueTypedefUnsizedInfallibleMut for Tagged<T, Tag> {
                fn from_inner_mut(__inner: &mut Self::Inner) -> &mut Self {
                    unsafe { &mut *(__inner as *mut Self::Inner as *mut Self) }
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn simple_tuple_explicit_inner() {
        let input = syn::parse_quote! {
            #[derive(
                OpaqueTypedefUnsized,
                OpaqueTypedefUnsizedInfallible,
                OpaqueTypedefUnsizedMut,
                OpaqueTypedefUnsizedInfallibleMut
            )]
            #[repr(transparent)]
            pub struct Simple<T>(#[opaque_typedef(inner)] T);
        };
        let toks = gen_base_unsized_infallible_mut(&Input::new(&input).unwrap()).unwrap();
        let expected = quote! {
            impl<T> opaque_typedef::OpaqueTypedefUnsizedInfallibleMut for Simple<T> {
                fn from_inner_mut(__inner: &mut Self::Inner) -> &mut Self {
                    unsafe { &mut *(__inner as *mut Self::Inner as *mut Self) }
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn simple_tuple_hide_impl_docs() {
        let input = syn::parse_quote! {
            #[derive(
                OpaqueTypedefUnsized,
                OpaqueTypedefUnsizedInfallible,
                OpaqueTypedefUnsizedMut,
                OpaqueTypedefUnsizedInfallibleMut
            )]
            #[repr(transparent)]
            #[opaque_typedef(hide_base_impl_docs)]
            pub struct Simple<T>(pub T);
        };
        let toks = gen_base_unsized_infallible_mut(&Input::new(&input).unwrap()).unwrap();
        let expected = quote! {
            #[doc(hidden)]
            impl<T> opaque_typedef::OpaqueTypedefUnsizedInfallibleMut for Simple<T> {
                fn from_inner_mut(__inner: &mut Self::Inner) -> &mut Self {
                    unsafe { &mut *(__inner as *mut Self::Inner as *mut Self) }
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
            #[derive(
                OpaqueTypedefUnsized,
                OpaqueTypedefUnsizedInfallible,
                OpaqueTypedefUnsizedMut,
                OpaqueTypedefUnsizedInfallibleMut
            )]
            struct MyStr(str);
        };
        let _ = gen_base_unsized_infallible_mut(&Input::new(&input).unwrap()).unwrap();
    }

    #[test]
    fn with_validator() {
        let input = syn::parse_quote! {
            #[derive(
                OpaqueTypedefUnsized,
                OpaqueTypedefUnsizedInfallible,
                OpaqueTypedefUnsizedMut,
                OpaqueTypedefUnsizedInfallibleMut
            )]
            #[opaque_typedef(validate(error = "Error", validator = "validate"))]
            pub struct Simple<T>(T);
        };
        let toks = gen_base_unsized_infallible_mut(&Input::new(&input).unwrap());
        assert!(toks.is_err());
    }
}
