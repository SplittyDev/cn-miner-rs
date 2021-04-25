extern crate cnminer;

#[macro_use]
extern crate bencher;
use bencher::Bencher;

extern crate rand;
use rand::RngCore;

fn bench_cryptonight(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut input = [0u8; 128];
    bench.iter(|| {
        rng.fill_bytes(&mut input);
        let mut output = [0u8; 32];
        ::cnminer::algorithm::cryptonight(input.as_ref(), &mut output[..]);
    });
}

benchmark_group!(benches, bench_cryptonight);
benchmark_main!(benches);