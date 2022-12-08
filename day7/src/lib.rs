use std::collections::{BTreeMap, HashMap};
use nom::{bytes::complete::tag, multi::many1, character::complete::{anychar, one_of}, multi::{many0, separated_list1}, sequence::{delimited, pair, terminated}, IResult};
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::character::complete::alpha1;
use nom::number::complete;
use nom::sequence::{preceded, separated_pair};

#[derive(Debug)]
enum Commands {
    CdDown(String),
    CdUp,
    Ls,
    Dir(String),
    File(u32, String),
}

fn filename(input: &str) -> IResult<&str, String> {
    let (input, dir) = is_a("abcdefghijklmnopqrstuvwxyz.-_")(input)?;

    return Ok((input, dir.to_string()));
}

fn dirname(input: &str) -> IResult<&str, String> {
    let (input, dir) = is_a("abcdefghijklmnopqrstuvwxyz")(input)?;

    return Ok((input, dir.to_string()));
}

fn cd_down(input: &str) -> IResult<&str, Commands> {
    let (input, dir) = preceded(tag("$ cd "), dirname)(input)?;

    return Ok((input, Commands::CdDown(dir.to_string())));
}

fn cd_up(input: &str) -> IResult<&str, Commands> {
    let (input, dir) = preceded(tag("$ cd "), tag(".."))(input)?;


    return Ok((input, Commands::CdUp));
}

fn ls(input: &str) -> IResult<&str, Commands> {
    let (input, dir) = tag("$ ls")(input)?;

    return Ok((input, Commands::Ls));
}

fn dir(input: &str) -> IResult<&str, Commands> {
    let (input, (_, dir)) = pair(tag("dir "), filename)(input)?;

    return Ok((input, Commands::Dir(dir.to_string())));
}

fn file(input: &str) -> IResult<&str, Commands> {
    let (input, (size, name)) = separated_pair(nom::character::complete::u32, tag(" "), filename)(input)?;

    return Ok((input, Commands::File(size, name.to_string())));
}


fn row(input: &str) -> IResult<&str, Commands> {
    let (input, command) = alt((cd_down, cd_up, ls, dir, file))(input)?;

    Ok((input, command))
}

fn rows(input: &str) -> IResult<&str, Vec<Commands>> {
    let (input, r) = many1(terminated(row, tag("\n")))(input)?;

    Ok((input, r))
}


pub fn run_part_1(input: &str) -> String {
    let (_, commands) = rows(input).unwrap();

    let mut tree: BTreeMap<String, HashMap<String, u32>> = BTreeMap::new();
    dbg!(&commands);
    let mut path: String = "".to_string();
    for command in commands.iter() {
        match command {
            Commands::CdDown(dir) => {
                path = format!("{}/{}", path, dir);
                println!("{}", &path);
            }
            Commands::CdUp => {
                let mut b: Vec<&str> = path.split("/").collect();
                b.pop();
                path = format!("{}", b.join("/"));
                println!("{}", &path);
            }
            Commands::Ls => {
                println!("LS!!!!")
            }
            Commands::Dir(name) => {
                tree.entry(path.clone()).or_insert(HashMap::new());
            }

            Commands::File(size, name) => {
                println!("{}", name);
                let mut a = tree.entry(path.clone()).or_insert(HashMap::new());
                a.insert(name.clone(), *size);
            }
        }
    }

    let sizes: BTreeMap<String, u32> = tree.iter().map(|(a, b)| {
        (a.clone(), b.values().sum())
    }).collect();

    dbg!(&sizes);

    let a: Vec<u32> = sizes.iter().map(|(k, size)| (k.to_string(), get_size(k, &sizes))).filter(|(k, s)| *s < 100000).inspect(|a| { dbg!(a); }).map(|(k, v)| v).collect();
    return a.iter().sum::<u32>().to_string();
}

fn get_size(key: &str, bt: &BTreeMap<String, u32>) -> u32 {
    return bt.iter().filter(|(k, v)| { k.starts_with(key) }).map(|(k, v)| { v }).sum();
}

pub fn run_part_2(input: &str) -> String {
    let (_, commands) = rows(input).unwrap();

    let mut tree: BTreeMap<String, HashMap<String, u32>> = BTreeMap::new();
    dbg!(&commands);
    let mut path: String = "".to_string();
    for command in commands.iter() {
        match command {
            Commands::CdDown(dir) => {
                path = format!("{}/{}", path, dir);
                println!("{}", &path);
            }
            Commands::CdUp => {
                let mut b: Vec<&str> = path.split("/").collect();
                b.pop();
                path = format!("{}", b.join("/"));
                println!("{}", &path);
            }
            Commands::Ls => {
                println!("LS!!!!")
            }
            Commands::Dir(name) => {
                tree.entry(path.clone()).or_insert(HashMap::new());
            }

            Commands::File(size, name) => {
                println!("{}", name);
                let mut a = tree.entry(path.clone()).or_insert(HashMap::new());
                a.insert(name.clone(), *size);
            }
        }
    }

    let sizes: BTreeMap<String, u32> = tree.iter().map(|(a, b)| {
        (a.clone(), b.values().sum())
    }).collect();

    let size_usd = get_size("", &sizes);
    let total_size = 70000000;
    let size_needed = 30000000;
    let size_left = total_size - size_usd;
    let size_needed_to_be_freed = size_needed - size_left;
    dbg!(size_needed_to_be_freed);

    let mut a: Vec<u32>= sizes.iter().map(|(k, size)| (k.to_string(), get_size(k, &sizes))).filter(|(k, s)| *s > size_needed_to_be_freed).inspect(|a| { dbg!(a); }).map(|(k, v)| v).collect();

    a.sort();
    dbg!(&a);
    return a.iter().sum::<u32>().to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";


        let result = run_part_1(input);
        assert_eq!(result, "95437");
    }
}
