use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").unwrap();

    let reader = BufReader::new(file);

    let mut numbers: Vec<usize> = reader
        .lines()
        .map(|line| line.unwrap().parse::<usize>().unwrap())
        .collect();

    let now = std::time::Instant::now();
    numbers.sort_unstable();
    println!("sorting took {} microseconds", now.elapsed().as_micros());

    let now = std::time::Instant::now();
    let pair = part_one(&numbers);
    let elapsed = now.elapsed().as_nanos();
    println!("matching pair {:?} in {} nanoseconds", pair, elapsed);

    let now = std::time::Instant::now();
    let res = part_two(&numbers);
    let elapsed = now.elapsed().as_nanos();

    println!("matching tripples: {:?} in {} nanoseconds", res, elapsed);
}

fn part_one<'a>(numbers: &'a [usize]) -> (&usize, &usize) {
    for (index, number) in numbers.iter().enumerate() {
        for possible_match in numbers.iter().skip(index) {
            if number + possible_match == 2020 {
                return (number, possible_match);
            }
        }
    }

    unreachable!();
}

fn part_two<'a>(numbers: &'a [usize]) -> (&usize, &usize, &usize) {
    for (first_index, one) in numbers.iter().enumerate() {
        for (second_index, two) in numbers.iter().enumerate().skip(first_index) {
            if one + two >= 2020 {
                continue;
            }
            for three in numbers
                .iter()
                .skip(std::cmp::max(first_index, second_index))
            {
                if one + two + three == 2020 {
                    return (one, two, three);
                }
            }
        }
    }

    unreachable!();
}
