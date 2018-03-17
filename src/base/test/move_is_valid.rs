use super::*;

#[test]
fn it_returns_whether_the_move_can_be_made_in_the_puzzle() {
    assert!(!subject().move_is_valid(&Direction::Left));
    assert!(subject().move_is_valid(&Direction::Right));
    assert!(subject().move_is_valid(&Direction::Up));
    assert!(!subject().move_is_valid(&Direction::Down));
}
