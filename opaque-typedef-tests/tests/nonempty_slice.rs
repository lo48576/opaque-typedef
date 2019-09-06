//! Non-empty slice.

use std::ops::{Bound, RangeBounds};

use opaque_typedef::{OpaqueTypedefUnsized, OpaqueTypedefUnsizedMut};

/// An error indicating the slice is empty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Empty;

fn validate_slice<T>(v: &[T]) -> Result<&[T], Empty> {
    if v.is_empty() {
        Err(Empty)
    } else {
        Ok(v)
    }
}

/// Non-empty slice.
#[derive(Debug, OpaqueTypedefUnsized, OpaqueTypedefUnsizedMut)]
#[repr(transparent)]
#[opaque_typedef(validate(error = "Empty", validator = "validate_slice"))]
pub struct NonEmptySlice<T>([T]);

impl<T> NonEmptySlice<T> {
    pub fn new(slice: &[T]) -> Result<&Self, Empty> {
        Self::try_from_inner(slice)
    }

    /// Returns subslice as `NonEmptySlice`.
    // NOTE: `SliceIndex<[T]>` is not implemented for `(Bound<usize>, Bound<usize>)`.
    // See <https://github.com/rust-lang/rust/issues/49976>.
    pub fn subslice<B: RangeBounds<usize>>(&self, range: B) -> Result<&Self, Empty> {
        let start = match bound_cloned(range.start_bound()) {
            Bound::Included(v) => v,
            Bound::Excluded(v) => v.checked_add(1).ok_or(Empty)?,
            Bound::Unbounded => 0,
        };
        let subslice = match bound_cloned(range.end_bound()) {
            Bound::Included(v) => &self.as_inner()[start..=v],
            Bound::Excluded(v) => &self.as_inner()[start..v],
            Bound::Unbounded => &self.as_inner()[start..],
        };

        Self::try_from_inner(subslice)
    }

    /// Returns the slice as `&[T]`.
    pub fn as_slice(&self) -> &[T] {
        self.as_inner()
    }
}

// See <https://github.com/rust-lang/rust/issues/61356>.
fn bound_cloned<T: Clone>(bound: Bound<&T>) -> Bound<T> {
    match bound {
        Bound::Excluded(v) => Bound::Excluded(v.clone()),
        Bound::Included(v) => Bound::Included(v.clone()),
        Bound::Unbounded => Bound::Unbounded,
    }
}

#[cfg(test)]
mod non_empty_slice {
    use super::*;

    #[test]
    fn from_empty() {
        assert!(NonEmptySlice::<i64>::new(&[]).is_err());
    }

    #[test]
    fn subslice_empty() {
        let slice = NonEmptySlice::new(&[0, 1, 2, 4, 8]).unwrap();
        assert!(slice.subslice(5..).is_err());
    }

    #[test]
    fn subslice_nonempty() {
        let slice = NonEmptySlice::new(&[0, 1, 2, 4, 8]).unwrap();
        let expected = NonEmptySlice::new(&[1, 2, 4]).unwrap();
        assert_eq!(
            slice.subslice(1..4).unwrap().as_slice(),
            expected.as_slice()
        );
    }
}
