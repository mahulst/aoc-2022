use std::collections::BTreeMap;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::newline;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, separated_pair};

#[derive(Debug)]
enum Instruction {
    Add(i32),
    Noop,
}

fn add(input: &str) -> IResult<&str, Instruction> {
    let (input, (amount)) = preceded(tag("addx "), complete::i32)(input)?;

    return Ok((input, Instruction::Add(amount)));
}

fn noop(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("noop")(input)?;

    return Ok((input, Instruction::Noop));
}


fn lines(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, lines) = separated_list1(newline, alt((noop, add)))(input)?;

    return Ok((input, lines));
}

pub fn run_part_1(input: &str) -> String {
    let (i, instructions) = lines(input).unwrap();

    let mut cycle: i32 = 0;
    let mut v: i32 = 1;

    let mut signals: Vec<i32> = vec![];

    let measures = vec![20, 60, 100, 140, 180, 220];

    for instruction in instructions {
        let cycles_to_add = match instruction {
            Instruction::Add(_) => 2,
            Instruction::Noop => 1
        };

        if measures.contains(&(cycle + 1)) {
            signals.push((cycle + 1) * v);
        }

        match instruction {
            Instruction::Add(amount) => {
                if measures.contains(&(cycle + 2)) {
                    signals.push((cycle + 2) * v);
                }
                v += amount;
            }
            Instruction::Noop => {}
        };
        cycle += cycles_to_add;
    }


    return signals.iter().sum::<i32>().to_string();
}

pub fn run_part_2(input: &str) -> String {
    let (i, instructions) = lines(input).unwrap();

    let mut cycle: i32 = 0;
    let mut sprite_pos: i32 = 1;

    let mut output = vec![];
    for instruction in instructions {
        let cycles_to_add = match instruction {
            Instruction::Add(_) => 2,
            Instruction::Noop => 1
        };
        for x in 0..cycles_to_add {
            let normalized_cycles = cycle.rem_euclid(40);

            if normalized_cycles == sprite_pos - 1 || normalized_cycles == sprite_pos || normalized_cycles == sprite_pos + 1 {
                output.push('#');
            } else {
                output.push('.');
            }
            cycle += 1;
        }

        match instruction {
            Instruction::Add(amount) => {
                sprite_pos += amount;
            }
            Instruction::Noop => {}
        };
    }

    for y in 0..8 {
        println!();
        for i in 0..40 {
            print!("{}", output.get(i + y * 40).unwrap());
        }
    }

    return "1".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run_part_1("addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop");
        assert_eq!(result, "13140");
    }

    #[test]
    fn it_works2() {
        let result = run_part_2("addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop");
        assert_eq!(result, "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....");
    }
}
