use std::collections::{HashSet, VecDeque};

use crate::get_input;

pub fn run() {
    let input = get_input(9);

    let width = {
        let mut temp = input.clone();
        let _ = temp.split_off(input.find('\n').unwrap());
        temp.len()
    };
    let grid = input
        .into_bytes()
        .into_iter()
        .filter(|&c| c != b'\n')
        .map(|c| c - b'0')
        .collect::<Vec<u8>>();
    let size = grid.len();

    let mut basins = vec![];
    for (pos, depth) in grid.iter().copied().enumerate() {
        let mut is_low_point = true;
        is_low_point &= pos < width || grid[pos - width] > depth; // Up
        is_low_point &= pos % width == 0 || grid[pos - 1] > depth; // Left
        is_low_point &= pos >= size - width || grid[pos + width] > depth; // Down
        is_low_point &= pos % width == width - 1 || grid[pos + 1] > depth; // Right
        if is_low_point {
            // All non-nines are in exactly 1 basin, per spec, so do a BFS to find the others
            let mut queue = VecDeque::new();
            let mut basin = HashSet::new();

            if pos >= width {
                queue.push_back(pos - width);
            }
            if pos % width != 0 {
                queue.push_back(pos - 1)
            }
            if pos < size - width {
                queue.push_back(pos + width);
            }
            if pos % width != width - 1 {
                queue.push_back(pos + 1);
            }

            while let Some(cell) = queue.pop_front() {
                if grid[cell] != 9 && basin.insert(cell) {
                    if cell >= width {
                        queue.push_back(cell - width);
                    }
                    if cell % width != 0 {
                        queue.push_back(cell - 1)
                    }
                    if cell < size - width {
                        queue.push_back(cell + width);
                    }
                    if cell % width != width - 1 {
                        queue.push_back(cell + 1);
                    }
                }
            }

            basins.push(basin.len());
        }
    }

    basins.sort_unstable();
    if let [first, second, third] = basins.rchunks(3).next().unwrap() {
        println!("{} * {} * {} = {}", first, second, third, first * second * third);
    }
}
