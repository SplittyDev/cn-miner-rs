extern crate cnminer;

fn main() {
    let mut output = [0u8; 32];
    ::cnminer::cryptonight(b"This is a test", &mut output[..]);
    output.iter().for_each(|b| print!("{:x}", b));
}