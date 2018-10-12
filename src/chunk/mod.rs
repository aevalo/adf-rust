mod fourcc;

use std::vec::Vec;

#[derive(Debug)]
pub struct Chunk {
	id: fourcc::FourCC,
    content: Vec<u8>
}

impl Chunk {
	pub fn new(id: &str) -> Chunk {
	    Chunk { id: fourcc::FourCC::new(id), content: Vec::new() }
	}

	pub fn with_capacity(id: &str, capacity: usize) -> Chunk {
	    Chunk { id: fourcc::FourCC::new(id), content: Vec::with_capacity(capacity) }
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new() {
		let chunk = Chunk::new("TEST");
    	assert_eq!("TEST", chunk.id.to_str());
    	assert_eq!(0, chunk.content.len());
    	assert_eq!(0, chunk.content.capacity());
    }

    #[test]
    fn create_new_with_capacity() {
		let chunk = Chunk::with_capacity("TEST", 10);
    	assert_eq!("TEST", chunk.id.to_str());
    	assert_eq!(0, chunk.content.len());
    	assert_eq!(10, chunk.content.capacity());
    }
}
