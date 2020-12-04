use std::time::Instant;

mod const_eval;

const CONST_TREES: i32 = crate::const_eval::traverse(3, 1);

fn main() {
    let lines: Vec<String> = day3::read_lines().expect("unable to load input file");

    let forest = day3::Forest::new(lines);

    let mut traverser = day3::Traverser::new(&forest);

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

    let now = Instant::now();
    let trees = CONST_TREES;
    let elapsed = now.elapsed();

    println!(
        "part_one const eval: encountered {} trees in {} nanoseconds",
        trees,
        elapsed.as_nanos()
    );
}
