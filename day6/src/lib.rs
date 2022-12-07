fn unique(s: &[char]) -> Option<(usize, usize, char)> {
    s.iter().map(|a| a.clone()).enumerate().find_map(|(i, c)| {
        s.iter().map(|a| a.clone()).enumerate()
            .skip(i + 1)
            .find(|(_, other)| c == *other)
            .map(|(j, _)| (i, j, c))
    })
}

pub fn run_part_1(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut count = 4;
    for w in chars.windows(4) {
        if unique(&w).is_none() {
            return count.to_string();
        } else {
            count += 1;
        }
    }
    return "0".to_string();
}

pub fn run_part_2(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut count = 14;
    for w in chars.windows(14) {
        if unique(&w).is_none() {
            return count.to_string();
        } else {
            count += 1;
        }
    }
    return "0".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run_part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(result, "7");
    }
}
