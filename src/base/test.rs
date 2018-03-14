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
    fn it_can_be_initialized_from_array_arguments() {
        assert_eq!(subject().tiles, vec![1, 2, 0, 3, 4, 5, 6, 7, 8]);
        assert_eq!(subject().rows, 3);
        assert_eq!(subject().columns, 3);
    }
}
