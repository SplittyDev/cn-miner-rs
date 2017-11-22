extern crate cnminer;

macro_rules! hex_print {
    ($o:expr) => {
        $o.iter().for_each(|b| print!("{:x}", b));
        println!();
    }
}

fn main() {
    let keccak_in = b"Hello world!";
    let mut keccak_out = [0u8; 32];
    ::cnminer::keccak(keccak_in, &mut keccak_out);
    hex_print!(keccak_out);

    let mut output = [0u8; 32];
    ::cnminer::cryptonight(b"\0", &mut output[..]);
    hex_print!(output);
}