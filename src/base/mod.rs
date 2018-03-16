use std::collections::HashSet;
use std::iter::FromIterator;

use result::Result;
use error::SlidingPuzzleError;

#[derive(Debug)]
pub struct SlidingPuzzle<T> {
    tiles: Vec<T>,
    rows: usize,
    columns: usize,
}

impl<T: Copy + Default + PartialEq> SlidingPuzzle<T> {
    pub fn new(slice_2d: &[&[T]]) -> Result<Self> {
        Self::must_be_rectangular(slice_2d)?;
        Self::must_not_be_empty(slice_2d)?;

        let rows = Self::number_of_rows(slice_2d);
        let columns = Self::number_of_columns(slice_2d);
        let tiles = Self::flatten(slice_2d);

        Self::must_contain_one_blank(&tiles)?;

        Ok(Self { tiles, rows, columns })
    }

    fn must_be_rectangular(slice_2d: &[&[T]]) -> Result<()> {
        let lengths = slice_2d.iter().map(|row| row.len());
        let uniques = HashSet::<usize>::from_iter(lengths);

        if uniques.len() > 1 {
            Err(SlidingPuzzleError::new("puzzle must be rectangular"))
        } else {
            Ok(())
        }
    }

    fn must_not_be_empty(slice_2d: &[&[T]]) -> Result<()> {
        let no_rows = slice_2d.len() == 0;
        let no_columns = slice_2d.iter().any(|row| row.len() == 0);

        if no_rows || no_columns {
            Err(SlidingPuzzleError::new("puzzle must not be empty"))
        } else {
            Ok(())
        }
    }

    fn must_contain_one_blank(slice: &[T]) -> Result<()> {
        let blanks = slice.iter().filter(Self::is_blank).count();

        if blanks != 1 {
            Err(SlidingPuzzleError::new("puzzle must contain a single blank"))
        } else {
            Ok(())
        }
    }

    fn number_of_rows(slice_2d: &[&[T]]) -> usize {
        slice_2d.len()
    }

    fn number_of_columns(slice_2d: &[&[T]]) -> usize {
        slice_2d.first().unwrap().len()
    }

    fn flatten(slice_2d: &[&[T]]) -> Vec<T> {
        slice_2d.iter().flat_map(|row| row.iter().cloned()).collect()
    }

    fn is_blank(tile: &&T) -> bool {
        **tile == T::default()
    }
}

#[cfg(test)]
mod test;
