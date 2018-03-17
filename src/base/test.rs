use super::*;

type Subject<T> = SlidingPuzzle<T>;

fn subject() -> Subject<u8> {
    Subject::new(&[
        &[1, 2, 0],
        &[3, 4, 5],
        &[6, 7, 8],
    ]).unwrap()
}

mod new {
    use super::*;

    #[test]
    fn it_is_initialized_with_a_2d_slice() {
        assert_eq!(subject().tiles, vec![1, 2, 0, 3, 4, 5, 6, 7, 8]);
        assert_eq!(subject().rows, 3);
        assert_eq!(subject().columns, 3);
    }

    #[test]
    fn it_initializes_correctly_when_theres_a_single_row() {
        let subject = Subject::new(&[&[1, 2, 0]]).unwrap();

        assert_eq!(subject.tiles, vec![1, 2, 0]);
        assert_eq!(subject.rows, 1);
        assert_eq!(subject.columns, 3);
    }

    #[test]
    fn it_can_be_used_in_a_generic_way() {
        let subject = Subject::new(&[&["a", "b"], &["c", ""]]).unwrap();
        assert_eq!(subject.tiles, vec!["a", "b", "c", ""])
    }

    #[test]
    fn it_errors_if_the_puzzle_has_no_rows() {
        let subject = Subject::<u8>::new(&[]);
        assert!(subject.is_err());

        let error = subject.unwrap_err();
        assert_eq!(error.description, "puzzle must not be empty");
    }

    #[test]
    fn it_errors_if_the_puzzle_has_no_columns() {
        let subject = Subject::<u8>::new(&[&[]]);
        assert!(subject.is_err());

        let error = subject.unwrap_err();
        assert_eq!(error.description, "puzzle must not be empty");
    }

    #[test]
    fn it_errors_if_the_puzzle_isnt_rectangular() {
        let subject = Subject::new(&[&[0, 1], &[2]]);
        assert!(subject.is_err());

        let error = subject.unwrap_err();
        assert_eq!(error.description, "puzzle must be rectangular");
    }

    #[test]
    fn it_errors_if_the_puzzle_doesnt_contain_a_blank() {
        let subject = Subject::new(&[&[1, 2]]);
        assert!(subject.is_err());

        let error = subject.unwrap_err();
        assert_eq!(error.description, "puzzle must contain a single blank");
    }

    #[test]
    fn it_errors_if_the_puzzle_contains_more_than_one_blank() {
        let subject = Subject::new(&[&[0], &[0]]);
        assert!(subject.is_err());

        let error = subject.unwrap_err();
        assert_eq!(error.description, "puzzle must contain a single blank");
    }
}

mod tiles {
    use super::*;

    #[test]
    fn it_reconstructs_the_2d_representation_of_the_puzzle() {
        assert_eq!(subject().tiles(), &[
            &[1, 2, 0],
            &[3, 4, 5],
            &[6, 7, 8],
        ]);
    }
}

mod slide_mut_unchecked {
    use super::*;

    #[test]
    fn it_slides_a_tile_in_the_direction_of_the_blank() {
        let mut subject = subject();

        subject.slide_mut_unchecked(Direction::Right);
        assert_eq!(subject.tiles(), &[
            &[1, 0, 2],
            &[3, 4, 5],
            &[6, 7, 8],
        ]);

        subject.slide_mut_unchecked(Direction::Up);
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
        subject.slide_mut_unchecked(Direction::Down);
    }

    #[test]
    fn it_might_not_panic_if_the_move_is_invalid() {
        let mut subject = Subject::new(&[&[1, 0], &[2, 3]]).unwrap();
        subject.slide_mut_unchecked(Direction::Left);
    }
}

mod blank_is_on_the_x {
    use super::*;

