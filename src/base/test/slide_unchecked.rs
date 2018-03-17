use super::*;

#[test]
fn it_slides_a_tile_in_the_direction_of_the_blank() {
    let subject = subject();
    let clone = subject.slide_unchecked(&Direction::Right);

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
#[should_panic]
fn it_might_panic_if_the_move_is_invalid() {
    let subject = subject();
    subject.slide_unchecked(&Direction::Down);
}

#[test]
fn it_might_not_panic_if_the_move_is_invalid() {
    let subject = Subject::new(&[&[1, 0], &[2, 3]]).unwrap();
    subject.slide_unchecked(&Direction::Left);
}

#[test]
fn it_can_be_chained_for_convenience() {
    let subject = subject();

    let clone = subject.slide_unchecked(&Direction::Right)
                       .slide_unchecked(&Direction::Up);

    assert_eq!(clone.tiles(), &[
        &[1, 4, 2],
        &[3, 0, 5],
        &[6, 7, 8],
    ]);
}
