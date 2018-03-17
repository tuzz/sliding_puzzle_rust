use super::*;

mod x {
    use super::*;

    #[test]
    fn it_returns_the_x_offset_relative_to_the_blank_for_the_moving_tile() {
        assert_eq!(Direction::Left.x(), 1);
        assert_eq!(Direction::Right.x(), -1);
        assert_eq!(Direction::Up.x(), 0);
        assert_eq!(Direction::Down.x(), 0);
    }
}

mod y {
    use super::*;

    #[test]
    fn it_returns_the_y_offset_relative_to_the_blank_for_the_moving_tile() {
        assert_eq!(Direction::Left.y(), 0);
        assert_eq!(Direction::Right.y(), 0);
        assert_eq!(Direction::Up.y(), 1);
        assert_eq!(Direction::Down.y(), -1);
    }
}
