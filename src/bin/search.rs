extern crate sliding_puzzle;
extern crate lehmer;
extern crate bit_set;

use sliding_puzzle::*;
use bit_set::BitSet;
use lehmer::Lehmer;
use std::collections::VecDeque;

fn main() {
    let rows = 3;
    let columns = 3;
    let max = Lehmer::max_value(rows * columns);
    let memory = estimate_memory(rows, columns, max);
    let time = estimate_time(max);

    println!("\n=== searching {} {}x{} puzzles ===", max / 2, rows, columns);
    println!("\nestimated memory requirement: {:.3} GB", memory);
    println!("estimated compute time: {:.2} minutes\n", time);

    let mut seen = BitSet::with_capacity(max as usize);
    let mut queue = VecDeque::<u64>::new();
    let mut next_queue = VecDeque::<u64>::new();
    let mut depth = -1;
    let mut depth_of_max = -1;
    let mut width = 0;
    let mut max_width = 0;
    let mut percent = 0.0;
    let mut hardest = 0;

    queue.push_back(0);

    while !queue.is_empty() {
        depth += 1;
        width = queue.len();
        percent += queue.len() as f64 / (max as f64 / 2.0) * 100.0;
        hardest = queue.front().unwrap().clone();

        println!("{:8.1}%  |  depth: {}, width: {}", percent, depth, width);

        while let Some(decimal) = queue.pop_back() {
            let puzzle = SlidingPuzzle::from_decimal_unchecked(decimal, rows, columns);

            for direction in puzzle.moves() {
                let next_decimal = puzzle.slide_unchecked(&direction)
                                         .to_decimal_unchecked();

                if seen.insert(next_decimal as usize) {
                    next_queue.push_back(next_decimal);
                }
            }
        }

        queue = next_queue;
        next_queue = VecDeque::<u64>::with_capacity(queue.len());

        if width > max_width {
            max_width = width;
            depth_of_max = depth;
        }
    }

    println!("\nradius: {}", depth);
    println!("max width: {}", max_width);
    println!("depth of max: {}", depth_of_max);
    println!("\nhere's one of the {} hardest {}x{} puzzles:", width, rows, columns);

    let hardest_puzzle = SlidingPuzzle::from_decimal(hardest, rows, columns).unwrap();
    println!("");
    for row in hardest_puzzle.tiles() {
        for tile in row {
            print!(" {:2} ", tile);
        }
        println!("\n");
    }

    println!("it requires {} moves to solve", depth);
}

fn estimate_memory(rows: usize, columns: usize, max: u64) -> f64 {
    let mut estimate = max; // bit vector

    let smaller;
    let bigger;
    if rows < columns {
        smaller = rows;
        bigger = columns;
    } else {
        smaller = columns;
        bigger = rows;
    }

    // From the paper: Linear-Time Disk-Based Implicit Graph Search
    let max_width = match (smaller, bigger) {
        (2, 2) => 2,
        (2, 3) => 44,
        (2, 4) => 1_999,
        (2, 5) => 133_107,
        (2, 6) => 13_002_649,
        (2, 7) => 1_862_320_864,
        (2, 8) => 367_084_684_402,
        (3, 3) => 24_047,
        (3, 4) => 21_841_159,
        (3, 5) => 45_473_143_333,
        (4, 4) => 784_195_801_886,
        _ => 0,
    };

    estimate += max_width * 8; // queue
    estimate as f64 / 8.0 / 1024.0 / 1024.0 / 1024.0 // GB
}

fn estimate_time(max: u64) -> f64 {
    let rate_per_second = 1_055_000;
    let states = max / 2;

    states as f64 / rate_per_second as f64 / 60.0
}
