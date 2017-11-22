#![deny(unused_variables, dead_code)]

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

pub use keccak::keccak;
pub use cryptonight::cryptonight;