use super::*;

#[test]
fn it_returns_true_if_the_x_y_position_is_in_bounds() {
    let subject = Subject::new(&[
        &[0, 1, 2],
        &[3, 4, 5]
    ]).unwrap();

    assert!(subject.in_bounds(0, 0));
    assert!(subject.in_bounds(0, 1));
    assert!(subject.in_bounds(0, 2));
    assert!(!subject.in_bounds(0, 3));

    assert!(subject.in_bounds(1, 0));
    assert!(subject.in_bounds(1, 1));
    assert!(subject.in_bounds(1, 2));
    assert!(!subject.in_bounds(1, 3));

    assert!(!subject.in_bounds(2, 0));
}
