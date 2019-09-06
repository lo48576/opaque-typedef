//! Attributes-related helpers.

use syn::{spanned::Spanned, Expr, Lit, Meta, NestedMeta, Path, Type};

/// Extension for `syn::Meta` type.
pub trait MetaExt {
    /// Returns `true` if the meta has the repr `C` or `transparent`.
    fn has_unsized_repr(&self) -> bool;
    /// Returns `true` if the meta has the word at depth 2, such as `foo(bar)` or path `foo::bar`.
    fn has_level2_word(&self, level1: &str, level2: &str) -> bool;
    /// Returns validator if available.
    fn validator(&self) -> Result<Option<Expr>, syn::Error>;
    /// Returns validation error type if available.
    fn ty_validation_error(&self) -> Result<Option<Type>, syn::Error>;
}

impl MetaExt for Meta {
    fn has_unsized_repr(&self) -> bool {
        match self {
            Meta::List(metalist) if metalist.path.is_ident("repr") => {
                metalist.nested.iter().any(|nested| match nested {
                    NestedMeta::Meta(Meta::Path(path)) => {
                        path.is_ident("C") || path.is_ident("transparent")
                    }
                    _ => false,
                })
            }
            _ => false,
        }
    }

    fn has_level2_word(&self, level1: &str, level2: &str) -> bool {
        // NOTE:
        // Though this impl supports both `#[foo::bar]` and `#[foo(bar)]`, type level attribute in
        // `#[foo::bar]` syntax is not intended (and not supported) by Rust 1.37.
        // See <https://github.com/rust-lang/rust/issues/55168>.
        match self {
            Meta::Path(path) => eq_path_components(path, &[level1, level2]),
            Meta::List(metalist) if metalist.path.is_ident(level1) => {
                metalist.nested.iter().any(|nested| match nested {
                    NestedMeta::Meta(Meta::Path(path)) => path.is_ident(level2),
                    _ => false,
                })
            }
            _ => false,
        }
    }

    fn validator(&self) -> Result<Option<Expr>, syn::Error> {
        find_validation_metas(self)
            .find_map(|meta| match meta {
                Meta::NameValue(namevalue) if namevalue.path.is_ident("validator") => {
                    Some(&namevalue.lit)
                }
                _ => None,
            })
            .map(|lit| match lit {
                Lit::Str(s) => s.parse().map_err(|e| {
                    syn::Error::new(
                        lit.span(),
                        format!("Failed to parse validator function: {}", e),
                    )
                }),
                _ => Err(syn::Error::new(
                    lit.span(),
                    "Expected string literal as validator, but got other literal",
                )),
            })
            .transpose()
    }

    fn ty_validation_error(&self) -> Result<Option<Type>, syn::Error> {
        find_validation_metas(self)
            .find_map(|meta| match meta {
                Meta::NameValue(namevalue) if namevalue.path.is_ident("error") => {
                    Some(&namevalue.lit)
                }
                _ => None,
            })
            .map(|lit| match lit {
                Lit::Str(s) => s.parse().map_err(|e| {
                    syn::Error::new(
                        lit.span(),
                        format!("Failed to parse validation error type: {}", e),
                    )
                }),
                _ => Err(syn::Error::new(
                    lit.span(),
                    "Expected string literal as validation error type, but got other literal",
                )),
            })
            .transpose()
    }
}

/// Find `#[(opaque_typedef(validate(**Metas HERE**))]`.
fn find_validation_metas(meta: &Meta) -> impl Iterator<Item = &Meta> {
    std::iter::once(meta)
        .filter_map(|meta| match meta {
            Meta::List(metalist) if metalist.path.is_ident("opaque_typedef") => {
                Some(&metalist.nested)
            }
            _ => None,
        })
        .flat_map(|nested_list| nested_list)
        .filter_map(|nested| match nested {
            NestedMeta::Meta(Meta::List(metalist)) if metalist.path.is_ident("validate") => {
                Some(&metalist.nested)
            }
            _ => None,
        })
        .flat_map(|nested_list| {
            nested_list.iter().filter_map(|nested| match nested {
                NestedMeta::Meta(meta) => Some(meta),
                _ => None,
            })
        })
}

/// Checks if the given path consists with the given idents (given as `&str`).
fn eq_path_components(path: &Path, components: &[&str]) -> bool {
    path.segments.len() == components.len()
        && path
            .segments
            .iter()
            .zip(components)
            .all(|(seg, comp)| match seg.arguments {
                syn::PathArguments::None => seg.ident == comp,
                _ => false,
            })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_path_components() {
        let foo: Path = syn::parse_str("foo").unwrap();
        assert!(eq_path_components(&foo, &["foo"]));
        assert!(!eq_path_components(&foo, &["bar"]));

        let foo_bar: Path = syn::parse_str("foo::bar").unwrap();
        assert!(eq_path_components(&foo_bar, &["foo", "bar"]));
        assert!(!eq_path_components(&foo_bar, &["foo"]));
        assert!(!eq_path_components(&foo_bar, &["bar"]));
        assert!(!eq_path_components(&foo_bar, &["foo::bar"]));
    }

    #[test]
    fn test_has_unsized_repr() {
        let repr_c: Meta = syn::parse_str("repr(C)").unwrap();
        assert!(repr_c.has_unsized_repr());

        let repr_transparent: Meta = syn::parse_str("repr(transparent)").unwrap();
        assert!(repr_transparent.has_unsized_repr());

        let repr_c_with_dummy: Meta = syn::parse_str("repr(dummy = 42, C, dummy)").unwrap();
        assert!(repr_c_with_dummy.has_unsized_repr());

        let repr_dummy: Meta = syn::parse_str("repr(dummy)").unwrap();
        assert!(!repr_dummy.has_unsized_repr());

        let no_repr: Meta = syn::parse_str("foo(C)").unwrap();
        assert!(!no_repr.has_unsized_repr());
    }

    #[test]
    fn test_has_level2_word() {
        let foo_bar_path: Meta = syn::parse_str("foo::bar").unwrap();
        assert!(foo_bar_path.has_level2_word("foo", "bar"));

        let foo_bar: Meta = syn::parse_str("foo(dummy, bar, dummy(dummy))").unwrap();
        assert!(foo_bar.has_level2_word("foo", "bar"));
        assert!(!foo_bar.has_level2_word("dummy", "dummy"));

        let foo_bar_baz: Meta = syn::parse_str("foo(dummy, bar(baz), dummy(dummy))").unwrap();
        assert!(!foo_bar_baz.has_level2_word("foo", "bar"));
    }
}
