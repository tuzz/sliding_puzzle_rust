use super::*;

#[test]
fn it_reconstructs_the_2d_representation_of_the_puzzle() {
    assert_eq!(subject().tiles(), &[
        &[1, 2, 0],
        &[3, 4, 5],
        &[6, 7, 8],
    ]);
}
