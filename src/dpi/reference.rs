use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::num::NonZeroU16;
use std::str::FromStr;

/// Represents a DPI Library Code
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lib(NonZeroU16);

/// A possible error value when converting a `DPI Library Code` from a `u16` or `&str`
///
/// This error indicates that the supplied input was not a valid number,
/// was less than 10000 or greater than 65535.
pub struct InvalidCode {
    _priv: (),
}

macro_rules! lib_codes {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => {
        impl Lib {
        $(
            $(#[$docs])*
            pub const $konst: Lib = Lib(unsafe { NonZeroU16::new_unchecked($num) });
        )+

        }

        fn get_value(num: u16) -> Option<&'static str> {
            match num {
                $(
                $num => Some($phrase),
                )+
                _ => None
            }
        }
    }
}

lib_codes! {
    /// 15000 Social Security Number abbreviated
    (15000, TEXT_SSN_FULL, r"Social Security Number");
    /// 15001 Social Security Number abbreviated
    (15001, TEXT_SSN_ABBR, r"SSN");
    /// 15002 Account
    (15002, TEXT_ACCOUNT, r"account");

    /// Regex PII
    /// 20001 Avenue or Ave (case insensitive)
    (20001, REGEX_ADDR_AVE, r"/avenue|\bave\b/gim");
    /// 20002 Boulevard or Blvrd (case insensitive)
    (20002, REGEX_ADDR_BLVD, r"/boulevard|\bblvd\b/gim");
    /// 20003 Drive or Dr (case insensitive)
    (20003, REGEX_ADDR_DR, r"/drive|\bdr\b/gim");
    /// 20004 East(case insensitive)
    (20004, REGEX_ADDR_EAST, r"/east/gim");
    /// 20005 Highway or Hwy (case insensitive)
    (20005, REGEX_ADDR_HWY, r"/highway|\bhwy\b/gim");
    /// 20006 Lane or Ln (case insensitive)
    (20006, REGEX_ADDR_LN, r"/lane|\bln\b/gim");
    /// 20007 North (case insensitive)
    (20007, REGEX_ADDR_NORTH, r"/north/gim");
    /// 20008 Road or Rd (case insensitive)
    (20008, REGEX_ADDR_RD, r"/road|\brd\b/gim");
    /// 20009 South (case insensitive)
    (20009, REGEX_ADDR_SOUTH, r"/south/gim");
    /// 20010 Street or St or Str (case insensitive)
    (20010, REGEX_ADDR_STR, r"/street|\bst\b|\bstr\b/gim");
    /// 20011 Township or Twp (case insensitive)
    (20011, REGEX_ADDR_TWP, r"/township|\btwp\b/gim");
    /// 20012 West (case insensitive)
    (20012, REGEX_ADDR_WEST, r"/west/gim");
    /// 20013 US phone number
    (20013, REGEX_US_PHONE, r"/([0-9]{10})|([0-9]{7})|^(\([0-9]{3}\))|([0-9]{3}-[0-9]{4})|([ext])/gim");
    /// 20014 Email (case insensitive)
    (20014, REGEX_EMAIL, r"/([\w\.-]+)@([\da-zA-Z\.-]+)\.([a-zA-Z\.]{2,6})/gim");

    /// 25000 Social Security Number with dashes
    (25000, REGEX_SSN_DASHES, r"^\d{3}-\d{2}-\d{4}$");
    /// 25000 Social Security Number without dashes
    (25001, REGEX_SSN_NODASHES, r"^\d{9}$");
    /// 25002 Account - word
    (25002, REGEX_ACCOUNT, r"/account|\bacct\b|\bacc\b|\ba/c\b/gim");
    //(25002, REGEX_ACCOUNT, r"([Aa]..[aeiouAEIOU]{2}..)");

    /// 27000 Credit Card Number - Visa
    (27000, REGEX_CREDIT_VISA, r"4[0-9]{12}(?:[0-9]{3})?");
    /// 27001 Credit Card Number - MasterCard
    (27001, REGEX_CREDIT_MASTER, r"(?:5[1-5][0-9]{2}|222[1-9]|22[3-9][0-9]|2[3-6][0-9]{2}|27[01][0-9]|2720)[0-9]{12}");
    /// 27002 Credit Card Number - AMEX
    (27002, REGEX_CREDIT_AMEX, r"3[47][0-9]{13}");
    /// 27003 Credit Card Number - Diners CLub
    (27003, REGEX_CREDIT_DINERS, r"3(?:0[0-5]|[68][0-9])[0-9]{11}");
    /// 27004 Credit Card Number - Discover
    (27004, REGEX_CREDIT_DISCOVER, r"6(?:011|5[0-9]{2})[0-9]{12}");
    /// 27005 Credit Card Number - JCB
    (27005, REGEX_CREDIT_JCB, r"(?:2131|1800|35\d{3})\d{11}");

    /// 35000 Social Security Number with dashes
    (35000, PTTRN_SSN_DASHES, r"###@##@####");
    /// 35001 Social Security Number without dahses
    (35001, PTTRN_SSN_NODASHES, r"#########");
    /// 35002 Account Camel
    (35002, PTTRN_ACCOUNT_CAMEL, r"Vccvvcc");
    /// 35003 Account Upper
    (35003, PTTRN_ACCOUNT_UPPER, r"VCCVVCC");
    /// 35004 Account Lower
    (35004, PTTRN_ACCOUNT_LOWER, r"vccvvcc");
}

