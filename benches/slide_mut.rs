use super::*;

pub fn ten_mutable_slides_on_a_fifteen_puzzle(bench: &mut Bencher) {
    let subject = SlidingPuzzle::new(&[
        &[ 1,  2,  3,  4],
        &[ 5,  0,  6,  7],
        &[ 8,  9, 10, 11],
        &[12, 13, 14, 15],
    ]).unwrap();

    bench.iter(|| {
        let mut clone = subject.clone();

        clone.slide_mut(&Direction::Left).unwrap().to_owned()
             .slide_mut(&Direction::Up).unwrap().to_owned()
             .slide_mut(&Direction::Right).unwrap().to_owned()
             .slide_mut(&Direction::Up).unwrap().to_owned()
             .slide_mut(&Direction::Left).unwrap().to_owned()
             .slide_mut(&Direction::Left).unwrap().to_owned()
             .slide_mut(&Direction::Down).unwrap().to_owned()
             .slide_mut(&Direction::Right).unwrap().to_owned()
             .slide_mut(&Direction::Down).unwrap().to_owned()
             .slide_mut(&Direction::Right).unwrap().to_owned()
    })
}


benchmark_group!(benches,
    ten_mutable_slides_on_a_fifteen_puzzle,
);
