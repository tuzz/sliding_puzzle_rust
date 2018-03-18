use super::*;

pub fn to_decimal_unchecked_on_a_fifteen_puzzle(bench: &mut Bencher) {
    let subject = SlidingPuzzle::new(&[
        &[ 0,  1,  2,  5],
        &[10, 15,  6,  4],
        &[ 7, 14, 11, 12],
        &[13,  3,  8,  9],
    ]).unwrap();

    bench.iter(|| {
        let clone = subject.clone();

        clone.to_decimal_unchecked()
    })
}


benchmark_group!(benches,
    to_decimal_unchecked_on_a_fifteen_puzzle,
);