impl InvalidCode {
    fn new() -> InvalidCode {
        InvalidCode { _priv: () }
    }
}

impl fmt::Debug for InvalidCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InvalidCode")
            // skip _priv noise
            .finish()
    }
}

impl fmt::Display for InvalidCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid code")
    }
}

impl Error for InvalidCode {}

/// The codes used in the DPI library are catalogued based on type of codes:
///
/// 1xxxx = Key Words for PII
/// 15xxx = Key Words for NPPI (Non-public Personal Information)
/// 2xxxx = Regular Expressions for PII
/// 25xxx = Regular Expressions for NPPI
/// 27xxx = Regular Expressions for PCI
/// 3xxxx = Pattern Definitions for PII
/// 35xxx = Pattern Definitions for NPPI
///
impl Lib {
    /// Returns a &str representation of the `Code`
    ///
    /// The return value representation of the code.
    ///
    /// # Example
    ///
    /// ```rust
    /// use pbd::dpi::reference::Lib;
    ///     
    /// let code = Lib::TEXT_SSN_ABBR;
    /// assert_eq!(code.as_str(), Some("SSN"));
    /// ```
    #[inline]
    pub fn as_str(&self) -> Option<&str> {
        get_value(self.0.get())
    }

    /// Returns the `u16` corresponding to this `Code`.
    ///
    /// # Note
    ///
    /// This is the same as the `From<Lib>` implementation, but
    /// included as an inherent method because that implementation doesn't
    /// appear in rustdocs, as well as a way to force the type instead of
    /// relying on inference.
    ///
    /// # Example
    ///
    /// ```rust    
    /// use pbd::dpi::reference::Lib;
    ///     
    /// let code = Lib::TEXT_SSN_ABBR;
    /// assert_eq!(code.as_u16(), 15001);
    /// ```
    #[inline]
    pub fn as_u16(&self) -> u16 {
        (*self).into()
    }

    /// Converts a &[u8] to a status code
    pub fn from_bytes(src: &[u8]) -> Result<Lib, InvalidCode> {
        let mut src_vec = Vec::new();

        if src.len() != 5 {
            return Err(InvalidCode::new());
        }

        for s in src {
            src_vec.push(s.wrapping_sub(b'0') as u16);
        }

        if src_vec[0] == 0
            || (src_vec[0] > 6
                && src_vec[1] > 5
                && src_vec[2] > 5
                && src_vec[3] > 3
                && src_vec[4] > 5)
        {
            return Err(InvalidCode::new());
        }

        let code = (src_vec[0] * 10000)
            + (src_vec[1] * 1000)
            + (src_vec[2] * 100)
            + (src_vec[3] * 10)
            + (src_vec[4]);
        NonZeroU16::new(code).map(Lib).ok_or_else(InvalidCode::new)
    }

    /// Converts a u16 to a library code.
    ///
    /// The function validates the correctness of the supplied u16. It must be
    /// greater or equal to 10000 and less than 65535.
    ///
    /// # Example
    ///
    /// ```rust
    /// use pbd::dpi::reference::Lib;
    ///
    /// let ssn = Lib::from_u16(15001).unwrap();
    /// assert_eq!(ssn, Lib::TEXT_SSN_ABBR);
    ///
    /// let err = Lib::from_u16(1000);
    /// assert!(err.is_err());
    /// ```
    #[inline]
    pub fn from_u16(src: u16) -> Result<Lib, InvalidCode> {
        if src < 10000 {
            return Err(InvalidCode::new());
        }

        NonZeroU16::new(src).map(Lib).ok_or_else(InvalidCode::new)
    }

    /// Get the standardised `reason-phrase` for this standard.
    ///
    /// This is mostly here for human readable understanding, but could potentially have application
    /// at other times.
    ///
    /// The reason phrase is defined as being exclusively for human readers. You should avoid
    /// deriving any meaning from it at all costs.
    ///
    /// # Example
    ///
    /// ```rust
    /// use pbd::dpi::reference::Lib;
    ///
    /// let code = Lib::TEXT_SSN_ABBR;
    /// assert_eq!(code.get_value(), Some("SSN"));
    /// ```
    pub fn get_value(&self) -> Option<&'static str> {
        get_value(self.0.get())
    }
}

