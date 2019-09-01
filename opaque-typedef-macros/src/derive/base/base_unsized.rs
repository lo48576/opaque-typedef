//! `OpaqueTypedefUnsized` codegen.

use proc_macro2::TokenStream;
use quote::quote;

use crate::input::Input;

/// Generate impl for `OpaqueTypedefUnsized`.
pub fn gen_base_unsized(input: &Input) -> TokenStream {
    input.ensure_acceptable_unsized_repr_or_panic();
    let ty = input.ident();
    let (generics_impl, generics_ty, generics_where) = input.generics().split_for_impl();
    let ty_inner = input.primary_field().ty();
    let ty_error = input.ty_error_force();
    let primary_field_accessor = input.primary_field().accessor();
    let inner_validated = input.validator().map_or_else(
        || quote!(__inner),
        |validator| quote!((#validator)(__inner)?),
    );
    let expr_try_from_inner =
        quote!(Ok(unsafe { &*(#inner_validated as *const Self::Inner as *const Self) }));
    let expr_from_inner_unchecked = quote!(&*(__inner as *const Self::Inner as *const Self));
    let base_impl_attrs = input.base_impl_attrs();

    quote! {
        #base_impl_attrs
        impl #generics_impl opaque_typedef::OpaqueTypedefUnsized for #ty #generics_ty #generics_where {
            type Inner = #ty_inner;
            type Error = #ty_error;

            fn try_from_inner(__inner: &Self::Inner) -> Result<&Self, Self::Error> {
                #expr_try_from_inner
            }

            unsafe fn from_inner_unchecked(__inner: &Self::Inner) -> &Self {
                #expr_from_inner_unchecked
            }

            fn as_inner(&self) -> &Self::Inner {
                &self.#primary_field_accessor
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_tuple_for_each_repr() {
        for repr in &[quote!(C), quote!(transparent)] {
            let input = syn::parse_quote! {
                #[derive(OpaqueTypedefUnsized)]
                #[repr(#repr)]
                pub struct Simple<T>(T);
            };
            let toks = gen_base_unsized(&Input::new(&input).unwrap());
            let expected = quote! {
                impl<T> opaque_typedef::OpaqueTypedefUnsized for Simple<T> {
                    type Inner = T;
                    type Error = std::convert::Infallible;
                    fn try_from_inner(__inner: &Self::Inner) -> Result<&Self, Self::Error> {
                        Ok(unsafe { &*(__inner as *const Self::Inner as *const Self) })
                    }
                    unsafe fn from_inner_unchecked(__inner: &Self::Inner) -> &Self {
                        &*(__inner as *const Self::Inner as *const Self)
                    }
                    fn as_inner(&self) -> &Self::Inner {
                        &self.0
                    }
                }
            };
            assert_eq!(toks.to_string(), expected.to_string());
        }
    }

    #[test]
    fn simple_struct() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefUnsized)]
            #[repr(transparent)]
            pub struct Simple<T> {
                /// Inner data.
                inner: T,
            }
        };
        let toks = gen_base_unsized(&Input::new(&input).unwrap());
        let expected = quote! {
            impl<T> opaque_typedef::OpaqueTypedefUnsized for Simple<T> {
                type Inner = T;
                type Error = std::convert::Infallible;
                fn try_from_inner(__inner: &Self::Inner) -> Result<&Self, Self::Error> {
                    Ok(unsafe { &*(__inner as *const Self::Inner as *const Self) })
                }
                unsafe fn from_inner_unchecked(__inner: &Self::Inner) -> &Self {
                    &*(__inner as *const Self::Inner as *const Self)
                }
                fn as_inner(&self) -> &Self::Inner {
                    &self.inner
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn simple_tuple_trait_bound() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefUnsized)]
            #[repr(transparent)]
            pub struct Simple<T: Debug>(T);
        };
        let toks = gen_base_unsized(&Input::new(&input).unwrap());
        let expected = quote! {
            impl<T: Debug> opaque_typedef::OpaqueTypedefUnsized for Simple<T> {
                type Inner = T;
                type Error = std::convert::Infallible;
                fn try_from_inner(__inner: &Self::Inner) -> Result<&Self, Self::Error> {
                    Ok(unsafe { &*(__inner as *const Self::Inner as *const Self) })
                }
                unsafe fn from_inner_unchecked(__inner: &Self::Inner) -> &Self {
                    &*(__inner as *const Self::Inner as *const Self)
                }
                fn as_inner(&self) -> &Self::Inner {
                    &self.0
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn multi_field_struct() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefUnsized)]
            #[repr(transparent)]
            pub struct Tagged<T, Tag> {
                /// Inner data.
                #[opaque_typedef::inner]
                inner: T,
                /// Tag.
                tag: Tag,
            }
        };
        let toks = gen_base_unsized(&Input::new(&input).unwrap());
        let expected = quote! {
            impl<T, Tag> opaque_typedef::OpaqueTypedefUnsized for Tagged<T, Tag> {
                type Inner = T;
                type Error = std::convert::Infallible;
                fn try_from_inner(__inner: &Self::Inner) -> Result<&Self, Self::Error> {
                    Ok(unsafe { &*(__inner as *const Self::Inner as *const Self) })
                }
                unsafe fn from_inner_unchecked(__inner: &Self::Inner) -> &Self {
                    &*(__inner as *const Self::Inner as *const Self)
                }
                fn as_inner(&self) -> &Self::Inner {
                    &self.inner
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn simple_tuple_explicit_inner() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefUnsized)]
            #[repr(transparent)]
            pub struct Simple<T>(#[opaque_typedef(inner)] T);
        };
        let toks = gen_base_unsized(&Input::new(&input).unwrap());
        let expected = quote! {
            impl<T> opaque_typedef::OpaqueTypedefUnsized for Simple<T> {
                type Inner = T;
                type Error = std::convert::Infallible;
                fn try_from_inner(__inner: &Self::Inner) -> Result<&Self, Self::Error> {
                    Ok(unsafe { &*(__inner as *const Self::Inner as *const Self) })
                }
                unsafe fn from_inner_unchecked(__inner: &Self::Inner) -> &Self {
                    &*(__inner as *const Self::Inner as *const Self)
                }
                fn as_inner(&self) -> &Self::Inner {
                    &self.0
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn simple_tuple_hide_impl_docs() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefUnsized)]
            #[repr(transparent)]
            #[opaque_typedef(hide_base_impl_docs)]
            pub struct Simple<T>(pub T);
        };
        let toks = gen_base_unsized(&Input::new(&input).unwrap());
        let expected = quote! {
            #[doc(hidden)]
            impl<T> opaque_typedef::OpaqueTypedefUnsized for Simple<T> {
                type Inner = T;
                type Error = std::convert::Infallible;
                fn try_from_inner(__inner: &Self::Inner) -> Result<&Self, Self::Error> {
                    Ok(unsafe { &*(__inner as *const Self::Inner as *const Self) })
                }
                unsafe fn from_inner_unchecked(__inner: &Self::Inner) -> &Self {
                    &*(__inner as *const Self::Inner as *const Self)
                }
                fn as_inner(&self) -> &Self::Inner {
                    &self.0
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn my_str_validation() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefUnsized)]
            #[repr(transparent)]
            #[opaque_typedef(validate(
                validator = "|s| std::str::from_utf8(s).map(|_| s)",
                error = "std::string::Utf8Error",
            ))]
            pub struct MyStr(&[u8]);
        };
        let toks = gen_base_unsized(&Input::new(&input).unwrap());
        let expected = quote! {
            impl opaque_typedef::OpaqueTypedefUnsized for MyStr {
                type Inner = &[u8];
                type Error = std::string::Utf8Error;
                fn try_from_inner(__inner: &Self::Inner) -> Result<&Self, Self::Error> {
                    Ok(unsafe {
                        &*((|s| std::str::from_utf8(s).map(|_| s))(__inner)? as *const Self::Inner as *const Self)
                    })
                }
                unsafe fn from_inner_unchecked(__inner: &Self::Inner) -> &Self {
                    &*(__inner as *const Self::Inner as *const Self)
                }
                fn as_inner(&self) -> &Self::Inner {
                    &self.0
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
            #[derive(OpaqueTypedefUnsized)]
            struct MyStr(str);
        };
        let _ = gen_base_unsized(&Input::new(&input).unwrap());
    }
}
