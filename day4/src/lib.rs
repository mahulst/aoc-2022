use std::ops::{Range};
use nom::{
    bytes::complete::tag,
    character::complete::{self},
    multi::many1,
    sequence::separated_pair,
    *,
};
use nom::sequence::{pair, terminated};

#[derive(Debug)]
struct Row {
    left: Range<u64>,
    right: Range<u64>,
}

fn range(input: &str) -> IResult<&str, Range<u64>> {
    let (input, (start, end)) = separated_pair(
        complete::u64,
        tag("-"),
        complete::u64,
    )(input)?;

    Ok((input, start..end + 1))
}

fn row(input: &str) -> IResult<&str, Row> {
    let (input, (l, r)) = separated_pair(range, tag(","), range)(input)?;

    Ok((input, Row { left: l, right: r }))
}

fn rows(input: &str) -> IResult<&str, Vec<Row>> {
    let (input, r) = many1(terminated(row, tag("\n")))(input)?;

    Ok((input, r))
}

pub fn run_part_1(input: &str) -> String {
    let (_, rows) = rows(input).unwrap();

    let result: usize = rows.iter().map(|row| {
        let left_contains_right = row.left.clone().into_iter().all(|i| row.right.contains(&i));
        let right_contains_left = row.right.clone().into_iter().all(|i| row.left.contains(&i));

        if left_contains_right || right_contains_left {
            1
        } else {
            0
        }
    }).sum();

    return result.to_string();
}

pub fn run_part_2(input: &str) -> String {
    let (_, rows) = rows(input).unwrap();

    let result: usize = rows.iter().map(|row| {
        let left_overlaps_right = row.left.clone().into_iter().any(|i| row.right.contains(&i));

        if left_overlaps_right {
            1
        } else {
            0
        }
    }).sum();

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let result = run_part_1(input);
        assert_eq!(result, "2");
    }

    #[test]
    fn it_works_2() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

        let result = run_part_2(input);

        assert_eq!(result, "4");
    }
}
