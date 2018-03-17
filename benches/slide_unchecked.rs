use super::*;

pub fn ten_unchecked_slides_on_a_fifteen_puzzle(bench: &mut Bencher) {
    let subject = SlidingPuzzle::new(&[
        &[ 1,  2,  3,  4],
        &[ 5,  0,  6,  7],
        &[ 8,  9, 10, 11],
        &[12, 13, 14, 15],
    ]).unwrap();

    bench.iter(|| {
        let clone = subject.clone(); // for consistency with mut benchmarks

        clone.slide_unchecked(&Direction::Left)
             .slide_unchecked(&Direction::Up)
             .slide_unchecked(&Direction::Right)
             .slide_unchecked(&Direction::Up)
             .slide_unchecked(&Direction::Left)
             .slide_unchecked(&Direction::Left)
             .slide_unchecked(&Direction::Down)
             .slide_unchecked(&Direction::Right)
             .slide_unchecked(&Direction::Down)
             .slide_unchecked(&Direction::Right)
    })
}


benchmark_group!(benches,
    ten_unchecked_slides_on_a_fifteen_puzzle,
);
