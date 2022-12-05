use std::cmp::{max, min};
use std::collections::HashMap;
use nom::{
    bytes::complete::tag,
    multi::many1,
    *,
    character::complete::{anychar, one_of},
    multi::{many0, separated_list1},
    sequence::{delimited, pair, terminated},
};

#[derive(Debug, Clone)]
enum ContainerSlot {
    Empty,
    Container(char),
}

#[derive(Debug)]
struct Command {
    from: u32,
    to: u32,
    amount: u32,
}

type Ship = HashMap<u32, Vec<char>>;

fn empty_slot(input: &str) -> IResult<&str, ContainerSlot> {
    let (input, _) = tag("   ")(input)?;

    return Ok((input, ContainerSlot::Empty));
}

fn container(input: &str) -> IResult<&str, ContainerSlot> {
    let (input, char) = delimited(tag("["), anychar, tag("]"))(input)?;

    return Ok((input, ContainerSlot::Container(char)));
}

fn row(input: &str) -> IResult<&str, Vec<ContainerSlot>> {
    let (input, container_slots) = separated_list1(tag(" "), branch::alt((container, empty_slot)))(input)?;

    Ok((input, container_slots))
}

fn rows(input: &str) -> IResult<&str, Vec<Vec<ContainerSlot>>> {
    let (input, r) = many1(terminated(row, tag("\n")))(input)?;

    Ok((input, r))
}

fn command(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("move ")(input)?;
    let (input, amount) = crate::character::complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = crate::character::complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = crate::character::complete::u32(input)?;

    return Ok((input, Command {
        from: from - 1,
        to: to - 1,
        amount,
    }));
}

fn commands(input: &str) -> IResult<&str, Vec<Command>> {
    let (input, r) = many1(terminated(command, tag("\n")))(input)?;

    Ok((input, r))
}

pub fn run_part_1(input: &str, input2: &str) -> String {
    let (_, initial) = rows(input).unwrap();
    let longest = initial.iter().fold(0, |acc, r| acc.max(r.len()));
    let mut ship: Ship = HashMap::new();

    initial.iter().for_each(|a| {
        a.iter().enumerate().for_each(|(i, container_slot)| {
            match container_slot {
                ContainerSlot::Container(c) => {
                    let mut row = ship.entry(i as u32).or_insert(vec![]);

                    row.push(*c);
                }
                _ => {}
            }
        });
    });

    let (_, commands) = commands(input2).unwrap();

    for command in commands {
        let mut containers_to_add: Vec<char> = vec![];
        {
            let mut containers_in_row = ship.get_mut(&command.from).unwrap();
            let mut containers_removed: Vec<char> = containers_in_row.drain(0..command.amount as usize).collect();
            containers_to_add.append(&mut containers_removed)
        }
        let mut containers_in_row = ship.get_mut(&command.to).unwrap();
        containers_to_add.into_iter().for_each(|container| {
            containers_in_row.insert(0, container);
        });
    }

    let mut output = String::new();
    for x in 0..longest {
        output.push(ship.get(&(x as u32)).unwrap().first().unwrap().clone());
    }

    return output;
}

pub fn run_part_2(input: &str, input2: &str) -> String {
    let (_, initial) = rows(input).unwrap();
    let longest = initial.iter().fold(0, |acc, r| acc.max(r.len()));
    let mut ship: Ship = HashMap::new();

    initial.iter().for_each(|a| {
        a.iter().enumerate().for_each(|(i, container_slot)| {
            match container_slot {
                ContainerSlot::Container(c) => {
                    let mut row = ship.entry(i as u32).or_insert(vec![]);

                    row.push(*c);
                }
                _ => {}
            }
        });
    });

    let (_, commands) = commands(input2).unwrap();

    for command in commands {
        let mut containers_to_add: Vec<char> = vec![];
        {
            let mut containers_in_row = ship.get_mut(&command.from).unwrap();
            let mut containers_removed: Vec<char> = containers_in_row.drain(0..command.amount as usize).collect();
            containers_to_add.append(&mut containers_removed)
        }
        let mut containers_in_row = ship.get_mut(&command.to).unwrap();
        containers_to_add.into_iter().rev().for_each(|container| {
            containers_in_row.insert(0, container);
        });
    }

    let mut output = String::new();
    for x in 0..longest {
        output.push(ship.get(&(x as u32)).unwrap().first().unwrap().clone());
    }
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3
";
        let input2 = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
        let result = run_part_1(input, input2);
        assert_eq!(result, "CMZ");
    }
    #[test]
    fn it_works2() {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3
";
        let input2 = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
        let result = run_part_2(input, input2);
        assert_eq!(result, "MCD");
    }
}
