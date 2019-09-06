//! Easy opaque typedef for Rust.
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

// Re-export custom derives through this crate when `derive` feature is enabled.
#[cfg(feature = "derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate opaque_typedef_macros;
#[cfg(feature = "derive")]
#[allow(unused_imports)]
#[doc(hidden)]
pub use opaque_typedef_macros::*;

/// A trait for an opaque type alias of a sized type.
pub trait OpaqueTypedefSized: Sized {
    /// Inner sized type.
    type Inner: Sized;
    /// Error type for conversion from the inner type.
    // `Error` requires `Debug` impl because `Result::expect()` does.
    type Error: std::fmt::Debug;

    /// Creates a new value from the given inner value.
    fn try_from_inner(inner: Self::Inner) -> Result<Self, Self::Error>;
    /// Creates a new value without validation.
    unsafe fn from_inner_unchecked(inner: Self::Inner) -> Self;
    /// Returns the inner value.
    fn into_inner(self) -> Self::Inner;
    /// Returns a reference to the inner value.
    fn as_inner(&self) -> &Self::Inner;
}

/// A trait for an opaque type alias of a sized type creatable with infallible conversion.
pub trait OpaqueTypedefSizedInfallible:
    OpaqueTypedefSized<Error = std::convert::Infallible>
{
    /// Creates a new value from the given inner value, without possibility of failure.
    fn from_inner(inner: Self::Inner) -> Self;
}

/// A trait for an opaque type alias of a mutable sized type.
pub trait OpaqueTypedefSizedMut: OpaqueTypedefSized {
    /// Returns a mutable reference to the inner value.
    fn as_inner_mut(&mut self) -> &mut Self::Inner;
}

/// A trait for an opaque type alias of an unsized type.
pub trait OpaqueTypedefUnsized {
    /// Inner unsized type.
    type Inner: ?Sized;
    /// Error type for conversion from the inner type.
    // `Error` requires `Debug` impl because `Result::expect()` does.
    type Error: std::fmt::Debug;

    /// Creates a new value from the given inner value.
    fn try_from_inner(inner: &Self::Inner) -> Result<&Self, Self::Error>;
    /// Creates a new value without validation.
    unsafe fn from_inner_unchecked(inner: &Self::Inner) -> &Self;
    /// Returns a reference to the inner value.
    fn as_inner(&self) -> &Self::Inner;
}

/// A trait for an opaque type alias of a unsized type creatable with infallible conversion.
pub trait OpaqueTypedefUnsizedInfallible:
    OpaqueTypedefUnsized<Error = std::convert::Infallible>
{
    /// Creates a new value from the given inner value, without possibility of failure.
    fn from_inner(inner: &Self::Inner) -> &Self;
}

/// A trait for an opaque type alias of a mutable unsized type.
pub trait OpaqueTypedefUnsizedMut: OpaqueTypedefUnsized {
    /// Creates a new value from the given inner value.
    fn try_from_inner_mut(inner: &mut Self::Inner) -> Result<&mut Self, Self::Error>;
    /// Creates a new value without validation.
    unsafe fn from_inner_unchecked_mut(inner: &mut Self::Inner) -> &mut Self;
    /// Returns a mutable reference to the inner slice.
    fn as_inner_mut(&mut self) -> &mut Self::Inner;
}

/// A trait for an opaque type alias of a unsized type creatable with infallible conversion.
pub trait OpaqueTypedefUnsizedInfallibleMut:
    OpaqueTypedefUnsizedMut + OpaqueTypedefUnsizedInfallible
{
    /// Creates a new value from the given inner value, without possibility of failure.
    fn from_inner_mut(inner: &mut Self::Inner) -> &mut Self;
}
