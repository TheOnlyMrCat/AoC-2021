use crate::get_input;

pub fn run() {
    run_formula();
}

// This part was my code.
fn run_sim() {
    let input = get_input(6);

    let mut fish = input
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut new_fish = vec![];

    // Yeah I tried to model it with a sim. I guess I forgot what "exponential time" means.
    let iterations = 80;
    for i in 0..iterations {
        for f in fish.iter_mut() {
            if *f == 0 {
                *f = 6;
                new_fish.push(8);
            } else {
                *f -= 1;
            }
        }
        fish.append(&mut new_fish);
        new_fish.clear();
        print_progress(i, iterations);
    }

    println!("{}", fish.len());
}

fn print_progress(current: i32, max: i32) {
    print!("\x1b[2K\r");
    let chars_progress = (current * 64) / max;
    for _ in 1..chars_progress {
        print!("=");
    }
    print!(">\x1b[64G| {}/{}", current, max);
}

// Stolen from somebody else's solution, and it's actually amazing.
fn run_formula() {
    static TABLE: [u64; 300] = {
        let mut table = [0; 300];
        let mut n = 0;
        while n < 300 {
            if n <= 9 {
                table[n] = 1;
            } else {
                table[n] = table[n - 7] + table[n - 9];
            }
            n += 1;
        }
        table
    };

    let input = get_input(6);

    let iterations = 256;
    let solution: u64 = input
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .map(|x| TABLE[iterations - x + 9])
        .sum();
    println!("{}", solution);
}
