use super::*;

type Subject = SlidingPuzzle;

fn subject() -> Subject {
    Subject::new(vec![
        vec![1, 2, 0],
        vec![3, 4, 5],
        vec![6, 7, 8],
    ]).unwrap()
}

mod new {
    use super::*;

    #[test]
    fn it_is_initialized_with_a_2d_vector() {
        assert_eq!(subject().tiles, vec![1, 2, 0, 3, 4, 5, 6, 7, 8]);
        assert_eq!(subject().rows, 3);
        assert_eq!(subject().columns, 3);
    }

    #[test]
    fn it_initializes_correctly_when_theres_a_single_row() {
        let subject = Subject::new(vec![vec![1, 2, 0]]).unwrap();

        assert_eq!(subject.tiles, vec![1, 2, 0]);
        assert_eq!(subject.rows, 1);
        assert_eq!(subject.columns, 3);
    }

    #[test]
    fn it_errors_if_the_puzzle_has_no_rows() {
        let subject = Subject::new(vec![]);
        assert!(subject.is_err());

        let error = subject.unwrap_err();
        assert_eq!(error.description, "puzzle must not be empty");
    }

    #[test]
    fn it_errors_if_the_puzzle_has_no_columns() {
        let subject = Subject::new(vec![vec![]]);
        assert!(subject.is_err());

        let error = subject.unwrap_err();
        assert_eq!(error.description, "puzzle must not be empty");
    }

    #[test]
    fn it_errors_if_the_puzzle_isnt_rectangular() {
        let subject = Subject::new(vec![
            vec![0, 1],
            vec![2],
        ]);
        assert!(subject.is_err());

        let error = subject.unwrap_err();
        assert_eq!(error.description, "puzzle must be rectangular");
    }

    #[test]
    fn it_errors_if_the_puzzle_doesnt_contain_a_blank() {
        let subject = Subject::new(vec![vec![1, 2]]);
        assert!(subject.is_err());

        let error = subject.unwrap_err();
        assert_eq!(error.description, "puzzle must contain a single blank");
    }

    #[test]
    fn it_errors_if_the_puzzle_contains_more_than_one_blank() {
        let subject = Subject::new(vec![vec![0], vec![0]]);
        assert!(subject.is_err());

        let error = subject.unwrap_err();
        assert_eq!(error.description, "puzzle must contain a single blank");
    }
}
