#![deny(unused_variables, dead_code)]

#![feature(untagged_unions)]

extern crate blake;
extern crate groestl;
extern crate jhffi;
extern crate skeinffi;

//
// Modules
//

mod keccak;
mod oaes;
pub mod cryptonight;

//
// Public API
//

pub use cryptonight::cryptonight;