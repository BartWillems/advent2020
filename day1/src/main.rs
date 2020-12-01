use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").unwrap();

    let reader = BufReader::new(file);

    let mut numbers: Vec<u16> = reader
        .lines()
        .map(|line| line.unwrap().parse::<u16>().unwrap())
        .collect();

    numbers.sort();

    let now = std::time::Instant::now();
    let pair = find_pairs(&numbers);
    let elapsed = now.elapsed().as_nanos();
    println!("matching pair {:?} in {} nanoseconds", pair, elapsed);

    let now = std::time::Instant::now();
    let res = find_tripples(&numbers);
    let elapsed = now.elapsed().as_nanos();

    println!("matching tripples: {:?} in {} nanoseconds", res, elapsed);
}

fn find_pairs(numbers: &[u16]) -> (u16, u16) {
    for (index, number) in numbers.iter().enumerate() {
        for possible_match in numbers.iter().skip(index) {
            if number + possible_match == 2020 {
                return (*number, *possible_match);
            }
        }
    }

    unreachable!();
}

fn find_tripples(numbers: &[u16]) -> (u16, u16, u16) {
    for (first_index, number) in numbers.iter().enumerate() {
        for (second_index, number_two) in numbers.iter().enumerate().skip(first_index) {
            if number + number_two >= 2020 {
                continue;
            }
            for possible_match in numbers.iter().skip(second_index) {
                if number + number_two + possible_match == 2020 {
                    return (*number, *number_two, *possible_match);
                }
            }
        }
    }

    unreachable!();
}
