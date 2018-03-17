#[macro_use] extern crate bencher;
extern crate sliding_puzzle;

use bencher::Bencher;
use sliding_puzzle::SlidingPuzzle;

mod new;

benchmark_main!(
    new::benches,
);
