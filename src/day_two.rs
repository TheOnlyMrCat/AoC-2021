use crate::get_input;

enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

pub fn run() {
    let input = get_input(2);
    let commands = input.trim().split('\n').map(|s| {
        let (dir, dist) = s.trim().split_once(' ').unwrap();
        match dir {
            "forward" => Command::Forward(dist.parse::<i32>().unwrap()),
            "up" => Command::Up(dist.parse::<i32>().unwrap()),
            "down" => Command::Down(dist.parse::<i32>().unwrap()),
            _ => panic!("Unknown command: {}", dir),
        }
    }).collect::<Vec<_>>();

    let mut distance = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands {
        match command {
            Command::Forward(d) => {
                distance += d;
                depth += aim * d;
            }
            Command::Up(d) => {
                aim -= d;
            }
            Command::Down(d) => {
                aim += d;
            }
        }
    }

    println!("{}", distance * depth);
}