#[macro_use] extern crate bencher;
extern crate sliding_puzzle;

use bencher::Bencher;
use sliding_puzzle::*;

mod new;
mod clone;
mod moves;
mod slide;
mod slide_mut;
mod slide_unchecked;
mod slide_mut_unchecked;
mod scramble;
mod from_decimal;
mod to_decimal;
mod from_decimal_unchecked;
mod to_decimal_unchecked;

benchmark_main!(
    new::benches,
    clone::benches,
    moves::benches,
    slide::benches,
    slide_mut::benches,
    slide_unchecked::benches,
    slide_mut_unchecked::benches,
    scramble::benches,
    from_decimal::benches,
    to_decimal::benches,
    from_decimal_unchecked::benches,
    to_decimal_unchecked::benches,
);
