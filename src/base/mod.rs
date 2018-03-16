use std::collections::HashSet;
use std::iter::FromIterator;

use result::Result;
use error::SlidingPuzzleError;

#[derive(Debug)]
pub struct SlidingPuzzle {
    tiles: Vec<u8>,
    rows: usize,
    columns: usize,
}

impl SlidingPuzzle {
    pub fn new(vec_2d: &Vec<Vec<u8>>) -> Result<Self> {
        Self::must_be_rectangular(vec_2d)?;
        Self::must_not_be_empty(vec_2d)?;

        let rows = Self::number_of_rows(vec_2d);
        let columns = Self::number_of_columns(vec_2d);
        let tiles = Self::flatten(vec_2d);

        Self::must_contain_one_blank(&tiles)?;

        Ok(Self { tiles, rows, columns })
    }

    fn must_be_rectangular<T>(vec_2d: &Vec<Vec<T>>) -> Result<()> {
        let lengths = vec_2d.iter().map(|row| row.len());
        let uniques = HashSet::<usize>::from_iter(lengths);

        if uniques.len() > 1 {
            Err(SlidingPuzzleError::new("puzzle must be rectangular"))
        } else {
            Ok(())
        }
    }

    fn must_not_be_empty<T>(vec_2d: &Vec<Vec<T>>) -> Result<()> {
        let no_rows = vec_2d.len() == 0;
        let no_columns = vec_2d.iter().any(|row| row.len() == 0);

        if no_rows || no_columns {
            Err(SlidingPuzzleError::new("puzzle must not be empty"))
        } else {
            Ok(())
        }
    }

    fn must_contain_one_blank(vec: &Vec<u8>) -> Result<()> {
        let blanks = vec.iter().filter(|&&e| e == 0).count();

        if blanks != 1 {
            Err(SlidingPuzzleError::new("puzzle must contain a single blank"))
        } else {
            Ok(())
        }
    }

    fn number_of_rows<T>(vec_2d: &Vec<Vec<T>>) -> usize {
        vec_2d.len()
    }

    fn number_of_columns<T>(vec_2d: &Vec<Vec<T>>) -> usize {
        vec_2d.first().unwrap().len()
    }

    fn flatten<T: Copy>(vec_2d: &Vec<Vec<T>>) -> Vec<T> {
        vec_2d.iter().flat_map(|row| row.iter().cloned()).collect()
    }
}

#[cfg(test)]
mod test;
