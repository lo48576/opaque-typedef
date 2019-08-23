//! Input data.

use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Data, DeriveInput, Expr, Field, Fields, Ident, Meta, Type};

use crate::attr::MetaExt;

/// Input data.
///
/// *Primary field* is the field which has data convertible from and into the inner type.
pub struct Input<'a> {
    /// Derive input.
    derive_input: &'a DeriveInput,
    /// Primary field.
    primary_field: FieldWrapper<'a>,
    /// Whether to hide the base traits impls docs.
    should_hide_base_impl_docs: bool,
    /// Attributes parsed as meta.
    meta_attrs: Vec<Meta>,
    /// Validator.
    validator: Option<Expr>,
    /// Validation error type.
    ty_validation_error: Option<Type>,
}

impl<'a> Input<'a> {
    /// Creates an `Input` form the given `DeriveInput`.
    pub fn new(derive_input: &'a DeriveInput) -> Self {
        let primary_field = get_primary_field(fields(&derive_input.data));
        let meta_attrs = derive_input
            .attrs
            .iter()
            .flat_map(|attr| attr.parse_meta())
            .collect::<Vec<_>>();
        let should_hide_base_impl_docs = meta_attrs
            .iter()
            .any(|meta| meta.has_level2_word("opaque_typedef", "hide_base_impl_docs"));
        let validator = meta_attrs.iter().find_map(|attr| attr.validator());
        let ty_validation_error = meta_attrs
            .iter()
            .find_map(|attr| attr.ty_validation_error());

        Self {
            derive_input,
            primary_field,
            should_hide_base_impl_docs,
            meta_attrs,
            validator,
            ty_validation_error,
        }
    }

    /// Returns the identifier of the type.
    pub fn ident(&self) -> &'a Ident {
        &self.derive_input.ident
    }

    /// Returns raw fields.
    fn raw_fields(&self) -> &'a Fields {
        match &self.derive_input.data {
            Data::Struct(v) => &v.fields,
            _ => panic!("Only struct type is supported"),
        }
    }

    /// Returns an iterator of fields.
    pub fn fields<'b>(&'b self) -> impl Iterator<Item = FieldWrapper<'a>> + 'b {
        self.raw_fields()
            .iter()
            .enumerate()
            .map(|(i, field)| FieldWrapper::new(i, field))
    }

    /// Returns an iterator of fields.
    pub fn fields_with_primary_flag<'b>(
        &'b self,
    ) -> impl Iterator<Item = (bool, FieldWrapper<'a>)> + 'b {
        let primary_field_index = self.primary_field.index();
        self.fields()
            .map(move |field| (field.index() == primary_field_index, field))
    }

    /// Returns the primary field.
    pub fn primary_field(&self) -> &FieldWrapper<'a> {
        &self.primary_field
    }

    /// Returns the generics.
    pub fn generics(&self) -> &syn::Generics {
        &self.derive_input.generics
    }

    /// Returns the validator if available.
    pub fn validator(&self) -> Option<&Expr> {
        self.validator.as_ref()
    }

    /// Returns the error type if available.
    pub fn ty_error(&self) -> Option<&Type> {
        self.ty_validation_error.as_ref()
    }

    /// Returns the error type.
    pub fn ty_error_force(&self) -> TokenStream {
        match self.ty_error() {
            Some(v) => v.into_token_stream(),
            None => quote!(std::convert::Infallible),
        }
    }

    /// Returns whether the base traits impls docs should be hidden.
    pub fn should_hide_base_impl_docs(&self) -> bool {
        self.should_hide_base_impl_docs
    }

    /// Returns token stream of attributes for base trait impls.
    pub fn base_impl_attrs(&self) -> TokenStream {
        if self.should_hide_base_impl_docs() {
            quote!(#[doc(hidden)])
        } else {
            quote!()
        }
    }

    /// Returns an iterator of attributes parsed as meta.
    pub fn meta_attrs(&self) -> impl Iterator<Item = &Meta> {
        self.meta_attrs.iter()
    }

    /// Ensures the type has acceptable `repr` meta for unsized type alias, or panic.
    ///
    /// # Panics
    ///
    /// Panics if an acceptable `repr` is not specified for the type.
    pub fn ensure_acceptable_unsized_repr_or_panic(&self) {
        if !self.meta_attrs().any(MetaExt::has_unsized_repr) {
            panic!("`#[repr(C)]` or `#[repr(transparent)]` is required for unsized type alias");
        }
    }
}

/// Struct and tuple field accessor.
pub struct FieldWrapper<'a> {
    /// Field index.
    index: usize,
    /// Field.
    field: &'a Field,
}

impl<'a> FieldWrapper<'a> {
    /// Creates a new accessor for the field at the given index.
    fn new(index: usize, field: &'a Field) -> Self {
        Self { index, field }
    }

    /// Creates a field.
    pub fn accessor(&self) -> FieldAccessor<'a> {
        match &self.field.ident {
            Some(ident) => FieldAccessor::Named(ident),
            None => FieldAccessor::Unnamed(self.index),
        }
    }

    /// Returns the field index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the field type.
    pub fn ty(&self) -> &'a Type {
        &self.field.ty
    }
}

/// Struct and tuple field accessor.
#[derive(Clone, Copy)]
pub enum FieldAccessor<'a> {
    /// Named field.
    Named(&'a Ident),
    /// Unnamed field.
    Unnamed(usize),
}

impl quote::ToTokens for FieldAccessor<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match *self {
            FieldAccessor::Named(ident) => {
                tokens.append(ident.clone());
            }
            FieldAccessor::Unnamed(index) => {
                tokens.append(proc_macro2::Literal::usize_unsuffixed(index));
            }
        }
    }
}

/// Returns the primary field.
///
/// # Panics
/// Panics if no primary fields found or multiple fields are marked as primary.
fn get_primary_field<'a>(
    mut fields: impl Iterator<Item = &'a Field> + std::iter::ExactSizeIterator,
) -> FieldWrapper<'a> {
    let fields_len = fields.len();
    if fields_len == 0 {
        panic!("No fields found");
    } else if fields_len == 1 {
        return FieldWrapper::new(0, fields.next().unwrap());
    }
    let mut primary_fields = fields
        .enumerate()
        .filter(|&(_, field)| is_primary_field(field))
        .map(|(i, field)| FieldWrapper::new(i, field));
    let first = match primary_fields.next() {
        Some(v) => v,
        None => panic!("There are multiple fields but none are marked as primary"),
    };
    if let Some(second) = primary_fields.next() {
        panic!(
            "Multiple fields are marked as primary: {}th and {}th",
            first.index(),
            second.index()
        );
    }
    first
}

/// Returns an iterator of the fields.
fn fields(data: &Data) -> impl Iterator<Item = &Field> + std::iter::ExactSizeIterator {
    let data = match &data {
        Data::Struct(v) => v,
        _ => panic!("Only struct type is supported"),
    };
    data.fields.iter()
}

/// Checks if the given field is marked as primary.
fn is_primary_field(field: &Field) -> bool {
    field
        .attrs
        .iter()
        .flat_map(|attr| attr.parse_meta())
        .any(|meta| meta.has_level2_word("opaque_typedef", "inner"))
}
