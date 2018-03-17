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

mod blank_is_on_the_left {
    use super::*;

    #[test]
    fn it_returns_the_expected_true_false_values() {
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
}


mod blank_is_on_the_right {
    use super::*;

    #[test]
    fn it_returns_the_expected_true_false_values() {
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
}

mod blank_is_on_the_top {
    use super::*;

    #[test]
    fn it_returns_the_expected_true_false_values() {
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
}

mod blank_is_on_the_bottom {
    use super::*;

    #[test]
    fn it_returns_the_expected_true_false_values() {
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
