//! `OpaqueTypedefSizedMut` codegen.

use proc_macro2::TokenStream;
use quote::quote;

use crate::input::Input;

/// Generate impl for `OpaqueTypedefSizedMut`.
pub fn gen_base_sized_mut(input: &Input) -> TokenStream {
    let ty = input.ident();
    let (generics_impl, generics_ty, generics_where) = input.generics().split_for_impl();
    let primary_field_accessor = input.primary_field().accessor();
    let base_impl_attrs = input.base_impl_attrs();
    quote! {
        #base_impl_attrs
        impl #generics_impl opaque_typedef::OpaqueTypedefSizedMut for #ty #generics_ty #generics_where {
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
    fn simple_tuple() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefSized, OpaqueTypedefSizedMut)]
            pub struct Simple<T>(pub T);
        };
        let toks = gen_base_sized_mut(&Input::new(&input));
        let expected = quote! {
            impl<T> opaque_typedef::OpaqueTypedefSizedMut for Simple<T> {
                fn as_inner_mut(&mut self) -> &mut Self::Inner {
                    &mut self.0
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn simple_struct() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefSized, OpaqueTypedefSizedMut)]
            pub struct Simple<T> {
                /// Inner data.
                inner: T,
            }
        };
        let toks = gen_base_sized_mut(&Input::new(&input));
        let expected = quote! {
            impl<T> opaque_typedef::OpaqueTypedefSizedMut for Simple<T> {
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
            #[derive(OpaqueTypedefSized, OpaqueTypedefSizedMut)]
            pub struct Simple<T: Clone>(pub T);
        };
        let toks = gen_base_sized_mut(&Input::new(&input));
        let expected = quote! {
            impl<T: Clone> opaque_typedef::OpaqueTypedefSizedMut for Simple<T> {
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
            #[derive(OpaqueTypedefSized, OpaqueTypedefSizedMut)]
            pub struct Tagged<T, Tag> {
                /// Inner data.
                #[opaque_typedef::inner]
                inner: T,
                /// Tag.
                tag: Tag,
            }
        };
        let toks = gen_base_sized_mut(&Input::new(&input));
        let expected = quote! {
            impl<T, Tag> opaque_typedef::OpaqueTypedefSizedMut for Tagged<T, Tag> {
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
            #[derive(OpaqueTypedefSized, OpaqueTypedefSizedMut)]
            pub struct Simple<T>(#[opaque_typedef(inner)] pub T);
        };
        let toks = gen_base_sized_mut(&Input::new(&input));
        let expected = quote! {
            impl<T> opaque_typedef::OpaqueTypedefSizedMut for Simple<T> {
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
            #[derive(OpaqueTypedefSized, OpaqueTypedefSizedMut)]
            #[opaque_typedef(hide_base_impl_docs)]
            pub struct Simple<T>(pub T);
        };
        let toks = gen_base_sized_mut(&Input::new(&input));
        let expected = quote! {
            #[doc(hidden)]
            impl<T> opaque_typedef::OpaqueTypedefSizedMut for Simple<T> {
                fn as_inner_mut(&mut self) -> &mut Self::Inner {
                    &mut self.0
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }
}