    const BLANK_IS_TOP_LEFT: &[&[u8]] = &[&[0, 1], &[2, 3]];
    const BLANK_IS_TOP_MIDDLE: &[&[u8]] = &[&[1, 0, 2], &[3, 4, 5]];
    const BLANK_IS_TOP_RIGHT: &[&[u8]] = &[&[1, 0], &[3, 4]];
    const BLANK_IS_MIDDLE_LEFT: &[&[u8]] = &[&[1, 2], &[0, 3], &[4, 5]];
    const BLANK_IS_MIDDLE: &[&[u8]] = &[&[1, 2, 3], &[4, 0, 5], &[6, 7, 8]];
    const BLANK_IS_MIDDLE_RIGHT: &[&[u8]] = &[&[1, 2], &[3, 0], &[4, 5]];
    const BLANK_IS_BOTTOM_LEFT: &[&[u8]] = &[&[1, 2], &[0, 3]];
    const BLANK_IS_BOTTOM_MIDDLE: &[&[u8]] = &[&[1, 2, 3], &[4, 0, 5]];
    const BLANK_IS_BOTTOM_RIGHT: &[&[u8]] = &[&[1, 2], &[3, 0]];

    #[test]
    fn blank_is_on_the_left_test() {
        let result = |s| Subject::new(s).unwrap().blank_is_on_the_left();

        assert!(result(BLANK_IS_TOP_LEFT));
        assert!(!result(BLANK_IS_TOP_MIDDLE));
        assert!(!result(BLANK_IS_TOP_RIGHT));
        assert!(result(BLANK_IS_MIDDLE_LEFT));
        assert!(!result(BLANK_IS_MIDDLE));
        assert!(!result(BLANK_IS_MIDDLE_RIGHT));
        assert!(result(BLANK_IS_BOTTOM_LEFT));
        assert!(!result(BLANK_IS_BOTTOM_MIDDLE));
        assert!(!result(BLANK_IS_BOTTOM_RIGHT));
    }

    #[test]
    fn blank_is_on_the_right_test() {
        let result = |s| Subject::new(s).unwrap().blank_is_on_the_right();

        assert!(!result(BLANK_IS_TOP_LEFT));
        assert!(!result(BLANK_IS_TOP_MIDDLE));
        assert!(result(BLANK_IS_TOP_RIGHT));
        assert!(!result(BLANK_IS_MIDDLE_LEFT));
        assert!(!result(BLANK_IS_MIDDLE));
        assert!(result(BLANK_IS_MIDDLE_RIGHT));
        assert!(!result(BLANK_IS_BOTTOM_LEFT));
        assert!(!result(BLANK_IS_BOTTOM_MIDDLE));
        assert!(result(BLANK_IS_BOTTOM_RIGHT));
    }

    #[test]
    fn blank_is_on_the_top_test() {
        let result = |s| Subject::new(s).unwrap().blank_is_on_the_top();

        assert!(result(BLANK_IS_TOP_LEFT));
        assert!(result(BLANK_IS_TOP_MIDDLE));
        assert!(result(BLANK_IS_TOP_RIGHT));
        assert!(!result(BLANK_IS_MIDDLE_LEFT));
        assert!(!result(BLANK_IS_MIDDLE));
        assert!(!result(BLANK_IS_MIDDLE_RIGHT));
        assert!(!result(BLANK_IS_BOTTOM_LEFT));
        assert!(!result(BLANK_IS_BOTTOM_MIDDLE));
        assert!(!result(BLANK_IS_BOTTOM_RIGHT));
    }

    #[test]
    fn blank_is_on_the_bottom_test() {
        let result = |s| Subject::new(s).unwrap().blank_is_on_the_bottom();

        assert!(!result(BLANK_IS_TOP_LEFT));
        assert!(!result(BLANK_IS_TOP_MIDDLE));
        assert!(!result(BLANK_IS_TOP_RIGHT));
        assert!(!result(BLANK_IS_MIDDLE_LEFT));
        assert!(!result(BLANK_IS_MIDDLE));
        assert!(!result(BLANK_IS_MIDDLE_RIGHT));
        assert!(result(BLANK_IS_BOTTOM_LEFT));
        assert!(result(BLANK_IS_BOTTOM_MIDDLE));
        assert!(result(BLANK_IS_BOTTOM_RIGHT));
    }
}
