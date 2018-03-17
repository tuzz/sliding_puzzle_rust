use super::*;

pub fn ten_mutable_unchecked_slides_on_a_fifteen_puzzle(bench: &mut Bencher) {
    let subject = SlidingPuzzle::new(&[
        &[ 1,  2,  3,  4],
        &[ 5,  0,  6,  7],
        &[ 8,  9, 10, 11],
        &[12, 13, 14, 15],
    ]).unwrap();

    bench.iter(|| {
        let mut clone = subject.clone(); // for consistency with mut benchmarks

        clone.slide_mut_unchecked(&Direction::Left).to_owned()
             .slide_mut_unchecked(&Direction::Up).to_owned()
             .slide_mut_unchecked(&Direction::Right).to_owned()
             .slide_mut_unchecked(&Direction::Up).to_owned()
             .slide_mut_unchecked(&Direction::Left).to_owned()
             .slide_mut_unchecked(&Direction::Left).to_owned()
             .slide_mut_unchecked(&Direction::Down).to_owned()
             .slide_mut_unchecked(&Direction::Right).to_owned()
             .slide_mut_unchecked(&Direction::Down).to_owned()
             .slide_mut_unchecked(&Direction::Right).to_owned()
    })
}


benchmark_group!(benches,
    ten_mutable_unchecked_slides_on_a_fifteen_puzzle,
);
