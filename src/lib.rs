#![deny(unused_variables, dead_code)]

extern crate blake;

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