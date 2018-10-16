
use std::fmt;
use std::str::{self, FromStr};
use std::error;
use std::string::String;
use serde::{de, ser};

/// A FourCC is a sequence of four bytes used to uniquely identify data chunks.
///
/// The byte sequence is usually restricted to ASCII printable characters, with space characters
/// reserved for padding shorter sequences. Case sensitivity is preserved.
///
/// Four-byte identifiers are useful because they can be made up of four human-readable characters
/// with mnemonic qualities, while still fitting in the four-byte memory space typically allocated
/// for integers in 32-bit systems (although endian issues may make them less readable). Thus, the
/// codes can be used efficiently in program code as integers, as well as giving cues in binary
/// data streams when inspected.
///
/// Some FourCCs however, do contain non-printable characters, and are not human-readable without
/// special formatting for display.

#[derive(PartialEq, Clone)]
pub struct FourCC {
    value: [u8; 4]
}

/// Error returned from parsing a `FourCC` in the `FromStr` implementation.
#[derive(Debug, Clone)]
pub struct FourCCParseError {
    _reason: String
}

impl FromStr for FourCC {
    type Err = FourCCParseError;
    fn from_str(id: &str) -> Result<FourCC, FourCCParseError> {
        if id.len() < 4 {
            return Err(FourCCParseError { _reason: String::from("given FourCC too short") })
        }
        else if id.len() > 4 {
            return Err(FourCCParseError { _reason: String::from("given FourCC too long") })
        }
        let bytes = id.as_bytes();
        Ok(FourCC { value: [
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3]
        ]})
    }
}

impl fmt::Debug for FourCC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for FourCC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match str::from_utf8(&self.value) {
            Ok(str) => write!(f, "{}", str)?,
            Err(err) => write!(f, "{}", err)?
        }
        Ok(())
    }
}
impl fmt::Display for FourCCParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format!("FourCC parsing error: {}", self._reason).fmt(f)
    }
}

impl error::Error for FourCCParseError {
    fn description(&self) -> &str {
        self._reason.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new() {
        let fourcc = FourCC::from_str("ABCD").unwrap();
        assert_eq!(fourcc.value[0], b'A');
        assert_eq!(fourcc.value[1], b'B');
        assert_eq!(fourcc.value[2], b'C');
        assert_eq!(fourcc.value[3], b'D');
    }
    #[test]
    #[should_panic(expected = "given FourCC too short")]
    fn create_new_too_short() {
        let _fourcc = FourCC::from_str("ABC").unwrap();
    }
    #[test]
    #[should_panic(expected = "given FourCC too long")]
    fn create_new_too_long() {
        let _fourcc = FourCC::from_str("ABCDE").unwrap();
    }
    #[test]
    fn should_return_fourcc_as_str() {
        let fourcc = FourCC::from_str("ABCD").unwrap();
        let fourcc_str = format!("{}", fourcc);
        assert_eq!("ABCD", fourcc_str);
    }
    #[test]
    fn should_return_too_short_fourcc_as_str() {
        let fourcc = FourCC::from_str("ABC");
        let fourcc_str = match fourcc {
            Ok(fourcc) => format!("{}", fourcc),
            Err(err) => format!("{}", err)
        };
        assert_eq!("FourCC parsing error: given FourCC too short", fourcc_str);
    }
    #[test]
    fn should_return_too_long_fourcc_as_str() {
        let fourcc = FourCC::from_str("ABCDE");
        let fourcc_str = match fourcc {
            Ok(fourcc) => format!("{}", fourcc),
            Err(err) => format!("{}", err)
        };
        assert_eq!("FourCC parsing error: given FourCC too long", fourcc_str);
    }
}
