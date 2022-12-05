use day::run_part_1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    let file2 = fs::read_to_string("./input2.txt").unwrap();
    println!("{}", run_part_1(&file, &file2));
}