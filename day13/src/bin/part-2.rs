use day::run_part_2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", run_part_2(&file));
}