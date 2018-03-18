use super::*;

#[test]
fn it_returns_zero_if_the_tiles_are_in_sequence() {
    let subject = Subject::<u64>::new(&[
        &[0, 1, 2],
        &[3, 4, 5],
    ]).unwrap();

    assert_eq!(subject.to_decimal_unchecked(), 0);
}

#[test]
fn it_returns_the_factorial_base_number_in_general() {
    let subject = Subject::<u64>::new(&[
        &[1, 0, 2],
        &[3, 4, 5],
    ]).unwrap();

    assert_eq!(subject.to_decimal_unchecked(), 120);
}
