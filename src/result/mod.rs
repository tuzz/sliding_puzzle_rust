use std::result;
use error::SlidingPuzzleError;

pub type Result<T> = result::Result<T, SlidingPuzzleError>;
