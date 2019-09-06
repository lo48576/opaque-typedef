//! `OpaqueTypedefUnsizedMut` codegen.

use proc_macro2::TokenStream;
use quote::quote;

use crate::input::Input;

/// Generate impl for `OpaqueTypedefUnsizedMut`.
pub fn gen_base_unsized_mut(input: &Input) -> TokenStream {
    input.ensure_acceptable_unsized_repr_or_panic();
    let ty = input.ident();
    let (generics_impl, generics_ty, generics_where) = input.generics().split_for_impl();
    let primary_field_accessor = input.primary_field().accessor();
    let expr_from_inner_unchecked = quote!(&mut *(__inner as *mut Self::Inner as *mut Self));
    // Safety condition of this `unsafe` is same as that of `base_unsized()`.
    // Note that using the resulting expression is NOT always safe.
    // i.e. unrestricted modification to the inner field may make the value internally inconsistent
    // and result in undefined behaivor.
    let expr_from_inner = quote!(unsafe { #expr_from_inner_unchecked });
    let base_impl_attrs = input.base_impl_attrs();

    quote! {
        #base_impl_attrs
        impl #generics_impl opaque_typedef::OpaqueTypedefUnsizedMut for #ty #generics_ty #generics_where {
            fn try_from_inner_mut(__inner: &mut Self::Inner) -> Result<&mut Self, Self::Error> {
                Ok(#expr_from_inner)
            }

            unsafe fn from_inner_unchecked_mut(__inner: &mut Self::Inner) -> &mut Self {
                #expr_from_inner_unchecked
            }

            fn as_inner_mut(&mut self) -> &mut Self::Inner {
                &mut self.#primary_field_accessor
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
                #[derive(OpaqueTypedefUnsized, OpaqueTypedefUnsizedMut)]
                #[repr(#repr)]
                pub struct Simple<T>(T);
            };
            let toks = gen_base_unsized_mut(&Input::new(&input).unwrap());
            let expected = quote! {
                impl<T> opaque_typedef::OpaqueTypedefUnsizedMut for Simple<T> {
                    fn try_from_inner_mut(__inner: &mut Self::Inner) -> Result<&mut Self, Self::Error> {
                        Ok(unsafe { &mut *(__inner as *mut Self::Inner as *mut Self) })
                    }
                    unsafe fn from_inner_unchecked_mut(__inner: &mut Self::Inner) -> &mut Self {
                        &mut *(__inner as *mut Self::Inner as *mut Self)
                    }
                    fn as_inner_mut(&mut self) -> &mut Self::Inner {
                        &mut self.0
                    }
                }
            };
            assert_eq!(toks.to_string(), expected.to_string());
        }
    }

    #[test]
    fn simple_struct() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefUnsized, OpaqueTypedefUnsizedMut)]
            #[repr(transparent)]
            pub struct Simple<T> {
                /// Inner data.
                inner: T,
            }
        };
        let toks = gen_base_unsized_mut(&Input::new(&input).unwrap());
        let expected = quote! {
            impl<T> opaque_typedef::OpaqueTypedefUnsizedMut for Simple<T> {
                fn try_from_inner_mut(__inner: &mut Self::Inner) -> Result<&mut Self, Self::Error> {
                    Ok(unsafe { &mut *(__inner as *mut Self::Inner as *mut Self) })
                }
                unsafe fn from_inner_unchecked_mut(__inner: &mut Self::Inner) -> &mut Self {
                    &mut *(__inner as *mut Self::Inner as *mut Self)
                }
                fn as_inner_mut(&mut self) -> &mut Self::Inner {
                    &mut self.inner
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn simple_tuple_trait_bound() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefUnsized, OpaqueTypedefUnsizedMut)]
            #[repr(transparent)]
            pub struct Simple<T: Debug>(T);
        };
        let toks = gen_base_unsized_mut(&Input::new(&input).unwrap());
        let expected = quote! {
            impl<T: Debug> opaque_typedef::OpaqueTypedefUnsizedMut for Simple<T> {
                fn try_from_inner_mut(__inner: &mut Self::Inner) -> Result<&mut Self, Self::Error> {
                    Ok(unsafe { &mut *(__inner as *mut Self::Inner as *mut Self) })
                }
                unsafe fn from_inner_unchecked_mut(__inner: &mut Self::Inner) -> &mut Self {
                    &mut *(__inner as *mut Self::Inner as *mut Self)
                }
                fn as_inner_mut(&mut self) -> &mut Self::Inner {
                    &mut self.0
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn multi_field_struct() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefUnsized, OpaqueTypedefUnsizedMut)]
            #[repr(transparent)]
            pub struct Tagged<T, Tag> {
                /// Inner data.
                #[opaque_typedef::inner]
                inner: T,
                /// Tag.
                tag: Tag,
            }
        };
        let toks = gen_base_unsized_mut(&Input::new(&input).unwrap());
        let expected = quote! {
            impl<T, Tag> opaque_typedef::OpaqueTypedefUnsizedMut for Tagged<T, Tag> {
                fn try_from_inner_mut(__inner: &mut Self::Inner) -> Result<&mut Self, Self::Error> {
                    Ok(unsafe { &mut *(__inner as *mut Self::Inner as *mut Self) })
                }
                unsafe fn from_inner_unchecked_mut(__inner: &mut Self::Inner) -> &mut Self {
                    &mut *(__inner as *mut Self::Inner as *mut Self)
                }
                fn as_inner_mut(&mut self) -> &mut Self::Inner {
                    &mut self.inner
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn simple_tuple_explicit_inner() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefUnsized, OpaqueTypedefUnsizedMut)]
            #[repr(transparent)]
            pub struct Simple<T>(#[opaque_typedef(inner)] T);
        };
        let toks = gen_base_unsized_mut(&Input::new(&input).unwrap());
        let expected = quote! {
            impl<T> opaque_typedef::OpaqueTypedefUnsizedMut for Simple<T> {
                fn try_from_inner_mut(__inner: &mut Self::Inner) -> Result<&mut Self, Self::Error> {
                    Ok(unsafe { &mut *(__inner as *mut Self::Inner as *mut Self) })
                }
                unsafe fn from_inner_unchecked_mut(__inner: &mut Self::Inner) -> &mut Self {
                    &mut *(__inner as *mut Self::Inner as *mut Self)
                }
                fn as_inner_mut(&mut self) -> &mut Self::Inner {
                    &mut self.0
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn simple_tuple_hide_impl_docs() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefUnsized, OpaqueTypedefUnsizedMut)]
            #[repr(transparent)]
            #[opaque_typedef(hide_base_impl_docs)]
            pub struct Simple<T>(pub T);
        };
        let toks = gen_base_unsized_mut(&Input::new(&input).unwrap());
        let expected = quote! {
            #[doc(hidden)]
            impl<T> opaque_typedef::OpaqueTypedefUnsizedMut for Simple<T> {
                fn try_from_inner_mut(__inner: &mut Self::Inner) -> Result<&mut Self, Self::Error> {
                    Ok(unsafe { &mut *(__inner as *mut Self::Inner as *mut Self) })
                }
                unsafe fn from_inner_unchecked_mut(__inner: &mut Self::Inner) -> &mut Self {
                    &mut *(__inner as *mut Self::Inner as *mut Self)
                }
                fn as_inner_mut(&mut self) -> &mut Self::Inner {
                    &mut self.0
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
            #[derive(OpaqueTypedefUnsized, OpaqueTypedefUnsizedMut)]
            struct MyStr(str);
        };
        let _ = gen_base_unsized_mut(&Input::new(&input).unwrap());
    }
}
