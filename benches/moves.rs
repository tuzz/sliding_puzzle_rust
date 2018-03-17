use super::*;

pub fn moves_for_a_fifteen_puzzle(bench: &mut Bencher) {
    let subject = SlidingPuzzle::new(&[
        &[ 1,  2,  3,  4],
        &[ 5,  0,  6,  7],
        &[ 8,  9, 10, 11],
        &[12, 13, 14, 15],
    ]).unwrap();

    bench.iter(|| {
        subject.moves()
    })
}


benchmark_group!(benches,
    moves_for_a_fifteen_puzzle,
);
