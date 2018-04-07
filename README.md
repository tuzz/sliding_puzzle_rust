## Sliding Puzzle

![Travis](https://travis-ci.org/tuzz/sliding_puzzle.svg?branch=master)

A Rust crate for manipulating sliding tile puzzles.

## Overview

This crate reimplements much of the functionality from
[this Ruby gem](https://github.com/tuzz/sliding_puzzle_ruby) in an effort to learn
Rust and provide a faster implementation.

I intend to use this crate to explore different algorithms for 'delayed
duplicate detection', e.g.
[Structured Duplicate Detection in External-Memory Graph Search](https://www.aaai.org/Papers/AAAI/2004/AAAI04-108.pdf).

## Usage

Refer to the [Ruby README.md](https://github.com/tuzz/sliding_puzzle_ruby) for
more detailed documentation.

```rust
let mut puzzle = SlidingPuzzle::new(&[
    &[1, 2, 0],
    &[3, 4, 5],
    &[6, 7, 8],
]).unwrap();

puzzle.slide_mut(&Direction::Right).unwrap();

assert_eq!(puzzle.tiles(), &[
    &[1, 0, 2],
    &[3, 4, 5],
    &[6, 7, 8],
]);

let top_left = puzzle.get(0, 0).unwrap();
assert_eq!(top_left, &1);

let position = puzzle.position(&1).unwrap();
assert_eq!(position, (0, 0));

assert_eq!(puzzle.moves(), &[
    Direction::Left,
    Direction::Right,
    Direction::Up,
]);

assert!(puzzle.move_is_valid(&Direction::Up));

puzzle.scramble(100);
```

## Benchmarks

Benchmarks can be run with `cargo bench`:

```
test cloning_a_fifteen_puzzle                         ... bench:          36 ns/iter (+/- 5)
test from_decimal_on_a_fifteen_puzzle                 ... bench:         416 ns/iter (+/- 40)
test from_decimal_unchecked_on_a_fifteen_puzzle       ... bench:         391 ns/iter (+/- 9)
test moves_for_a_fifteen_puzzle                       ... bench:          45 ns/iter (+/- 3)
test new_eight_puzzle                                 ... bench:         320 ns/iter (+/- 30)
test new_eighty_puzzle                                ... bench:         739 ns/iter (+/- 27)
test scramble_fifty_moves_on_a_fifteen_puzzle         ... bench:      10,809 ns/iter (+/- 1,181)
test ten_mutable_slides_on_a_fifteen_puzzle           ... bench:         520 ns/iter (+/- 24)
test ten_mutable_unchecked_slides_on_a_fifteen_puzzle ... bench:         455 ns/iter (+/- 4)
test ten_slides_on_a_fifteen_puzzle                   ... bench:         899 ns/iter (+/- 61)
test ten_unchecked_slides_on_a_fifteen_puzzle         ... bench:         782 ns/iter (+/- 11)
test to_decimal_on_a_fifteen_puzzle                   ... bench:         234 ns/iter (+/- 4)
test to_decimal_unchecked_on_a_fifteen_puzzle         ... bench:         104 ns/iter (+/- 1)
```

## Example Algorithm

There is an example search algorithm in `src/bin/search.rs`:

```
$ time cargo run --release

=== searching 181439 3x3 puzzles ===

estimated memory requirement: 0.000 GB
estimated compute time: 0.00 minutes

     0.0%  |  depth: 0, width: 1
     0.0%  |  depth: 1, width: 2
     0.0%  |  depth: 2, width: 4
     0.0%  |  depth: 3, width: 8
     0.0%  |  depth: 4, width: 16
     0.0%  |  depth: 5, width: 20
     0.0%  |  depth: 6, width: 39
     0.1%  |  depth: 7, width: 62
     0.1%  |  depth: 8, width: 116
     0.2%  |  depth: 9, width: 152
     0.4%  |  depth: 10, width: 286
     0.6%  |  depth: 11, width: 396
     1.0%  |  depth: 12, width: 748
     1.6%  |  depth: 13, width: 1024
     2.6%  |  depth: 14, width: 1893
     4.0%  |  depth: 15, width: 2512
     6.5%  |  depth: 16, width: 4485
     9.6%  |  depth: 17, width: 5638
    14.8%  |  depth: 18, width: 9529
    20.8%  |  depth: 19, width: 10878
    30.2%  |  depth: 20, width: 16993
    39.6%  |  depth: 21, width: 17110
    52.8%  |  depth: 22, width: 23952
    64.0%  |  depth: 23, width: 20224
    77.2%  |  depth: 24, width: 24047
    85.8%  |  depth: 25, width: 15578
    93.8%  |  depth: 26, width: 14560
    97.3%  |  depth: 27, width: 6274
    99.5%  |  depth: 28, width: 3910
    99.9%  |  depth: 29, width: 760
   100.0%  |  depth: 30, width: 221
   100.0%  |  depth: 31, width: 2

radius: 31
max width: 24047
depth of max: 24

here's one of the 2 hardest 3x3 puzzles:

  8   7   6

  0   4   1

  2   5   3

it requires 31 moves to solve

real 0m0.261s
```
