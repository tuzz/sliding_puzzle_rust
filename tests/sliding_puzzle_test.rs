extern crate sliding_puzzle;

use sliding_puzzle::SlidingPuzzle;
use sliding_puzzle::Direction;

#[test]
fn it_works_for_the_example_in_the_readme() {
    let mut puzzle = SlidingPuzzle::new(&[
        &[1, 2, 0],
        &[3, 4, 5],
        &[6, 7, 8],
    ]).unwrap();

    puzzle.slide_mut(&Direction::Right).unwrap();

    assert_eq!(puzzle.tiles(), &[
        &[1, 0, 2],
        &[3, 4, 5],
        &[6, 7, 8],
    ]);

    let top_left = puzzle.get(0, 0).unwrap();
    assert_eq!(top_left, &1);

    let position = puzzle.position(&1).unwrap();
    assert_eq!(position, (0, 0));

    assert_eq!(puzzle.moves(), &[
        Direction::Left,
        Direction::Right,
        Direction::Up,
    ]);

    assert!(puzzle.move_is_valid(&Direction::Up));

    puzzle.scramble(100);
}
