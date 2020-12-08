use std::fs;

use nom::{
    bytes::complete::take_while_m_n,
    combinator::{all_consuming, map, map_opt},
    sequence::tuple,
    IResult,
};

trait CanBinary: Sized {
    fn from_char(c: char) -> Option<char>;
    fn from_str(s: &str) -> Option<usize> {
        s.chars()
            .map(Self::from_char)
            .collect::<Option<String>>()
            .and_then(|s| usize::from_str_radix(&s, 2).ok())
    }
}

struct BinaryFB;

impl CanBinary for BinaryFB {
    fn from_char(c: char) -> Option<char> {
        Some(match c {
            'F' => '0',
            'B' => '1',
            _ => return None,
        })
    }
}

struct BinaryRL;

impl CanBinary for BinaryRL {
    fn from_char(c: char) -> Option<char> {
        Some(match c {
            'R' => '1',
            'L' => '0',
            _ => return None,
        })
    }
}

fn bad_binary<T: CanBinary>(input: &str, length: usize) -> IResult<&str, usize> {
    map_opt(
        take_while_m_n(length, length, |c: char| T::from_char(c).is_some()),
        T::from_str,
    )(input)
}

fn row(input: &str) -> IResult<&str, usize> {
    bad_binary::<BinaryFB>(input, 7)
}

fn column(input: &str) -> IResult<&str, usize> {
    bad_binary::<BinaryRL>(input, 3)
}

#[derive(PartialEq, Debug)]
struct Seat {
    row: usize,
    column: usize,
}

impl Seat {
    fn id(&self) -> usize {
        self.row * 8 + self.column
    }
}

fn seat(input: &str) -> Result<Seat, String> {
    let (_, seat) = all_consuming(map(tuple((row, column)), |(row, column)| Seat {
        row,
        column,
    }))(input)
    .map_err(|s| format!("{}", s))?;
    Ok(seat)
}

#[test]
fn parse_seat() -> Result<(), String> {
    let input = "FBFBBFFRLR";
    let s = seat(input)?;
    assert_eq!(s, Seat { row: 44, column: 5 });
    assert_eq!(s.id(), 357);

    let input = "BFFFBBFRRR";
    let s = seat(input)?;
    assert_eq!(s, Seat { row: 70, column: 7 });
    assert_eq!(s.id(), 567);

    let input = "FFFBBBFRRR";
    let s = seat(input)?;
    assert_eq!(s, Seat { row: 14, column: 7 });
    assert_eq!(s.id(), 119);

    let input = "BBFFBBFRLL";
    let s = seat(input)?;
    assert_eq!(
        s,
        Seat {
            row: 102,
            column: 4
        }
    );
    assert_eq!(s.id(), 820);

    Ok(())
}

pub fn part1() -> Result<(), String> {
    let input =
        fs::read_to_string("src/day_05_input.txt").map_err(|_| String::from("bad path, buddy."))?;
    let largest_id = input
        .trim()
        .split("\n")
        .map(seat)
        .fold(Ok(0 as usize), |a, b| {
            b.map_err(|e| format!("{}", e))
                .and_then(|b| Ok(b.id().max(a?)))
        })?;
    println!("Part 1: {}", largest_id);
    Ok(())
}

pub fn part2() -> Result<(), String> {
    let input =
        fs::read_to_string("src/day_05_input.txt").map_err(|_| String::from("bad path, buddy."))?;
    let mut seats = input
        .trim()
        .split("\n")
        .map(seat)
        .collect::<Result<Vec<_>, String>>()?;
    seats.sort_by_key(|s| s.id());
    let mut seats = seats.iter();
    let mut your_id = seats
        .next()
        .map(Seat::id)
        .ok_or(String::from("There's a goof in here"))?;
    for seat in seats {
        let this_seat = seat.id();
        if your_id + 1 != this_seat {
            your_id = your_id + 1;
            break;
        }
        your_id = this_seat
    }
    println!("Part 2: {}", your_id);
    Ok(())
}
