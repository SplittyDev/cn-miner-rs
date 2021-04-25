#![deny(unused_variables, dead_code)]

extern crate blake;
extern crate groestl;
extern crate jhffi;
extern crate skeinffi;
extern crate rayon;
#[macro_use]
extern crate json;

//
// Modules
//

mod keccak;
mod oaes;
mod cryptonight;
mod stratum;
mod miner;

//
// Public API
//

pub mod algorithm {
    pub use crate::cryptonight::cryptonight;
}

pub mod protocol {
    pub use crate::miner::{MinerConf, ValidatedMinerConf};
    pub use crate::stratum::{StratumClient, StratumResponse, StratumJob};
}

pub use crate::miner::{Miner, MinerConf};