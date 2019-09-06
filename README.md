# opaque-typedef

[![Build Status](https://travis-ci.org/lo48576/opaque-typedef.svg?branch=develop)](https://travis-ci.org/lo48576/opaque-typedef)
![Minimum rustc version: 1.37](https://img.shields.io/badge/rustc-1.37+-lightgray.svg)

This crate helps developers to define opaque type alias with less boilerplates and `unsafe`.

## Safety notes

This library may generate codes with `unsafe` but some conditions cannot be checked at compile time by `opaque-typedef` library, so **the users should guarantee the conditions below**.

* For types deriving `OpaqueTypedefUnsized`, the field marked as `inner` should have an unsized type, and all other fields should have zero-sized types.

If the conditions are not met, the generated codes will be unsound and will cause undefined behavior.

## How to use

### Derive

For sized types, you can derive:

* `OpaqueTypedefSized`
    + Provides basic (fallible) conversions.
* `OpaqueTypedefSizedMut`
    + Provides mutable access to the inner field.
    + This requires `OpaqueTypedefSized` to be derived.
* `OpaqueTypedefSizedInfallible`
    + Provides infallible conversions.
    + This requires `OpaqueTypedefSized` to be derived.
    + It is recommended to derive this if a validator is not specified.

For unsized types, you can derive:

* `OpaqueTypedefUnsized`
    + Provides fallible conversions from / to an immutable reference.
* `OpaqueTypedefUnsizedMut`
    + Provides fallible conversions from / to a mutable reference.
    + This requires `OpaqueTypedefUnsized` to be derived.
* `OpaqueTypedefSizedInfallible`
    + Provides infallible conversions from / to an immutable reference.
    + This requires `OpaqueTypedefUnsized` to be derived.
    + It is recommended to derive this if a validator is not specified.
* `OpaqueTypedefUnsizedInfallibleMut`
    + Provides infallible conversions from / to a mutable reference.
    + This requires `OpaqueTypedefUnsizedMut` and `OpaqueTypedefUnsizedInfallible` to be derived.
    + It is recommended to derive this if a validator is not specified and `OpaqueTypedefUnsizedMut` is derived.

Note that **the inner field should be unsized type for `OpaqueTypedefUnsized`**.
If not, undefined behavior can happen.
It is library user's responsibility to guarantee that.

These traits are intended to be used by library developers (but not by users).

### Type-level attributes

#### `repr` for unsized types

For unsized types, `#[repr(transparent)]` or `#[repr(C)]` should be specified.
Without this, compilation fails.

These are required to force common internal representation for the inner and outer types.
This is necessary to prevent undefined behavior on conversion from the inner type to the outer type.

#### Validator and error

Validator can be specified as below:

```rust
#[derive(OpaqueTypedefSized)]
#[opaque_typedef(validate(
    // Error type.
    error = "Error",
    // Validation function.
    validator = "validation_function"
))]
struct Outer(Inner);
```

The error type is used as the associated `Error` type of `OpaqueTypedefSized` or `OpaqueTypedefUnsized` trait.
If not specified, `std::convert::Infallilble` is used.

The validator function can be function name or closure.
It will be used in `(value_you_specified)(inner_value)`.

For `derive(OpaqueTypedefSized)`, validator function should receive `Inner` and return `Result<Inner, Error>`.

For `derive(OpaqueTypedefUnsized)`, validator function should receive `&Inner` and return `Result<&Inner, Error>`.

#### Hiding trait impl document

You can hide the trait impl from rustdoc document by `#[opaque_typedef(hide_base_impl_docs)]`.
This is recommended to use, if there are special reason to show the trait impl.

```rust
#[derive(OpaqueTypedefSized)]
#[opaque_typedef(hide_base_impl_docs)] // THIS
pub struct Wrapper<T>(T);
```

### Field-level attributes

#### Inner field

A type to be opaque typedef'ed can have zero or more zero-sized types.
In such case, `opaque-typedef` cannot distinguish which one is non-zero-sized type, so users should specify that by `#[opaque_typedef(inner)]`.

```rust
#[derive(OpaqueTypedefSized)]
pub struct Tagged<T, Tag> {
    /// Inner data.
    #[opaque_typedef(inner)] // THIS
    inner: T,
    /// Tag (zero-sized type).
    tag: Tag,
}
```

If the type has only one field, the inner field can be automatically detected and the attribute can be omitted.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE.txt](LICENSE-APACHE.txt) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT.txt](LICENSE-MIT.txt) or
  <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
