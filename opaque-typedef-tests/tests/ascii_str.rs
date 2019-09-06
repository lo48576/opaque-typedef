//! Ascii string.

use opaque_typedef::{
    OpaqueTypedefSized, OpaqueTypedefSizedMut, OpaqueTypedefUnsized, OpaqueTypedefUnsizedMut,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AsciiError {
    valid_up_to: usize,
}

impl AsciiError {
    pub fn valid_up_to(&self) -> usize {
        self.valid_up_to
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FromAsciiError {
    string: String,
    error: AsciiError,
}

impl FromAsciiError {
    pub fn into_string(self) -> String {
        self.string
    }

    pub fn ascii_error(&self) -> AsciiError {
        self.error
    }
}

fn validate_str(s: &str) -> Result<&str, AsciiError> {
    if let Some(pos) = s.bytes().position(|v| !v.is_ascii()) {
        Err(AsciiError { valid_up_to: pos })
    } else {
        Ok(s)
    }
}

fn validate_string(s: String) -> Result<String, FromAsciiError> {
    match validate_str(&s) {
        Ok(_) => Ok(s),
        Err(e) => Err(FromAsciiError {
            string: s,
            error: e,
        }),
    }
}

/// Ascii string slice.
#[derive(Debug, OpaqueTypedefUnsized, OpaqueTypedefUnsizedMut)]
#[repr(transparent)]
#[opaque_typedef(validate(error = "AsciiError", validator = "validate_str"))]
pub struct AsciiStr(str);

/// Ascii owned string.
#[derive(Debug, Clone, OpaqueTypedefSized, OpaqueTypedefSizedMut)]
#[opaque_typedef(validate(error = "FromAsciiError", validator = "validate_string"))]
pub struct AsciiString(String);

#[cfg(test)]
mod ascii_str {
    use super::*;

    #[test]
    fn assert_traits()
    where
        AsciiStr: OpaqueTypedefUnsized + OpaqueTypedefUnsizedMut,
    {
    }

    #[test]
    fn try_from_inner() {
        let inner = "hello";
        let my = AsciiStr::try_from_inner(inner).unwrap();
        assert_eq!(my.as_inner(), inner);
    }

    #[test]
    fn from_inner_unchecked() {
        let inner = "hello";
        let my = unsafe { AsciiStr::from_inner_unchecked(inner) };
        assert_eq!(my.as_inner(), inner);
    }

    #[test]
    fn try_from_inner_fail() {
        let inner = "hello\u{FFFD}";
        let err = AsciiStr::try_from_inner(inner).unwrap_err();
        assert_eq!(err.valid_up_to(), 5);
    }

    #[test]
    fn try_from_inner_mut() {
        let mut inner: String = "hello".to_owned();
        let inner_readonly = "hello";
        let my: &mut AsciiStr = AsciiStr::try_from_inner_mut(&mut inner).unwrap();
        assert_eq!(my.as_inner_mut(), inner_readonly);
    }

    #[test]
    fn try_from_inner_mut_fail() {
        let inner = "hello\u{FFFD}";
        let err = AsciiStr::try_from_inner(inner).unwrap_err();
        assert_eq!(err.valid_up_to(), 5);
    }

    #[test]
    fn as_inner_mut() {
        let mut inner = "hello".to_owned();
        let my = AsciiStr::try_from_inner_mut(&mut inner).unwrap();
        let _: &mut str = my.as_inner_mut();
    }
}

#[cfg(test)]
mod ascii_string {
    use super::*;

    #[test]
    fn assert_traits()
    where
        AsciiString: OpaqueTypedefSized + OpaqueTypedefSizedMut,
    {
    }

    #[test]
    fn try_from_inner() {
        let inner = "hello".to_owned();
        let my = AsciiString::try_from_inner(inner.clone()).unwrap();
        assert_eq!(my.as_inner(), &inner);
    }

    #[test]
    fn from_inner_unchecked() {
        let inner = "hello".to_owned();
        let my = unsafe { AsciiString::from_inner_unchecked(inner.clone()) };
        assert_eq!(my.as_inner(), &inner);
    }

    #[test]
    fn try_from_inner_fail() {
        let inner = "hello\u{FFFD}".to_owned();
        let err = AsciiString::try_from_inner(inner.clone()).unwrap_err();
        assert_eq!(err.ascii_error().valid_up_to(), 5);
        assert_eq!(err.into_string(), inner);
    }

    #[test]
    fn as_inner_mut() {
        let inner = "hello".to_owned();
        let mut my = AsciiString::try_from_inner(inner).unwrap();
        let _: &mut String = my.as_inner_mut();
    }
}
