use std::collections::HashMap;
use array_tool::vec::Intersect;
use iter_tools::*;
pub fn get_score(letter: char) -> usize {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    return *letter_scores.get(&letter).unwrap();
}

pub fn run_part_1(input: &str) -> String {
    let v: Vec<char> = input.lines().map(|l| {
        let left: Vec<char> = l.chars().take(l.len() / 2).collect();
        let right: Vec<char> = l.chars().rev().take(l.len() / 2).collect();
        let intersect: Vec<char> = left.intersect(right.clone());

        intersect[0]
    }).collect();

    v.iter().map(|i| {
        get_score(*i)
    }).sum::<usize>().to_string()
}

pub fn run_part_2(input: &str) -> String {
    let mut count = 0;
    let mut lines = input.lines();
    while let (Some(line1), Some(line2),Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        let chars1: Vec<char> = line1.chars().collect();
        let chars2: Vec<char> = line2.chars().collect();
        let chars3: Vec<char> = line3.chars().collect();

        let a = chars1.intersect(chars2).intersect(chars3);

        count += get_score(*a.first().unwrap());
    }

    return count.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let result = run_part_1(input);
        assert_eq!(result, "157");
    }

    #[test]
    fn it_works_2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let result = run_part_2(input);

        assert_eq!(result, "70");
    }
}
