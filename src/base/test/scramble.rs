use super::*;

#[test]
fn it_slides_in_a_random_direction_a_number_of_times() {
    let first = &[
        &[1, 0, 2],
        &[3, 4, 5],
        &[6, 7, 8],
    ];

    let second = &[
        &[1, 2, 5],
        &[3, 4, 0],
        &[6, 7, 8],
    ];

    let mut first_count = 0;
    let mut second_count = 0;

    for _ in 0..1000 {
        let scrambled = subject().scramble(1).tiles();

        if scrambled == first { first_count += 1 }
        if scrambled == second { second_count += 1 }
    }

    assert!(first_count >= 400 && first_count <= 600);
    assert_eq!(second_count, 1000 - first_count);
}

#[test]
fn it_does_not_immediately_double_back_on_itself() {
    let subject = subject();
    let mut double_backs = 0;

    for _ in 0..1000 {
        if subject.scramble(2) == subject { double_backs += 1 }
    }

    assert_eq!(double_backs, 0);
}
