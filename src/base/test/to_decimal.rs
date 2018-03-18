use super::*;

#[test]
fn it_returns_the_factorial_base_number_in_general() {
    let subject = Subject::<u64>::new(&[
        &[1, 0, 2],
        &[3, 4, 5],
    ]).unwrap();

    assert_eq!(subject.to_decimal().unwrap(), 120);
}

#[test]
fn it_errors_if_the_puzzle_doesnt_contain_zero_through_n_minus_one() {
    let subject = Subject::<u64>::new(&[&[0, 2, 3]]).unwrap();

    let result = subject.to_decimal();
    assert!(result.is_err());

    let error = result.unwrap_err();
    assert_eq!(error.description, "puzzle must contain all numbers from 0 to n-1");
}
