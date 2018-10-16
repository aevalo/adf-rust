/// # chuck
///
/// `chunk` provides utility methods for managing data chunks.
use std::vec::Vec;
use std::str::{self, FromStr};


mod fourcc;
use self::fourcc::{FourCC, FourCCParseError};

#[derive(Debug)]
pub struct Chunk {
    id: fourcc::FourCC,
    content: Vec<u8>,
}

impl Chunk {

    /// Create new empty chunk.
    ///
    /// # Examples
    ///
    /// ```
    /// use adf::chunk::Chunk;
    ///
    /// let chunk = Chunk::new("CHNK");
    /// ```
    pub fn new(id: &str) -> Chunk {
        Chunk {
            id: FourCC::from_str(id).unwrap(),
            content: Vec::new(),
        }
    }

    /// Create new chunk with capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use adf::chunk::Chunk;
    ///
    /// let chunk = Chunk::with_capacity("CHNK", 15);
    /// ```
    pub fn with_capacity(id: &str, capacity: usize) -> Chunk {
        Chunk {
            id: fourcc::FourCC::from_str(id).unwrap(),
            content: Vec::with_capacity(capacity),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new() {
        let chunk = Chunk::new("TEST");
        //assert_eq!("TEST", chunk.id.to_str());
        assert_eq!(0, chunk.content.len());
        assert_eq!(0, chunk.content.capacity());
    }

    #[test]
    fn create_new_with_capacity() {
        let chunk = Chunk::with_capacity("TEST", 10);
        //assert_eq!("TEST", chunk.id.to_str());
        assert_eq!(0, chunk.content.len());
        assert_eq!(10, chunk.content.capacity());
    }
}
