use super::*;

#[test]
fn it_gets_the_tile_at_x_y_position() {
    assert_eq!(subject().get(0, 0).unwrap(), &1);
    assert_eq!(subject().get(1, 1).unwrap(), &4);
    assert_eq!(subject().get(2, 0).unwrap(), &6);
}

#[test]
fn it_returns_none_if_out_of_bounds() {
    assert!(subject().get(0, 3).is_none());
    assert!(subject().get(3, 0).is_none());
    assert!(subject().get(3, 3).is_none());
}
