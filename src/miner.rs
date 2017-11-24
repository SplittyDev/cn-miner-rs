//
// Imports
//

use std::sync::mpsc::{channel, Receiver};
use super::protocol::{StratumClient, StratumResponse};

//
// Structures
//

pub struct Pool {
    pub host: String,
    pub port: u16,
}

pub struct MinerConf {
    pub user: Option<String>,
    pub pass: Option<String>,
    pub pool: Option<Pool>,
}

#[derive(Clone)]
pub struct ValidatedMinerConf {
    pub user: String,
    pub pass: String,
    pub pool: String,
}

pub struct Miner {
    stratum: StratumClient,
    receiver: Receiver<StratumResponse>,
}

//
// Implementations
//

impl Miner {

    /// Constructs a new `Miner`.
    pub fn new(conf: MinerConf) -> Miner {
        let conf = conf.validate();
        let (tx, rx) = channel();
        let handlers = vec![tx];
        Miner {
            stratum: StratumClient::new(conf, handlers),
            receiver: rx,
        }
    }

    pub fn connect(&mut self) {
        self.stratum.connect();
        self.stratum.login();
    }

    pub fn start(&mut self) {
        loop {
            match self.receiver.recv() {
                Ok(StratumResponse::Login(miner_id, job)) => {
                    println!("Received miner id: {}", miner_id);
                    println!("Received job with target {}", job.target);
                },
                Ok(_) => println!("Invalid Stratum response!"),
                Err(_) => println!("Unable to receive Stratum response!"),
            };
        }
    }

    pub fn join(self) {
        self.stratum.join();
    }
}

impl MinerConf {
    pub fn with_pool<T: Into<String>>(mut self, host: T, port: u16) -> MinerConf {
        self.pool = Some(Pool { host: host.into(), port: port });
        self
    }
    pub fn with_user<T: Into<String>>(mut self, user: T) -> MinerConf {
        self.user = Some(user.into());
        self
    }
    pub fn with_pass<T: Into<String>>(mut self, pass: T) -> MinerConf {
        self.pass = Some(pass.into());
        self
    }
    fn validate(self) -> ValidatedMinerConf {
        let pool = self.pool.unwrap();
        ValidatedMinerConf {
            user: self.user.unwrap(),
            pass: self.pass.unwrap(),
            pool: format!("{}:{}", pool.host, pool.port),
        }
    }
}

//
// Trait implementations
//

impl Default for MinerConf {
    fn default() -> MinerConf {
        MinerConf {
            pool: None,
            user: None,
            pass: None,
        }
    }
}