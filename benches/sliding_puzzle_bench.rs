#[macro_use] extern crate bencher;
extern crate sliding_puzzle;

use bencher::Bencher;
use sliding_puzzle::SlidingPuzzle;

fn benchmark_hello(bench: &mut Bencher) {
    bench.iter(|| {
        SlidingPuzzle::hello();
    })
}

benchmark_group!(
    benches,
    benchmark_hello
);

benchmark_main!(benches);
