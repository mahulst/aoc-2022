pub fn run_part_1(input: &str) -> String {
    let elf_with_heaviest_load = input.split("\n\n").map(|single_elf| {
        return single_elf.lines().map(|item| {
            item.trim().parse::<usize>().unwrap()
        }).sum::<usize>();
    }).max().unwrap();

    return elf_with_heaviest_load.to_string();
}

pub fn run_part_2(input: &str) -> String {
    let mut three_elves: Vec<usize> = input.split("\n\n").map(|single_elf| {
        return single_elf.lines().map(|item| {
            item.trim().parse::<usize>().unwrap()
        }).sum::<usize>();
    }).collect();

    three_elves.sort();
    three_elves.reverse();

    return three_elves.iter().take(3).sum::<usize>().to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_works() {
        let input = "1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000";
        let result = run_part_1(input);
        assert_eq!(result, "24000");
    }

    #[test]
    fn test_part_2_works() {
        let input = "1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000";
        let result = run_part_2(input);
        assert_eq!(result, "45000");
    }
}
