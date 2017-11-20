#![deny(unused_variables, dead_code)]

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