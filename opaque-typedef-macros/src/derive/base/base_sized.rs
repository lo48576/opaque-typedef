//! `OpaqueTypedefSized` codegen.

use proc_macro2::TokenStream;
use quote::quote;

use crate::input::Input;

/// Generate impl for `OpaqueTypedefSized`.
pub fn gen_base_sized(input: &Input) -> TokenStream {
    let ty = input.ident();
    let (generics_impl, generics_ty, generics_where) = input.generics().split_for_impl();
    let ty_inner = input.primary_field().ty();
    let ty_error = input.ty_error_force();
    let primary_field_accessor = input.primary_field().accessor();
    let inner_validated = input.validator().map_or_else(
        || quote!(__inner),
        |validator| quote!((#validator)(__inner)?),
    );
    let expr_try_from_inner = {
        let init_fields = input.fields_with_primary_flag().map(|(is_primary, field)| {
            let accessor = field.accessor();
            if is_primary {
                quote!(#accessor: #inner_validated)
            } else {
                quote!(#accessor: std::default::Default::default())
            }
        });
        quote!(Ok(Self {
            #(#init_fields,)*
        }))
    };
    let expr_from_inner_unchecked = {
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

    quote! {
        #base_impl_attrs
        impl #generics_impl opaque_typedef::OpaqueTypedefSized for #ty #generics_ty #generics_where {
            type Inner = #ty_inner;
            type Error = #ty_error;

            fn try_from_inner(__inner: Self::Inner) -> Result<Self, Self::Error> {
                #expr_try_from_inner
            }

            unsafe fn from_inner_unchecked(__inner: Self::Inner) -> Self {
                #expr_from_inner_unchecked
            }

            fn into_inner(self) -> Self::Inner {
                self.#primary_field_accessor
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
    fn simple_tuple() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefSized)]
            pub struct Simple<T>(pub T);
        };
        let toks = gen_base_sized(&Input::new(&input));
        let expected = quote! {
            impl<T> opaque_typedef::OpaqueTypedefSized for Simple<T> {
                type Inner = T;
                type Error = std::convert::Infallible;
                fn try_from_inner(__inner: Self::Inner) -> Result<Self, Self::Error> {
                    Ok(Self { 0: __inner, })
                }
                unsafe fn from_inner_unchecked(__inner: Self::Inner) -> Self {
                    Self { 0: __inner, }
                }
                fn into_inner(self) -> Self::Inner {
                    self.0
                }
                fn as_inner(&self) -> &Self::Inner {
                    &self.0
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn simple_struct() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefSized)]
            pub struct Simple<T> {
                /// Inner data.
                inner: T,
            }
        };
        let toks = gen_base_sized(&Input::new(&input));
        let expected = quote! {
            impl<T> opaque_typedef::OpaqueTypedefSized for Simple<T> {
                type Inner = T;
                type Error = std::convert::Infallible;
                fn try_from_inner(__inner: Self::Inner) -> Result<Self, Self::Error> {
                    Ok(Self { inner: __inner, })
                }
                unsafe fn from_inner_unchecked(__inner: Self::Inner) -> Self {
                    Self { inner: __inner, }
                }
                fn into_inner(self) -> Self::Inner {
                    self.inner
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
            #[derive(OpaqueTypedefSized)]
            pub struct Simple<T: Clone>(pub T);
        };
        let toks = gen_base_sized(&Input::new(&input));
        let expected = quote! {
            impl<T: Clone> opaque_typedef::OpaqueTypedefSized for Simple<T> {
                type Inner = T;
                type Error = std::convert::Infallible;
                fn try_from_inner(__inner: Self::Inner) -> Result<Self, Self::Error> {
                    Ok(Self { 0: __inner, })
                }
                unsafe fn from_inner_unchecked(__inner: Self::Inner) -> Self {
                    Self { 0: __inner, }
                }
                fn into_inner(self) -> Self::Inner {
                    self.0
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
            #[derive(OpaqueTypedefSized)]
            pub struct Tagged<T, Tag> {
                /// Inner data.
                #[opaque_typedef::inner]
                inner: T,
                /// Tag.
                tag: Tag,
            }
        };
        let toks = gen_base_sized(&Input::new(&input));
        let expected = quote! {
            impl<T, Tag> opaque_typedef::OpaqueTypedefSized for Tagged<T, Tag> {
                type Inner = T;
                type Error = std::convert::Infallible;
                fn try_from_inner(__inner: Self::Inner) -> Result<Self, Self::Error> {
                    Ok(Self {
                        inner: __inner,
                        tag: std::default::Default::default(),
                    })
                }
                unsafe fn from_inner_unchecked(__inner: Self::Inner) -> Self {
                    Self {
                        inner: __inner,
                        tag: std::default::Default::default(),
                    }
                }
                fn into_inner(self) -> Self::Inner {
                    self.inner
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
            #[derive(OpaqueTypedefSized)]
            pub struct Simple<T>(#[opaque_typedef(inner)] pub T);
        };
        let toks = gen_base_sized(&Input::new(&input));
        let expected = quote! {
            impl<T> opaque_typedef::OpaqueTypedefSized for Simple<T> {
                type Inner = T;
                type Error = std::convert::Infallible;
                fn try_from_inner(__inner: Self::Inner) -> Result<Self, Self::Error> {
                    Ok(Self { 0: __inner, })
                }
                unsafe fn from_inner_unchecked(__inner: Self::Inner) -> Self {
                    Self { 0: __inner, }
                }
                fn into_inner(self) -> Self::Inner {
                    self.0
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
            #[derive(OpaqueTypedefSized)]
            #[opaque_typedef(hide_base_impl_docs)]
            pub struct Simple<T>(pub T);
        };
        let toks = gen_base_sized(&Input::new(&input));
        let expected = quote! {
            #[doc(hidden)]
            impl<T> opaque_typedef::OpaqueTypedefSized for Simple<T> {
                type Inner = T;
                type Error = std::convert::Infallible;
                fn try_from_inner(__inner: Self::Inner) -> Result<Self, Self::Error> {
                    Ok(Self { 0: __inner, })
                }
                unsafe fn from_inner_unchecked(__inner: Self::Inner) -> Self {
                    Self { 0: __inner, }
                }
                fn into_inner(self) -> Self::Inner {
                    self.0
                }
                fn as_inner(&self) -> &Self::Inner {
                    &self.0
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }

    #[test]
    fn my_string_validation() {
        let input = syn::parse_quote! {
            #[derive(OpaqueTypedefSized)]
            #[opaque_typedef(validate(
                validator = "|s: Vec<u8>| match std::str::from_utf8(&s) {
                    Ok(_) => Ok(s),
                    Err(e) => Err(e),
                }",
                error = "std::string::Utf8Error",
            ))]
            pub struct MyString(Vec<u8>);
        };
        let toks = gen_base_sized(&Input::new(&input));
        let expected = quote! {
            impl opaque_typedef::OpaqueTypedefSized for MyString {
                type Inner = Vec<u8>;
                type Error = std::string::Utf8Error;
                fn try_from_inner(__inner: Self::Inner) -> Result<Self, Self::Error> {
                    Ok(Self {
                        0: (|s: Vec<u8>| match std::str::from_utf8(&s) {
                            Ok(_) => Ok(s),
                            Err(e) => Err(e),
                        })(__inner)?,
                    })
                }
                unsafe fn from_inner_unchecked(__inner: Self::Inner) -> Self {
                    Self { 0: __inner, }
                }
                fn into_inner(self) -> Self::Inner {
                    self.0
                }
                fn as_inner(&self) -> &Self::Inner {
                    &self.0
                }
            }
        };
        assert_eq!(toks.to_string(), expected.to_string());
    }
}
