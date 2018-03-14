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
    fn it_errors_when_there_are_no_rows() {
        let subject = Subject::new(vec![]);
        assert!(subject.is_err());

        let error = subject.unwrap_err();
        assert_eq!(error.description, "vector contains no rows");
    }
}
