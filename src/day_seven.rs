use crate::get_input;

pub fn run() {
    let input = get_input(7);

    let positions = {
        let mut positions = input.trim().split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        positions.sort_unstable();
        positions
    };
    let mean = positions.iter().sum::<i32>() / positions.len() as i32;
    let median = positions[positions.len() / 2];

    // Sum of difference between each position and the median
    let median_sum = positions.iter().map(|x| (x - median).abs()).sum::<i32>();
    println!("Gross distance to median: {}", median_sum);

    // Sum of the (difference between each position and the mean)'th triangular numbers
    let mean_sum = positions.iter().map(|x| {
        let n = (x - mean).abs();
        n * (n + 1) / 2
    }).sum::<i32>();
    println!("Gross distance to mean: {}", mean_sum);
}