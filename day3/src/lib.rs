use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const WIDTH: usize = 31;
const HEIGHT: usize = 323;

pub fn read_lines() -> std::io::Result<Vec<String>> {
    let file = File::open("input.txt")?;

    Ok(BufReader::new(file)
        .lines()
        .map(|line| line.expect("unable to read line"))
        .collect())
}

#[derive(Clone, Copy)]
pub(crate) enum Tile {
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

pub struct Forest {
    area: [[Tile; WIDTH]; HEIGHT],
}

impl Forest {
    pub fn new(input: Vec<String>) -> Self {
        let mut area: [[Tile; WIDTH]; HEIGHT] = [[Tile::Open; WIDTH]; HEIGHT];

        for (row, line) in input.iter().enumerate() {
            for (col, tile) in line.chars().enumerate() {
                area[row][col] = tile.into();
            }
        }

        Forest { area }
    }
}

pub struct Traverser<'a> {
    forest: &'a Forest,
    /// number of encountered trees
    trees: usize,
    /// current position in the forest
    position: (usize, usize),
}

impl<'a> Traverser<'a> {
    pub fn new(forest: &'a Forest) -> Self {
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
                .get_unchecked(self.position.1 % WIDTH)
        }
    }

    #[inline]
    fn has_reached_bottom(&self) -> bool {
        self.position.0 >= self.forest.area.len() - 1
    }

    #[inline]
    fn go_right(&mut self, amount: usize) {
        self.position.1 += amount;
    }

    #[inline]
    fn go_down(&mut self, amount: usize) {
        self.position.0 += amount;
    }

    #[inline]
    fn traverse(&mut self, right: usize, down: usize) -> usize {
        while !self.has_reached_bottom() {
            self.go_right(right);
            self.go_down(down);

            self.detect_tree();
        }
        self.trees
    }

    pub fn part_one(&mut self) -> usize {
        self.reset();
        self.traverse(3, 1)
    }

    pub fn part_two(&mut self) -> usize {
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
