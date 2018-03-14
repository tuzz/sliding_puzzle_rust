use result::Result;
use error::SlidingPuzzleError;

pub struct SlidingPuzzle {
    tiles: Vec<u8>,
    rows: usize,
    columns: usize,
}

impl SlidingPuzzle {
    pub fn new(vec_2d: Vec<Vec<u8>>) -> Result<Self> {
        let rows = Self::number_of_rows(&vec_2d);
        let columns = Self::number_of_columns(&vec_2d)?;
        let tiles = Self::flatten(vec_2d);

        Ok(Self { tiles, rows, columns })
    }

    fn number_of_rows<T>(vec_2d: &Vec<Vec<T>>) -> usize {
        vec_2d.len()
    }

    fn number_of_columns<T>(vec_2d: &Vec<Vec<T>>) -> Result<usize> {
        vec_2d.first().map(|row| row.len()).ok_or(
            SlidingPuzzleError {}
        )
    }

    fn flatten<T>(vec_2d: Vec<Vec<T>>) -> Vec<T> {
        let mut vec = Vec::new();

        for row in vec_2d {
            for element in row {
                vec.push(element);
            }
        }

        vec
    }
}

#[cfg(test)]
mod test;
