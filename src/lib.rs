//! ALPaCA
//!
//! A library to implement the ALPaCA defense to Website Fingerprinting
//! attacks.
#![warn(missing_docs)]

extern crate rand;
extern crate pcg_rand;

pub mod pad;
pub mod types;
pub mod morphing;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
