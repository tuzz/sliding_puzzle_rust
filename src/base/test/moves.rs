use super::*;


#[test]
fn it_returns_two_directions_when_the_blank_is_in_a_corner() {
    let subject = Subject::new(&[
        &[0, 1, 2],
        &[3, 4, 5],
        &[6, 7, 8],
    ]).unwrap();

    assert_eq!(subject.moves(), &[
        Direction::Left,
        Direction::Up,
    ]);
}

#[test]
fn it_returns_three_directions_when_the_blank_is_at_an_edge() {
    let subject = Subject::new(&[
        &[1, 0, 2],
        &[3, 4, 5],
        &[6, 7, 8],
    ]).unwrap();

    assert_eq!(subject.moves(), &[
        Direction::Left,
        Direction::Right,
        Direction::Up,
    ]);
}

#[test]
fn it_returns_four_directions_when_the_blank_is_in_the_middle() {
    let subject = Subject::new(&[
        &[1, 2, 3],
        &[4, 0, 5],
        &[6, 7, 8],
    ]).unwrap();

    assert_eq!(subject.moves(), &[
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ]);
}
