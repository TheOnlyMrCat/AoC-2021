use crate::get_input;

pub fn run() {
    let input = get_input(3);
    let rates = input.trim().split('\n').collect::<Vec<_>>();
    
    let mut gamma = String::new();
    let mut epsilon = String::new();

    for bit in 0..rates[0].len() {
        let mut zeroes = 0;
        let mut ones = 0;
        for rate in rates.iter() {
            if rate.chars().nth(bit).unwrap() == '0' {
                zeroes += 1;
            } else {
                ones += 1;
            }
        }
        if zeroes > ones {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }

    let gamma = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon, 2).unwrap();
    println!("Power: {}", gamma * epsilon);

    let mut o2_filter = rates.clone();
    for bit in 0..rates[0].len() {
        if o2_filter.len() == 1 {
            break;
        }
        let mut zeroes = vec![];
        let mut ones = vec![];
        for rate in o2_filter.iter() {
            if rate.chars().nth(bit).unwrap() == '0' {
                zeroes.push(*rate);
            } else {
                ones.push(*rate);
            }
        }
        if zeroes.len() > ones.len() {
            o2_filter = zeroes;
        } else {
            o2_filter = ones;
        }
    }

    let mut co2_filter = rates.clone();
    for bit in 0..rates[0].len() {
        if co2_filter.len() == 1 {
            break;
        }
        let mut zeroes = vec![];
        let mut ones = vec![];
        for rate in co2_filter.iter() {
            if rate.chars().nth(bit).unwrap() == '0' {
                zeroes.push(*rate);
            } else {
                ones.push(*rate);
            }
        }
        if zeroes.len() > ones.len() {
            co2_filter = ones;
        } else {
            co2_filter = zeroes;
        }
    }

    let o2_filter = u32::from_str_radix(o2_filter[0], 2).unwrap();
    let co2_filter = u32::from_str_radix(co2_filter[0], 2).unwrap();
    println!("LSR: {}", o2_filter * co2_filter);
}