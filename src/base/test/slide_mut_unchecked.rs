use super::*;

#[test]
fn it_slides_a_tile_in_the_direction_of_the_blank() {
    let mut subject = subject();

    subject.slide_mut_unchecked(&Direction::Right);
    assert_eq!(subject.tiles(), &[
        &[1, 0, 2],
        &[3, 4, 5],
        &[6, 7, 8],
    ]);

    subject.slide_mut_unchecked(&Direction::Up);
    assert_eq!(subject.tiles(), &[
        &[1, 4, 2],
        &[3, 0, 5],
        &[6, 7, 8],
    ]);
}

#[test]
#[should_panic]
fn it_might_panic_if_the_move_is_invalid() {
    let mut subject = subject();
    subject.slide_mut_unchecked(&Direction::Down);
}

#[test]
fn it_might_not_panic_if_the_move_is_invalid() {
    let mut subject = Subject::new(&[&[1, 0], &[2, 3]]).unwrap();
    subject.slide_mut_unchecked(&Direction::Left);
}

#[test]
fn it_can_be_chained_for_convenience() {
    let mut subject = subject();

    subject.slide_mut_unchecked(&Direction::Right)
           .slide_mut_unchecked(&Direction::Up);

    assert_eq!(subject.tiles(), &[
        &[1, 4, 2],
        &[3, 0, 5],
        &[6, 7, 8],
    ]);
}
