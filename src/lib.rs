#![deny(unused_variables, dead_code)]

#[macro_use]
extern crate json;

//
// Modules
//

mod cryptonight;
mod keccak;
mod miner;
mod oaes;
mod stratum;

//
// Public API
//

pub mod algorithm {
    pub use crate::cryptonight::cryptonight;
}

pub mod protocol {
    pub use crate::miner::{MinerConf, ValidatedMinerConf};
    pub use crate::stratum::{StratumClient, StratumJob, StratumResponse};
}

pub use crate::miner::{Miner, MinerConf};
