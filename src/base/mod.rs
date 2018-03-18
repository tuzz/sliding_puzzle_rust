use std::collections::HashSet;
use std::iter::FromIterator;

use lehmer::Lehmer;
use rand::{thread_rng,seq};

use direction::Direction;
use error::SlidingPuzzleError;
use result::Result;

#[derive(Debug, Clone, PartialEq)]
pub struct SlidingPuzzle<T> {
    tiles: Vec<T>,
    rows: usize,
    columns: usize,
    blank: usize,
}

impl<T: Clone + Default + PartialEq> SlidingPuzzle<T> {
    pub fn new(slice_2d: &[&[T]]) -> Result<Self> {
        Self::must_be_rectangular(slice_2d)?;
        Self::must_not_be_empty(slice_2d)?;

        let rows = Self::number_of_rows(slice_2d);
        let columns = Self::number_of_columns(slice_2d);
        let tiles = Self::flatten(slice_2d);

        Self::must_contain_one_blank(&tiles)?;
        let blank = Self::index_of_blank(&tiles);

        Ok(Self { tiles, rows, columns, blank })
    }

    pub fn tiles(&self) -> Vec<Vec<T>> {
        self.tiles
            .chunks(self.columns)
            .map(|c| c.iter().cloned().collect())
            .collect()
    }

    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        if !self.in_bounds(row, column) { return None }

        let index = row * self.columns + column;
        let tile = &self.tiles[index];

        Some(tile)
    }

    pub fn position(&self, tile: &T) -> Option<(usize, usize)> {
        let index = self.tiles.iter().position(|t| t == tile)?;

        let row = index / self.columns;
        let column = index % self.columns;

        Some((row, column))
    }

    pub fn moves(&self) -> Vec<Direction> {
        let mut vec = Vec::with_capacity(4);

        self.push_if_valid(&mut vec, Direction::Left);
        self.push_if_valid(&mut vec, Direction::Right);
        self.push_if_valid(&mut vec, Direction::Up);
        self.push_if_valid(&mut vec, Direction::Down);

        vec
    }

    pub fn slide(&self, direction: &Direction) -> Result<Self> {
        self.clone().slide_mut(direction).map(|s| s.to_owned())
    }

    pub fn slide_mut(&mut self, direction: &Direction) -> Result<(&mut Self)> {
        if self.move_is_valid(direction) {
            self.slide_mut_unchecked(direction);
            Ok(self)
        } else {
            Err(SlidingPuzzleError::new("move is invalid"))
        }
    }

    pub fn slide_unchecked(&self, direction: &Direction) -> Self {
        self.clone().slide_mut_unchecked(direction).to_owned()
    }

    pub fn slide_mut_unchecked(&mut self, direction: &Direction) -> &mut Self {
        let tile = self.index_of_tile_to_swap(direction);

        self.tiles.swap(self.blank, tile);
        self.blank = tile;
        self
    }

    pub fn scramble(&self, number_of_moves: usize) -> Self {
        let mut clone = self.clone();
        let mut previous = None;

        for _ in 0..number_of_moves {
            let direction = clone.random_move(previous);
            clone.slide_mut_unchecked(&direction);

            previous = Some(direction);
        }

        clone.to_owned()
    }

    pub fn in_bounds(&self, row: usize, column: usize) -> bool {
        row < self.rows && column < self.columns
    }

    pub fn move_is_valid(&self, direction: &Direction) -> bool {
        match *direction {
            Direction::Left => !self.blank_is_on_the_right(),
            Direction::Right => !self.blank_is_on_the_left(),
            Direction::Up => !self.blank_is_on_the_bottom(),
            Direction::Down => !self.blank_is_on_the_top(),
        }
    }

    pub fn blank_is_on_the_left(&self) -> bool {
        self.blank % self.columns == 0
    }

    pub fn blank_is_on_the_right(&self) -> bool {
        (self.blank + 1) % self.columns == 0
    }

    pub fn blank_is_on_the_top(&self) -> bool {
        self.blank < self.columns
    }

    pub fn blank_is_on_the_bottom(&self) -> bool {
        self.blank >= (self.tiles.len() - self.columns)
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

    fn index_of_blank(slice: &[T]) -> usize {
        slice.iter().position(Self::is_blank).expect(
            "invariant violated: puzzle must contain a single blank",
        )
    }

    fn index_of_tile_to_swap(&self, direction: &Direction) -> usize {
        let columns = self.columns as isize;
        let blank = self.blank as isize;

        (blank + direction.y() * columns + direction.x()) as usize
    }

    fn number_of_rows(slice_2d: &[&[T]]) -> usize {
        slice_2d.len()
    }

    fn number_of_columns(slice_2d: &[&[T]]) -> usize {
        slice_2d.first().expect(
            "invariant violated: puzzle must not be empty",
        ).len()
    }

    fn flatten(slice_2d: &[&[T]]) -> Vec<T> {
        slice_2d.iter().flat_map(|row| row.iter().cloned()).collect()
    }

    fn is_blank(tile: &T) -> bool {
        *tile == T::default()
    }

    fn push_if_valid(&self, vec: &mut Vec<Direction>, direction: Direction) {
        if self.move_is_valid(&direction) {
            vec.push(direction);
        }
    }

    fn random_move(&self, previous: Option<Direction>) -> Direction {
        let mut moves = self.moves();

        let opposite = previous.map(|d| d.opposite());
        Self::remove(&mut moves, opposite);

        let mut rng = thread_rng();
        let vec = seq::sample_slice(&mut rng, &moves, 1);

        vec.first().expect(
            "there should always be at least one available move",
        ).to_owned()
    }

    pub fn remove<U: PartialEq>(vec: &mut Vec<U>, item_option: Option<U>) -> Option<U> {
        let item = item_option?;
        let index = vec.iter().position(|x| *x == item)?;

        Some(vec.remove(index))
    }
}

impl SlidingPuzzle<u64> {
    pub fn to_decimal_unchecked(self) -> u64 {
        Lehmer::from_permutation(self.tiles).to_decimal()
    }
}

#[cfg(test)]
mod test;
