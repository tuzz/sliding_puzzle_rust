use super::*;

#[test]
fn it_slides_a_tile_in_the_direction_of_the_blank() {
    let subject = subject();
    let clone = subject.slide(&Direction::Right).unwrap();

    assert_eq!(clone.tiles(), &[
        &[1, 0, 2],
        &[3, 4, 5],
        &[6, 7, 8],
    ]);
}

#[test]
#[allow(unused_mut)]
fn it_does_not_mutate_the_subject() {
    let mut mutable = subject();
    let _ = mutable.slide_unchecked(&Direction::Right);

    assert_eq!(mutable, subject());
}

#[test]
fn it_errors_if_the_move_is_invalid() {
    let subject = subject();

    let result = subject.slide(&Direction::Left);
    assert!(result.is_err());

    let error = result.unwrap_err();
    assert_eq!(error.description, "move is invalid");
}
