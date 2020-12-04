use std::fs;

use nom::{
    bytes::complete::tag,
    character::{complete::alphanumeric1, complete::anychar, complete::digit1},
    combinator::all_consuming,
    combinator::{map, map_res},
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
struct PasswordEntry<'a> {
    range: Range,
    required_letter: char,
    password: &'a str,
}

#[derive(Debug)]
struct Range {
    from: usize,
    to: usize,
}

fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |input: &str| input.parse::<usize>())(input)
}

fn range(input: &str) -> IResult<&str, Range> {
    let range = tuple((number, tag("-"), number));
    map(range, |(from, _, to)| Range { from, to })(input)
}

fn required_letter(input: &str) -> IResult<&str, char> {
    anychar(input)
}

fn password(input: &str) -> IResult<&str, &str> {
    alphanumeric1(input)
}

fn password_entry<'a>(input: &'a str) -> IResult<&str, PasswordEntry<'a>> {
    map(
        tuple((range, tag(" "), required_letter, tag(": "), password)),
        |(range, _, required_letter, _, password)| PasswordEntry {
            range,
            required_letter,
            password,
        },
    )(input)
}

fn validate_password_entry(password_entry: PasswordEntry) -> usize {
    let PasswordEntry {
        range: Range { from, to },
        required_letter,
        password,
    } = password_entry;
    let required_letter_count = password.chars().filter(|c| c == &required_letter).count();
    if required_letter_count >= from && required_letter_count <= to {
        1
    } else {
        0
    }
}

fn validate_password_entry_part_2(password_entry: PasswordEntry) -> usize {
    let PasswordEntry {
        range: Range { from, to },
        required_letter,
        password,
    } = password_entry;
    let chars = password.chars();
    let first = chars.clone().nth(from - 1);
    let second = chars.clone().nth(to - 1);
    let first = if first == Some(required_letter) { 0 } else { 1 };
    let second = if second == Some(required_letter) {
        0
    } else {
        1
    };
    first ^ second
}

pub fn part1_solution(input: &str) -> Result<usize, String> {
    Ok(input
        .split("\n")
        .map(|line| {
            let (_remainder, entry) =
                all_consuming(password_entry)(line).map_err(|e| format!("{:?}", e))?;
            Ok(validate_password_entry(entry))
        })
        .collect::<Result<Vec<usize>, String>>()?
        .iter()
        .fold(0, |a, b| a + b))
}

pub fn part1() -> Result<(), String> {
    let input =
        fs::read_to_string("src/day_02_input.txt").map_err(|_| String::from("bad path, buddy."))?;
    let valid_password_count = part1_solution(input.trim())?;
    println!("part 1: {}", valid_password_count);
    Ok(())
}

pub fn part2_solution(input: &str) -> Result<usize, String> {
    Ok(input
        .split("\n")
        .map(|line| {
            let (_remainder, entry) =
                all_consuming(password_entry)(line).map_err(|e| format!("{:?}", e))?;
            Ok(validate_password_entry_part_2(entry))
        })
        .collect::<Result<Vec<usize>, String>>()?
        .iter()
        .fold(0, |a, b| a + b))
}

pub fn part2() -> Result<(), String> {
    let input =
        fs::read_to_string("src/day_02_input.txt").map_err(|_| String::from("bad path, buddy."))?;
    let valid_password_count = part2_solution(input.trim())?;
    println!("part 2: {}", valid_password_count);
    Ok(())
}

#[test]
fn part1_test_input() -> Result<(), String> {
    let input = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc\
    ";
    let actual = part1_solution(input)?;
    assert_eq!(actual, 2);
    Ok(())
}
#[test]
fn part2_test_input() -> Result<(), String> {
    let input = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc\
    ";
    let actual = part2_solution(input)?;
    assert_eq!(actual, 1);
    Ok(())
}
