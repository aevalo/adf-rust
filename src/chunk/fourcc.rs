use std::str;

#[derive(Debug)]
pub struct FourCC {
    value: [u8; 4]
}

impl FourCC {
	pub fn new(value: &str) -> FourCC {
		let value_bytes = value.get(0..4).unwrap().as_bytes();
	    FourCC { value: [
	    			value_bytes[0],
	    			value_bytes[1],
	    			value_bytes[2],
	    			value_bytes[3]] }
	}

    pub fn to_str(&self) -> &str {
        str::from_utf8(&self.value).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new() {
    	let fourcc = FourCC::new("ABCD");
    	assert_eq!(fourcc.value[0], b'A');
    	assert_eq!(fourcc.value[1], b'B');
    	assert_eq!(fourcc.value[2], b'C');
    	assert_eq!(fourcc.value[3], b'D');
    }
    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn create_new_too_short() {
    	let fourcc = FourCC::new("ABC");
    }
    #[test]
    fn should_return_fourcc_as_str() {
    	let fourcc = FourCC::new("ABCD");
    	assert_eq!("ABCD", fourcc.to_str());
    }
}
