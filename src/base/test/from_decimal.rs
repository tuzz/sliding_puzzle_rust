use super::*;

#[test]
fn it_constructs_a_puzzle_from_the_factorial_base_number_in_general() {
    let subject = Subject::from_decimal(120, 2, 3).unwrap();

    assert_eq!(subject.columns, 3);
    assert_eq!(subject.blank, 1);

    assert_eq!(subject.tiles(), &[
        &[1, 0, 2],
        &[3, 4, 5],
    ]);
}

#[test]
fn it_errors_if_rows_is_zero() {
    let subject = Subject::from_decimal(120, 0, 3);
    assert!(subject.is_err());

    let error = subject.unwrap_err();
    assert_eq!(error.description, "puzzle must contain at least one row and column");
}

#[test]
fn it_errors_if_columns_is_zero() {
    let subject = Subject::from_decimal(120, 3, 0);
    assert!(subject.is_err());

    let error = subject.unwrap_err();
    assert_eq!(error.description, "puzzle must contain at least one row and column");
}

#[test]
fn it_errors_if_n_is_too_large() {
    let too_large = Lehmer::max_value(2 * 3) + 1;
    let subject = Subject::from_decimal(too_large, 2, 3);
    assert!(subject.is_err());

    let error = subject.unwrap_err();
    let expected = "720 is greater than the maximum (719) for a 2 by 3 puzzle";
    assert_eq!(error.description, expected);
}

#[test]
fn it_errors_if_the_puzzle_contains_more_than_twenty_tiles() {
    let subject = Subject::from_decimal(0, 3, 7);
    assert!(subject.is_err());

    let error = subject.unwrap_err();
    assert_eq!(error.description, "21 is greater than the maximum (20) tiles");
}
