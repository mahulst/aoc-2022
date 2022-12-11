use std::collections::BTreeMap;
use std::iter;

use nom::branch::alt;
use nom::bytes::complete::{is_a, tag};
use nom::character::complete;
use nom::character::complete::newline;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

use sliding_windows::IterExt;
use sliding_windows::Storage;

#[derive(Clone, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn left(input: &str) -> IResult<&str, Direction> {
    let (input, _) = is_a("L")(input)?;

    return Ok((input, Direction::Left));
}

fn right(input: &str) -> IResult<&str, Direction> {
    let (input, _) = is_a("R")(input)?;

    return Ok((input, Direction::Right));
}

fn up(input: &str) -> IResult<&str, Direction> {
    let (input, _) = is_a("U")(input)?;

    return Ok((input, Direction::Up));
}

fn down(input: &str) -> IResult<&str, Direction> {
    let (input, _) = is_a("D")(input)?;

    return Ok((input, Direction::Down));
}

fn line(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, (dir, amount)) = separated_pair(alt((left, right, up, down)), tag(" "), complete::u32)(input)?;

    return Ok((input, iter::repeat(dir).take(amount as usize).collect()));
}

fn lines(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, lines) = separated_list1(newline, line)(input)?;

    return Ok((input, lines.into_iter().flatten().collect()));
}

pub fn run_part_1(input: &str) -> String {
    let (_, directions) = lines(input).unwrap();

    let mut map: BTreeMap<(isize, isize), isize> = BTreeMap::new();
    let mut head_coord: (isize, isize) = (0, 0);
    let mut last_head_coord: (isize, isize) = (0, 0);
    let mut tail_coord: (isize, isize) = (0, 0);
    for dir in directions {
        match dir {
            Direction::Left => {
                head_coord.0 -= 1;
            }
            Direction::Right => {
                head_coord.0 += 1;
            }
            Direction::Up => {
                head_coord.1 += 1;
            }
            Direction::Down => {
                head_coord.1 -= 1;
            }
        }

        let diff_x = (tail_coord.0 - head_coord.0).abs();
        let diff_y = (tail_coord.1 - head_coord.1).abs();
        if diff_x > 1 || diff_y > 1 {
            tail_coord = last_head_coord;
            println!("{}, {}", tail_coord.0, tail_coord.1);
            let mut entry = map.entry(tail_coord).or_insert(0);
            *entry += 1;
        }
        last_head_coord = head_coord;
    }

    dbg!(&map);
    return (map.len() + 1).to_string();
}

pub fn run_part_2(input: &str) -> String {
    let (_, directions) = lines(input).unwrap();
    let mut snake: [(isize, isize); 10] = [(0, 0); 10];

    let mut map: BTreeMap<(isize, isize), isize> = BTreeMap::new();
    let mut last_head_coord: (isize, isize) = (0, 0);
    for dir in directions {
        // print_map(&map, &snake);

        match dir {
            Direction::Left => {
                snake[0].0 -= 1;
            }
            Direction::Right => {
                snake[0].0 += 1;
            }
            Direction::Up => {
                snake[0].1 += 1;
            }
            Direction::Down => {
                snake[0].1 -= 1;
            }
        }

        for x in 1..10 {
            let (l, r) = snake.split_at_mut(x);
            let mut head_coord = l.last_mut().unwrap();
            let mut tail_coord = r.first_mut().unwrap();

            let diff_x =  head_coord.0 - tail_coord.0;
            let diff_y = head_coord.1 - tail_coord.1;

            if diff_x.abs() > 1 || diff_y.abs() > 1 {
                // println!();
                // println!();
                // println!("Moving knot {}", x);
                // println!("head moved to diff: {},{}",diff_x, diff_y);
                if diff_x.abs() > 1 {
                    tail_coord.0 += diff_x.signum() * 1;

                    if diff_y.abs() > 0 {
                        tail_coord.1 += diff_y.signum() * 1;
                    }
                } else if diff_y.abs() > 1 {
                    tail_coord.1 += diff_y.signum() * 1;

                    if diff_x.abs() > 0 {
                        tail_coord.0 += diff_x.signum() * 1;
                    }
                }
                // eprintln!("tail moved to {},{}", tail_coord.0, tail_coord.1);


                if x == 9 {
                    let mut entry = map.entry(tail_coord.clone()).or_insert(0);
                    *entry += 1;
                }
            }
        }
    }
    return (map.len() + 1).to_string();
}

const MAP_SIZE: isize = 8;

fn print_map(map: &BTreeMap<(isize, isize), isize>, snake: &[(isize, isize); 10]) {
    println!();
    for y in -MAP_SIZE..MAP_SIZE {
        for x in -MAP_SIZE..MAP_SIZE {
            let is_snake = snake.iter().enumerate().find_map(|(i, knot)| {
                if (x, y) == *knot {
                    match i {
                        0 => Some("H".to_string()),
                        9 => Some("S".to_string()),
                        _ => Some(i.to_string())
                    }
                } else {
                    None
                }
            });
            let in_map = map.get(&(x, y)).map(|v| "#".to_string()).unwrap_or(".".to_string());
            print!("{}", is_snake.unwrap_or(in_map));
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run_part_1("R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2");
        assert_eq!(result, "13");
    }

    #[test]
    fn it_works2() {
        let result = run_part_2("R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2");
        assert_eq!(result, "1");
    }

    #[test]
    fn it_works2_larger_test() {
        let result = run_part_2("R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20");
        assert_eq!(result, "36");
    }
}
