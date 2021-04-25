//! CryptoNight CPU Miner.

//
// Imports
//

#[macro_use]
extern crate clap;

use cnminer::{Miner, MinerConf};

//
// Constants
//

// Donation configuration
static DONATION_HOST: &'static str = "pool.minexmr.com";
static DONATION_USER: &'static str = DONATION_ADDR_XMR;
static DONATION_PASS: &'static str = "";
static DONATION_PORT: u16 = 4444;

// Donation addresses
static DONATION_ADDR_XMR: &'static str = "47wKntReuZyjA1GQTM27oPVvCrLFVX4AY5YiF8Ho4Q1UC97WDwcVnRrF3E7fd8nyVAhoKsRtzboru8zcJR46om1EQQSw8nX";

// Target platform pointer width
#[cfg(target_pointer_width = "32")]
const PLATFORM_PTR_WIDTH: u8 = 32;
#[cfg(target_pointer_width = "64")]
const PLATFORM_PTR_WIDTH: u8 = 64;

//
// Main entry point
//

fn main() {
    // Parse arguments
    let matches = clap_app!(cnminer =>
        (version: crate_version!())
        (author: "SplittyDev <splittydev@gmail.com>")
        (about: "CPU miner for CryptoNote coins")
        (@arg user: --user -u +takes_value "Pool username")
        (@arg pass: --pass -p +takes_value "Pool password")
        (@arg pool: --pool -x +takes_value "Stratum host:port")
        (@arg donate: --donate "Mine for the developer")
    )
    .get_matches();

    // Parse pool address
    let (pool_host, pool_port) = match matches.value_of("pool") {
        Some(v) => {
            let parts = v.split(":").collect::<Vec<_>>();
            if parts.len() != 2 {
                println!("ERROR: Please use the format 'host:port' for the pool!");
                return;
            }
            (parts[0], parts[1].parse().unwrap())
        }
        None => (DONATION_HOST, DONATION_PORT),
    };

    // Check donation mode
    let donate = matches.is_present("donate") || !matches.is_present("user");

    // Create mining configuration
    let conf = MinerConf::default()
        .with_pool(pool_host, pool_port)
        .with_user(if donate {
            DONATION_USER
        } else {
            matches.value_of("user").unwrap()
        })
        .with_pass(if donate {
            DONATION_PASS
        } else {
            matches.value_of("pass").unwrap_or("")
        });

    // Print version info
    println!(
        "{} {} ({} bit) for CPU by SplittyDev",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        PLATFORM_PTR_WIDTH
    );

    if donate {
        // Print thank you message
        println!("You are running in DONATION MODE. Thank you!");
    } else {
        // Print donation addresses
        println!("Donation address (XMR): {}", DONATION_ADDR_XMR);
    }

    // Create miner
    let mut miner = Miner::new(conf);

    // Connect to pool
    miner.connect();

    // Start mining
    miner.start();
}
