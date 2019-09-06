//! `OpaqueTypedefSizedInfallible` codegen.

use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

use crate::input::Input;

/// Generate impl for `OpaqueTypedefSizedInfallible`.
pub fn gen_base_sized_infallible(input: &Input) -> syn::Result<TokenStream> {
    if let Some(validator) = input.validator() {
        // A validator is specified and it may fail.
        return Err(syn::Error::new(
            validator.span(),
            "Validator and `OpaqueTypedefSizedInfallible` cannot be specified at the same time",
        ));
    }

    let ty = input.ident();
    let (generics_impl, generics_ty, generics_where) = input.generics().split_for_impl();
    let expr_from_inner = {
        let init_fields = input.fields_with_primary_flag().map(|(is_primary, field)| {
            let accessor = field.accessor();
            if is_primary {
                quote!(#accessor: __inner)
            } else {
                quote!(#accessor: std::default::Default::default())
            }
        });
        quote!(Self {
            #(#init_fields,)*
        })
    };
    let base_impl_attrs = input.base_impl_attrs();

    Ok(quote! {
        #base_impl_attrs
        impl #generics_impl opaque_typedef::OpaqueTypedefSizedInfallible for #ty #generics_ty #generics_where {
            fn from_inner(__inner: Self::Inner) -> Self {
                #expr_from_inner
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_tuple() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefSized, OpaqueTypedefSizedInfallible)]
            pub struct Simple<T>(pub T);
        };
        let toks = gen_base_sized_infallible(&Input::new(&input).unwrap()).unwrap();
        let expected = quote! {
            impl<T> opaque_typedef::OpaqueTypedefSizedInfallible for Simple<T> {
                fn from_inner(__inner: Self::Inner) -> Self {
                    Self { 0: __inner, }
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn simple_struct() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefSized, OpaqueTypedefSizedInfallible)]
            pub struct Simple<T> {
                /// Inner data.
                inner: T,
            }
        };
        let toks = gen_base_sized_infallible(&Input::new(&input).unwrap()).unwrap();
        let expected = quote! {
            impl<T> opaque_typedef::OpaqueTypedefSizedInfallible for Simple<T> {
                fn from_inner(__inner: Self::Inner) -> Self {
                    Self { inner: __inner, }
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn simple_tuple_trait_bound() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefSized, OpaqueTypedefSizedInfallible)]
            pub struct Simple<T: Clone>(pub T);
        };
        let toks = gen_base_sized_infallible(&Input::new(&input).unwrap()).unwrap();
        let expected = quote! {
            impl<T: Clone> opaque_typedef::OpaqueTypedefSizedInfallible for Simple<T> {
                fn from_inner(__inner: Self::Inner) -> Self {
                    Self { 0: __inner, }
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn multi_field_struct() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefSized, OpaqueTypedefSizedInfallible)]
            pub struct Tagged<T, Tag> {
                /// Inner data.
                #[opaque_typedef::inner]
                inner: T,
                /// Tag.
                tag: Tag,
            }
        };
        let toks = gen_base_sized_infallible(&Input::new(&input).unwrap()).unwrap();
        let expected = quote! {
            impl<T, Tag> opaque_typedef::OpaqueTypedefSizedInfallible for Tagged<T, Tag> {
                fn from_inner(__inner: Self::Inner) -> Self {
                    Self {
                        inner: __inner,
                        tag: std::default::Default::default(),
                    }
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn simple_tuple_explicit_inner() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefSized, OpaqueTypedefSizedInfallible)]
            pub struct Simple<T>(#[opaque_typedef(inner)] pub T);
        };
        let toks = gen_base_sized_infallible(&Input::new(&input).unwrap()).unwrap();
        let expected = quote! {
            impl<T> opaque_typedef::OpaqueTypedefSizedInfallible for Simple<T> {
                fn from_inner(__inner: Self::Inner) -> Self {
                    Self { 0: __inner, }
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn simple_tuple_hide_impl_docs() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefSized, OpaqueTypedefSizedInfallible)]
            #[opaque_typedef(hide_base_impl_docs)]
            pub struct Simple<T>(pub T);
        };
        let toks = gen_base_sized_infallible(&Input::new(&input).unwrap()).unwrap();
        let expected = quote! {
            #[doc(hidden)]
            impl<T> opaque_typedef::OpaqueTypedefSizedInfallible for Simple<T> {
                fn from_inner(__inner: Self::Inner) -> Self {
                    Self { 0: __inner, }
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn with_validator() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefSized, OpaqueTypedefSizedInfallible)]
            #[opaque_typedef(validate(error = "Error", validator = "validate"))]
            pub struct Simple<T>(pub T);
        };
        let toks = gen_base_sized_infallible(&Input::new(&input).unwrap());
        assert!(toks.is_err());
    }
}
