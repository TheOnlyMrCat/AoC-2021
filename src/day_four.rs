use crate::get_input;

/// 5x5 Bingo board, with cached matches
#[derive(Clone)]
struct Board {
    board: Vec<Vec<u32>>,
    matches: [[bool; 5]; 5],
}

impl From<Vec<Vec<u32>>> for Board {
    fn from(board: Vec<Vec<u32>>) -> Self {
        Board {
            board,
            matches: [[false; 5]; 5],
        }
    }
}

pub fn run() {
    let input = get_input(4);

    let paras = input.trim().split("\n\n").collect::<Vec<_>>();
    let (numbers, boards) = paras.split_first().unwrap();
    let numbers = numbers
        .trim()
        .split(',')
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    // Create boards from strings
    let mut boards = boards
        .iter()
        .map(|s| {
            s.trim().split('\n')
                .map(|s| {
                    s.split(' ')
                        .filter(|s| !s.is_empty())
                        .map(|s| s.trim().parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .map(Board::from)
        .collect::<Vec<_>>();

    let mut last_win = 0;

    // Iterate through the numbers, and find the first board that has 5 in a row horizontally, vertically or diagonally
    for number in numbers {
        let mut new_boards = vec![];
        for board in boards.iter_mut() {
            for (y, row) in board.board.iter_mut().enumerate() {
                if let Some(pos) = row.iter().position(|n| n == &number) {
                    board.matches[y][pos] = true;
                }
            }
            let mut win = false;
            
            // Check if we have a horizontal match
            if board.matches.iter().any(|row| row.iter().all(|n| *n)) {
                win = true;
            }
            // Check if we have a vertical match
            for y in 0..5 {
                if board.matches.iter().map(|row| row[y]).all(|n| n) {
                    win = true;
                }
            }

            // Check if we have a diagonal match
            if board.matches.iter().enumerate().map(|(y, row)| row[y]).all(|n| n) {
                win = true;
            }
            // Check if we have an anti-diagonal match
            if board.matches.iter().enumerate().map(|(y, row)| row[4 - y]).all(|n| n) {
                win = true;
            }

            if win {
                let sum_unmatched = board
                    .board
                    .iter()
                    .flat_map(|row| row.iter())
                    .enumerate()
                    .filter(|(i, _)| !board.matches[i / 5][i % 5])
                    .map(|(_, n)| n)
                    .sum::<u32>();
                last_win = sum_unmatched * number;
            } else {
                new_boards.push(board.clone());
            }
        }
        boards = new_boards;
    }

    println!("{}", last_win);
}
