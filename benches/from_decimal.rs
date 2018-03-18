use super::*;

pub fn from_decimal_on_a_fifteen_puzzle(bench: &mut Bencher) {
    bench.iter(|| {
        SlidingPuzzle::from_decimal(1_234_567_890, 4, 4).unwrap()
    })
}


benchmark_group!(benches,
    from_decimal_on_a_fifteen_puzzle,
);
