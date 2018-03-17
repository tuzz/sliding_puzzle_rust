use std::collections::HashSet;
use std::iter::FromIterator;

use direction::Direction;
use error::SlidingPuzzleError;
use result::Result;

#[derive(Debug)]
pub struct SlidingPuzzle<T> {
    tiles: Vec<T>,
    rows: usize,
    columns: usize,
}

impl<T: Clone + Default + PartialEq> SlidingPuzzle<T> {
    pub fn new(slice_2d: &[&[T]]) -> Result<Self> {
        Self::must_be_rectangular(slice_2d)?;
        Self::must_not_be_empty(slice_2d)?;

        let rows = Self::number_of_rows(slice_2d);
        let columns = Self::number_of_columns(slice_2d);
        let tiles = Self::flatten(slice_2d);

        Self::must_contain_one_blank(&tiles)?;

        Ok(Self { tiles, rows, columns })
    }

    pub fn tiles(&self) -> Vec<Vec<T>> {
        self.tiles
            .chunks(self.columns)
            .map(|c| c.iter().cloned().collect())
            .collect()
    }

    pub fn slide_mut_unchecked(&mut self, direction: Direction) {
        let blank = self.tiles.iter().position(Self::is_blank).unwrap();

        let tile = blank as isize +
                   direction.y() * self.columns as isize +
                   direction.x();

        self.tiles.swap(blank, tile as usize)
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
        let blanks = slice.iter().filter(|t| Self::is_blank(t)).count();

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

    fn is_blank(tile: &T) -> bool {
        *tile == T::default()
    }
}

#[cfg(test)]
mod test;