impl fmt::Debug for Lib {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

/// Formats the code, *including* the value.
///
/// # Example
///
/// ```rust
/// use pbd::dpi::reference::Lib;
///
/// assert_eq!(format!("{}", Lib::TEXT_SSN_ABBR), "SSN");
/// ```
impl fmt::Display for Lib {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_value().unwrap_or("<unknown code>"))
    }
}

impl PartialEq<u16> for Lib {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        self.as_u16() == *other
    }
}

impl PartialEq<Lib> for u16 {
    #[inline]
    fn eq(&self, other: &Lib) -> bool {
        *self == other.as_u16()
    }
}

impl From<Lib> for u16 {
    #[inline]
    fn from(status: Lib) -> u16 {
        status.0.get()
    }
}

impl FromStr for Lib {
    type Err = InvalidCode;

    fn from_str(s: &str) -> Result<Lib, InvalidCode> {
        Lib::from_bytes(s.as_ref())
    }
}

impl<'a> From<&'a Lib> for Lib {
    #[inline]
    fn from(t: &'a Lib) -> Self {
        *t
    }
}

impl<'a> TryFrom<&'a [u8]> for Lib {
    type Error = InvalidCode;

    #[inline]
    fn try_from(t: &'a [u8]) -> Result<Self, Self::Error> {
        Lib::from_bytes(t)
    }
}

impl<'a> TryFrom<&'a str> for Lib {
    type Error = InvalidCode;

    #[inline]
    fn try_from(t: &'a str) -> Result<Self, Self::Error> {
        t.parse()
    }
}

impl TryFrom<u16> for Lib {
    type Error = InvalidCode;

    #[inline]
    fn try_from(t: u16) -> Result<Self, Self::Error> {
        Lib::from_u16(t)
    }
}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_str() {
        let code = Lib::TEXT_SSN_ABBR;
        assert_eq!(code.as_str(), Some("SSN"));
    }

    #[test]
    fn test_as_u16() {
        let code = Lib::TEXT_SSN_ABBR;
        assert_eq!(code.as_u16(), 15001);
    }

    #[test]
    fn test_debug_code() {
        assert_eq!(format!("{:?}", Lib::TEXT_SSN_ABBR), "15001");
    }

    #[test]
    fn test_display_code() {
        assert_eq!(format!("{}", Lib::TEXT_SSN_ABBR), "SSN");
    }

    #[test]
    fn test_from_bytes() {
        assert!(!Lib::from_bytes("15001".as_bytes()).is_err());
        assert!(!Lib::from_bytes("36666".as_bytes()).is_err());
        assert!(Lib::from_bytes("1000".as_bytes()).is_err());
        assert!(Lib::from_bytes("99999".as_bytes()).is_err());
    }

    #[test]
    fn test_from_str() {
        let ssn = Lib::from_str("15001").unwrap();
        assert_eq!(ssn, Lib::TEXT_SSN_ABBR);

        let err = Lib::from_str("ssn");
        assert!(err.is_err());
    }

    #[test]
    fn test_from_u16() {
        let ssn = Lib::from_u16(15001).unwrap();
        assert_eq!(ssn, Lib::TEXT_SSN_ABBR);

        let err = Lib::from_u16(1000);
        assert!(err.is_err());
    }

    #[test]
    fn test_invalid_code() {
        let invalid_code = InvalidCode::new();
        assert_eq!(format!("{:?}", invalid_code), "InvalidCode");
        assert_eq!(format!("{}", invalid_code), "invalid code");
    }

    #[test]
    fn test_nppi_code() {
        let code = Lib::TEXT_SSN_ABBR;
        assert_eq!(code.get_value(), Some(r"SSN"));
        assert_eq!(Lib::from_u16(15001).unwrap(), Lib::TEXT_SSN_ABBR);
    }

    #[test]
    fn test_try_from_lib() {
        let try_successful_lib = Lib::try_from(Lib::TEXT_SSN_ABBR);
        assert!(try_successful_lib.is_ok());
    }

    #[test]
    fn test_try_from_str() {
        let try_successful_str = Lib::try_from("15000");
        assert!(try_successful_str.is_ok());
    }

    #[test]
    fn test_try_from_u8() {
        let try_successful_u8 = Lib::try_from("15000".as_bytes());
        assert!(try_successful_u8.is_ok());
    }

    #[test]
    fn test_try_from_u16() {
        let try_successful_u16 = Lib::try_from(15000 as u16);
        assert!(try_successful_u16.is_ok());
    }
}
