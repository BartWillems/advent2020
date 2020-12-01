use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").unwrap();

    let reader = BufReader::new(file);

    let numbers: Vec<u16> = reader
        .lines()
        .map(|line| line.unwrap().parse::<u16>().unwrap())
        .collect();

    let now = std::time::Instant::now();
    let pair = find_pairs(&numbers);
    println!(
        "matching pair {:?} in {} nano seconds",
        pair,
        now.elapsed().as_nanos()
    );

    let now = std::time::Instant::now();
    let res = find_tripples(&numbers);

    println!(
        "matching tripples: {:?} in {} nanoseconds",
        res,
        now.elapsed().as_nanos()
    );
}

fn find_pairs(numbers: &[u16]) -> (u16, u16) {
    for (first_index, number) in numbers.iter().enumerate() {
        for possible_match in numbers.iter().skip(first_index) {
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
            for possible_match in numbers.iter().skip(second_index) {
                if number + number_two + possible_match == 2020 {
                    return (*number, *number_two, *possible_match);
                }
            }
        }
    }

    unreachable!();
}
