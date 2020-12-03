use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

fn read_lines() -> std::io::Result<Vec<String>> {
    let file = File::open("input.txt")?;

    Ok(BufReader::new(file)
        .lines()
        .map(|line| line.expect("unable to read line"))
        .collect())
}

#[derive(Clone, Copy, Debug)]
enum Tile {
    Open,
    Tree,
}

impl From<char> for Tile {
    fn from(tile: char) -> Self {
        match tile {
            '.' => Tile::Open,
            '#' => Tile::Tree,
            _ => unimplemented!("Tile is not implemented for char '{}'", tile),
        }
    }
}

struct Forest {
    area: [[Tile; 31]; 323],
}

impl Forest {
    fn new(input: Vec<String>) -> Self {
        let mut area: [[Tile; 31]; 323] = [[Tile::Open; 31]; 323];

        for (row, line) in input.iter().enumerate() {
            for (col, tile) in line.chars().enumerate() {
                area[row][col] = tile.into();
            }
        }

        Forest { area }
    }
}

enum Direction {
    Right,
    Down,
}

struct Traverser<'a> {
    forest: &'a Forest,
    /// number of encountered trees
    trees: usize,
    /// current position in the forest
    position: (usize, usize),
}

impl<'a> Traverser<'a> {
    fn new(forest: &'a Forest) -> Self {
        let mut traverser = Traverser {
            forest,
            trees: 0,
            position: (0, 0),
        };

        traverser.detect_tree();

        traverser
    }

    fn reset(&mut self) {
        self.trees = 0;
        self.position = (0, 0);
        self.detect_tree();
    }

    #[inline]
    fn detect_tree(&mut self) {
        if let Tile::Tree = self.current_tile() {
            self.trees += 1;
        }
    }

    #[inline]
    fn current_tile(&self) -> &Tile {
        unsafe {
            self.forest
                .area
                .get_unchecked(self.position.0)
                .get_unchecked(self.position.1)
        }
    }

    #[inline]
    fn has_reached_bottom(&self) -> bool {
        self.position.0 == self.forest.area.len() - 1
    }

    /// returns true if wen can still go down
    #[inline]
    fn walk(&mut self, direction: Direction) {
        match direction {
            Direction::Right => {
                self.go_right();
            }
            Direction::Down => self.go_down(),
        };
    }

    #[inline]
    fn go_right(&mut self) {
        // wrap around at the end
        if self.position.1 == 30 {
            self.position.1 = 0
        } else {
            self.position.1 += 1;
        }
    }

    #[inline]
    fn go_down(&mut self) {
        self.position.0 += 1;
    }

    #[inline]
    fn traverse(&mut self, right: usize, down: usize) -> usize {
        while !self.has_reached_bottom() {
            for _i in 0..right {
                self.walk(Direction::Right);
            }
            for _i in 0..down {
                self.walk(Direction::Down);
            }

            self.detect_tree();
        }
        self.trees
    }

    fn part_one(&mut self) -> usize {
        self.reset();
        self.traverse(3, 1)
    }

    fn part_two(&mut self) -> usize {
        let mut trees = 1;
        self.reset();
        trees *= self.traverse(1, 1);
        self.reset();
        trees *= self.traverse(3, 1);
        self.reset();
        trees *= self.traverse(5, 1);
        self.reset();
        trees *= self.traverse(7, 1);
        self.reset();
        trees *= self.traverse(1, 2);
        trees
    }
}

fn main() {
    let lines: Vec<String> = read_lines().expect("unable to load input file");

    let forest = Forest::new(lines);

    let mut traverser = Traverser::new(&forest);

    let now = Instant::now();
    let trees = traverser.part_one();
    let elapsed = now.elapsed();

    println!(
        "encountered {} trees in {} nanoseconds",
        trees,
        elapsed.as_nanos()
    );

    let now = Instant::now();
    let trees = traverser.part_two();
    let elapsed = now.elapsed();

    println!(
        "part_two: encountered {} trees in {} nanoseconds",
        trees,
        elapsed.as_nanos()
    );
}
