use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use smallvec::SmallVec;

struct Password<'a> {
    token: u8,
    min: usize,
    max: usize,
    password: &'a [u8],
}

impl<'a> Password<'a> {
    fn from_string(input: &'a str) -> Self {
        // SAFETY: this should not fail as long as the correct code advent day 2 2020 input is used
        // otherwise, bad things will happen
        unsafe {
            let pairs: SmallVec<[&str; 3]> = input.split_whitespace().collect();

            let min_max: SmallVec<[&str; 2]> = pairs.get_unchecked(0).split("-").collect();

            let min: usize = min_max.get_unchecked(0).parse().unwrap();
            let max: usize = min_max.get_unchecked(1).parse().unwrap();

            let token: u8 = *pairs.get_unchecked(1).as_bytes().get_unchecked(0);

            let password = pairs.get_unchecked(2);

            Password {
                token,
                min,
                max,
                password: password.as_bytes(),
            }
        }
    }

    #[inline(always)]
    fn is_valid_part_one(&self) -> bool {
        let mut token_count = 0;

        for t in self.password {
            if t == &self.token {
                token_count += 1;
            }
        }

        token_count >= self.min && token_count <= self.max
    }

    #[inline(always)]
    fn is_valid_part_two(&self) -> bool {
        // SAFETY: this should not fail as long as the correct code advent day 2 2020 input is used
        // otherwise, bad things will happen
        unsafe {
            let min = self.password.get_unchecked(self.min - 1) == &self.token;
            let max = self.password.get_unchecked(self.max - 1) == &self.token;

            min ^ max
        }
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();

    let reader: BufReader<File> = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let now = std::time::Instant::now();
    let mut count = 0;
    lines.iter().for_each(|line| {
        if Password::from_string(line).is_valid_part_one() {
            count += 1;
        }
    });
    let elapsed = now.elapsed();

    println!(
        "part 1: found {} correct passwords in {} microseconds",
        count,
        elapsed.as_micros()
    );

    let now = std::time::Instant::now();
    let mut count = 0;
    lines.iter().for_each(|line| {
        if Password::from_string(line).is_valid_part_two() {
            count += 1;
        }
    });
    let elapsed = now.elapsed();

    println!(
        "part 2: found {} correct passwords in {} microseconds",
        count,
        elapsed.as_micros()
    );
}
