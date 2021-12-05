use tap::Pipe;

use crate::get_input;

pub fn run() {
    let input = get_input(5);

    let vents = input.trim().split('\n').map(|line| {
        let (start, end) = line.trim().split_once(" -> ").unwrap();
        let (x1, y1) = start.split_once(',').unwrap().pipe(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()));
        let (x2, y2) = end.split_once(',').unwrap().pipe(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()));
        ((x1, y1), (x2, y2))
    });

    let mut grid = [[0; 1000]; 1000];

    for ((x1, y1), (x2, y2)) in vents {
        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                grid[x1 as usize][y as usize] += 1;
            }
        } else if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                grid[x as usize][y1 as usize] += 1;
            }
        } else {
            let ((x_min, y_start), (x_max, y_end)) = if x1 < x2 {
                ((x1, y1), (x2, y2))
            } else {
                ((x2, y2), (x1, y1))
            };

            let mut x = x_min;
            let mut y = y_start;

            loop {
                grid[x as usize][y as usize] += 1;

                if x == x_max {
                    break;
                }

                x += 1;
                if y_start < y_end {
                    y += 1;
                } else {
                    y -= 1;
                }
            }
        }
    }

    let mut dangerous_count = 0;
    for row in grid.iter() {
        for &cell in row.iter() {
            if cell > 1 {
                dangerous_count += 1;
            }
        }
    }

    println!("{}", dangerous_count);
}