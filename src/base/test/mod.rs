use super::*;

type Subject<T> = SlidingPuzzle<T>;

fn subject() -> Subject<u8> {
    Subject::new(&[
        &[1, 2, 0],
        &[3, 4, 5],
        &[6, 7, 8],
    ]).unwrap()
}

mod new;
mod tiles;
mod slide_mut_unchecked;
mod slide_mut;
mod slide_unchecked;
mod slide;
mod move_is_valid;
mod blank_is_on_the_x;
