use super::*;

#[test]
fn it_constructs_a_puzzle_with_sequential_tiles_for_zero() {
    let subject = Subject::from_decimal_unchecked(0, 2, 3);

    assert_eq!(subject.rows, 2);
    assert_eq!(subject.columns, 3);
    assert_eq!(subject.blank, 0);

    assert_eq!(subject.tiles(), &[
        &[0, 1, 2],
        &[3, 4, 5],
    ]);
}

#[test]
fn it_constructs_a_puzzle_from_the_factorial_base_number_in_general() {
    let subject = Subject::from_decimal_unchecked(120, 2, 3);

    assert_eq!(subject.rows, 2);
    assert_eq!(subject.columns, 3);
    assert_eq!(subject.blank, 1);

    assert_eq!(subject.tiles(), &[
        &[1, 0, 2],
        &[3, 4, 5],
    ]);
}
