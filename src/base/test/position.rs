use super::*;

#[test]
fn it_finds_the_x_y_position_of_the_tile() {
    assert_eq!(subject().position(&0).unwrap(), (0, 2));
    assert_eq!(subject().position(&1).unwrap(), (0, 0));
    assert_eq!(subject().position(&2).unwrap(), (0, 1));
    assert_eq!(subject().position(&3).unwrap(), (1, 0));
    assert_eq!(subject().position(&4).unwrap(), (1, 1));
    assert_eq!(subject().position(&5).unwrap(), (1, 2));
    assert_eq!(subject().position(&6).unwrap(), (2, 0));
    assert_eq!(subject().position(&7).unwrap(), (2, 1));
    assert_eq!(subject().position(&8).unwrap(), (2, 2));
}

#[test]
fn it_returns_none_if_the_tile_cant_be_found() {
    assert!(subject().position(&9).is_none());
    assert!(subject().position(&10).is_none());
    assert!(subject().position(&11).is_none());
}
