use std::fmt;

#[derive(Debug)]
pub struct SlidingPuzzleError {
    pub description: String,
}

impl SlidingPuzzleError {
    pub fn new(s: &str) -> Self {
        Self { description: s.to_string() }
    }
}

impl fmt::Display for SlidingPuzzleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SlidingPuzzleError: {}", self.description)
    }
}

#[cfg(test)]
mod test;
