use super::*;

#[test]
fn it_slides_a_tile_in_the_direction_of_the_blank() {
    let mut subject = subject();
    subject.slide_mut(&Direction::Right).unwrap();

    assert_eq!(subject.tiles(), &[
        &[1, 0, 2],
        &[3, 4, 5],
        &[6, 7, 8],
    ]);
}

#[test]
fn it_errors_if_the_move_is_invalid() {
    let mut subject = subject();

    let result = subject.slide_mut(&Direction::Left);
    assert!(result.is_err());

    let error = result.unwrap_err();
    assert_eq!(error.description, "move is invalid");
}

#[test]
fn it_does_not_mutate_the_puzzle_if_the_move_is_invalid() {
    let mut mutable = subject();
    let _ = mutable.slide_mut(&Direction::Left);

    assert_eq!(mutable.tiles, subject().tiles);
}
