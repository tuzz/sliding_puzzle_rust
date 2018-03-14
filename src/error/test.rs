use super::*;

type Subject = SlidingPuzzleError;

fn subject() -> Subject {
    Subject::new("description")
}

mod new {
    use super::*;

    #[test]
    fn it_is_initialized_with_a_description() {
        assert_eq!(subject().description, "description");
    }
}

mod fmt {
    use super::*;

    #[test]
    fn it_formats_the_error() {
        let formatted = format!("{}", subject());
        assert_eq!(formatted, "SlidingPuzzleError: description");
    }
}
