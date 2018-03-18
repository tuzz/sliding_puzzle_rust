use super::*;

pub fn to_decimal_on_a_fifteen_puzzle(bench: &mut Bencher) {
    let subject = SlidingPuzzle::<u64>::new(&[
        &[ 0,  1,  2,  5],
        &[10, 15,  6,  4],
        &[ 7, 14, 11, 12],
        &[13,  3,  8,  9],
    ]).unwrap();

    bench.iter(|| {
        let clone = subject.clone();

        clone.to_decimal().unwrap()
    })
}


benchmark_group!(benches,
    to_decimal_on_a_fifteen_puzzle,
);
