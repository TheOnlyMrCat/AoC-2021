use std::collections::HashSet;

use crate::get_input;

pub fn run() {
    let input = get_input(8);

    let mut count = 0;
    for entry in input.lines() {
        let (input, output) = entry.split_once('|').unwrap();
        let (input, output) = (input.trim(), output.trim());
        // First, analyse and find which segments are which
        let mut one = HashSet::new();
        let mut four = HashSet::new();
        for number in input.split(' ').chain(output.split(' ')) {
            match number.len() {
                2 => &mut one,
                4 => &mut four,
                _ => continue,
            }
            .extend(number.bytes())
        }
        assert!(!one.is_empty());
        assert!(!four.is_empty());
        let four_diff = four.difference(&one).copied().collect::<HashSet<_>>();
        let mut output_number = 0;
        for number in output.split(' ') {
            output_number = output_number * 10
                + match number.len() {
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    5 => {
                        // 2, 3, or 5. 3 contains 1. 5 contains (4 - 1)
                        let segments = number.bytes().collect::<HashSet<_>>();
                        if segments.is_superset(&one) {
                            3
                        } else if segments.is_superset(&four_diff) {
                            5
                        } else {
                            2
                        }
                    }
                    6 => {
                        // 0, 6, or 9. 9 contains 4, and 0 contains 1.
                        let segments = number.bytes().collect::<HashSet<_>>();
                        if segments.is_superset(&four) {
                            9
                        } else if segments.is_superset(&one) {
                            0
                        } else {
                            6
                        }
                    }
                    7 => 8,
                    _ => unreachable!(),
                }
        }
        count += output_number;
    }

    println!("{count}");
}
