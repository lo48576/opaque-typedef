//! Internally same as std string, but different type.

use opaque_typedef::{
    OpaqueTypedefSized, OpaqueTypedefSizedInfallible, OpaqueTypedefSizedMut, OpaqueTypedefUnsized,
    OpaqueTypedefUnsizedInfallible, OpaqueTypedefUnsizedInfallibleMut, OpaqueTypedefUnsizedMut,
};

/// My string slice.
#[derive(
    Debug,
    OpaqueTypedefUnsized,
    OpaqueTypedefUnsizedMut,
    OpaqueTypedefUnsizedInfallible,
    OpaqueTypedefUnsizedInfallibleMut,
)]
#[repr(transparent)]
pub struct MyStr(str);

/// My owned string.
#[derive(Debug, Clone, OpaqueTypedefSized, OpaqueTypedefSizedMut, OpaqueTypedefSizedInfallible)]
pub struct MyString(String);

#[cfg(test)]
mod my_str {
    use super::*;

    #[test]
    fn assert_traits()
    where
        MyStr: OpaqueTypedefUnsized
            + OpaqueTypedefUnsizedMut
            + OpaqueTypedefUnsizedInfallible
            + OpaqueTypedefUnsizedInfallibleMut,
    {
    }

    #[test]
    fn try_from_inner() {
        let inner = "hello";
        let my = MyStr::try_from_inner(inner).unwrap();
        assert_eq!(my.as_inner(), inner);
    }

    #[test]
    fn from_inner_unchecked() {
        let inner = "hello";
        let my = unsafe { MyStr::from_inner_unchecked(inner) };
        assert_eq!(my.as_inner(), inner);
    }

    #[test]
    fn try_from_inner_mut() {
        let mut inner = "hello".to_owned();
        let inner_readonly = "hello";
        let my: &mut MyStr = MyStr::try_from_inner_mut(&mut inner).unwrap();
        assert_eq!(my.as_inner_mut(), inner_readonly);
    }

    #[test]
    fn as_inner_mut() {
        let mut inner = "hello".to_owned();
        let my = MyStr::try_from_inner_mut(&mut inner).unwrap();
        let _: &mut str = my.as_inner_mut();
    }

    #[test]
    fn from_inner_infallible() {
        let inner = "hello";
        let my = MyStr::from_inner(inner);
        assert_eq!(my.as_inner(), inner);
    }

    #[test]
    fn from_inner_infallible_mut() {
        let mut inner = "hello".to_owned();
        let inner_readonly = "hello";
        let my: &mut MyStr = MyStr::from_inner_mut(&mut inner);
        assert_eq!(my.as_inner_mut(), inner_readonly);
    }
}

#[cfg(test)]
mod my_string {
    use super::*;

    #[test]
    fn assert_traits()
    where
        MyString: OpaqueTypedefSized + OpaqueTypedefSizedMut + OpaqueTypedefSizedInfallible,
    {
    }

    #[test]
    fn try_from_inner() {
        let inner = "hello".to_owned();
        let my = MyString::try_from_inner(inner.clone()).unwrap();
        assert_eq!(my.as_inner(), &inner);
    }

    #[test]
    fn from_inner_unchecked() {
        let inner = "hello".to_owned();
        let my = unsafe { MyString::from_inner_unchecked(inner.clone()) };
        assert_eq!(my.as_inner(), &inner);
    }

    #[test]
    fn as_inner_mut() {
        let inner = "hello".to_owned();
        let mut my = MyString::try_from_inner(inner).unwrap();
        let _: &mut str = my.as_inner_mut();
    }

    #[test]
    fn from_inner() {
        let inner = "hello".to_owned();
        let my = MyString::from_inner(inner.clone());
        assert_eq!(my.as_inner(), &inner);
    }
}
