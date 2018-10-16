//! # adf-rust
//!
//! `adf-rust` provides utility methods for reading and writing ADF-formatted files.

extern crate byteorder;
extern crate directories;
extern crate log;
extern crate toml;
#[macro_use]
extern crate serde;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub mod chunk;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
