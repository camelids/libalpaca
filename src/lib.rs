//! ALPaCA
//!
//! A library to implement the ALPaCA defense to Website Fingerprinting
//! attacks.
#![warn(missing_docs)]

extern crate rand;
extern crate select;
extern crate url;

pub mod pad;
pub mod objects;
pub mod parsing;
pub mod morphing;
pub mod distribution;
