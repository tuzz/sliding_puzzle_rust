use super::*;

pub fn ten_slides_on_a_fifteen_puzzle(bench: &mut Bencher) {
    let subject = SlidingPuzzle::new(&[
        &[ 1,  2,  3,  4],
        &[ 5,  0,  6,  7],
        &[ 8,  9, 10, 11],
        &[12, 13, 14, 15],
    ]).unwrap();

    bench.iter(|| {
        let clone = subject.clone(); // for consistency with mut benchmarks

        clone.slide(&Direction::Left).unwrap()
             .slide(&Direction::Up).unwrap()
             .slide(&Direction::Right).unwrap()
             .slide(&Direction::Up).unwrap()
             .slide(&Direction::Left).unwrap()
             .slide(&Direction::Left).unwrap()
             .slide(&Direction::Down).unwrap()
             .slide(&Direction::Right).unwrap()
             .slide(&Direction::Down).unwrap()
             .slide(&Direction::Right).unwrap()
    })
}


benchmark_group!(benches,
    ten_slides_on_a_fifteen_puzzle,
);
