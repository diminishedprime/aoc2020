use std::{cell::RefCell, fs, rc::Rc};

use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::take_while_m_n,
    character::complete::alphanumeric1,
    character::complete::{digit1, multispace0},
    combinator::map,
    combinator::{all_consuming, map_res},
    multi::fold_many0,
    multi::fold_many1,
    sequence::tuple,
    IResult,
};

#[derive(Debug, Clone)]
struct Passport {
    original: String,
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn empty(original: &str) -> Self {
        Self {
            original: String::from(original),
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }
    fn is_valid(&self) -> bool {
        [
            &self.byr, &self.iyr, &self.eyr, &self.hgt, &self.hcl, &self.ecl, &self.pid,
        ]
        .iter()
        .all(|a| a.is_some())
    }
    fn is_valid_2(&self) -> bool {
        let temparray = [
            (self.byr.as_ref()).map(|byr: &String| {
                let parsed = byr.parse::<usize>().ok()?;
                if parsed >= 1920 && parsed <= 2002 {
                    Some(true)
                } else {
                    None
                }
            }),
            (self.iyr.as_ref()).map(|byr: &String| {
                let parsed = byr.parse::<usize>().ok()?;
                if parsed >= 2010 && parsed <= 2020 {
                    Some(true)
                } else {
                    None
                }
            }),
            (self.eyr.as_ref()).map(|byr: &String| {
                let parsed = byr.parse::<usize>().ok()?;
                if parsed >= 2020 && parsed <= 2030 {
                    Some(true)
                } else {
                    None
                }
            }),
            (self.hgt.as_deref()).map(|byr: &str| {
                let (_, h) = all_consuming(height)(byr).ok()?;
                if h.in_bounds() {
                    Some(true)
                } else {
                    None
                }
            }),
            (self.hcl.as_deref()).map(|byr: &str| {
                let (_, _) = hex(byr).ok()?;
                Some(true)
            }),
            (self.ecl.as_deref()).map(|byr: &str| {
                let (_, _) = eye_color(byr).ok()?;
                Some(true)
            }),
            (self.pid.as_deref()).map(|byr: &str| {
                if byr.len() == 9 && byr.parse::<u64>().is_ok() {
                    Some(true)
                } else {
                    None
                }
            }),
        ];
        temparray.iter().map(|a| a.unwrap()).all(|a| a.is_some())
    }
}

#[derive(Debug)]
enum Unit {
    Centimeter,
    Inch,
}

impl Unit {
    fn from(s: &str) -> Result<Unit, String> {
        Ok(match s {
            "cm" => Unit::Centimeter,
            "in" => Unit::Inch,
            _ => return Err(String::from("Not a valid unit")),
        })
    }
}

fn eye_color(input: &str) -> IResult<&str, &str> {
    alt((
        tag("amb"),
        tag("blu"),
        tag("brn"),
        tag("gry"),
        tag("grn"),
        tag("hzl"),
        tag("oth"),
    ))(input)
}

#[derive(Debug)]
struct Height {
    value: usize,
    unit: Unit,
}

impl Height {
    fn in_bounds(&self) -> bool {
        match self.unit {
            Unit::Centimeter => self.value >= 150 && self.value <= 193,
            Unit::Inch => self.value >= 59 && self.value <= 76,
        }
    }
}

fn height(input: &str) -> IResult<&str, Height> {
    map(
        tuple((
            map_res(digit1, |a: &str| a.parse::<usize>()),
            map_res(alt((tag("cm"), tag("in"))), Unit::from),
        )),
        |(value, unit)| Height { value, unit },
    )(input)
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit() && !c.is_ascii_uppercase()
}

fn hex(input: &str) -> IResult<&str, (&str, &str)> {
    all_consuming(tuple((tag("#"), (take_while_m_n)(6, 6, is_hex_digit))))(input)
}

#[test]
fn hex_works() {
    let (_, actual) = hex("#abcdef").unwrap();
    assert_eq!(actual, ("#", "abcdef"));
}

type PassportBuild = Rc<RefCell<Passport>>;

fn byr(passport: PassportBuild, input: &str) -> IResult<&str, PassportBuild> {
    map(tuple((tag("byr:"), alphanumeric1)), |(_, byr)| {
        passport.borrow_mut().byr = Some(String::from(byr));
        passport.clone()
    })(input)
}

fn iyr(passport: PassportBuild, input: &str) -> IResult<&str, PassportBuild> {
    map(tuple((tag("iyr:"), alphanumeric1)), |(_, iyr)| {
        passport.borrow_mut().iyr = Some(String::from(iyr));
        passport.clone()
    })(input)
}

fn eyr(passport: PassportBuild, input: &str) -> IResult<&str, PassportBuild> {
    map(tuple((tag("eyr:"), alphanumeric1)), |(_, eyr)| {
        passport.borrow_mut().eyr = Some(String::from(eyr));
        passport.clone()
    })(input)
}

fn hgt(passport: PassportBuild, input: &str) -> IResult<&str, PassportBuild> {
    map(tuple((tag("hgt:"), alphanumeric1)), |(_, hgt)| {
        passport.borrow_mut().hgt = Some(String::from(hgt));
        passport.clone()
    })(input)
}

fn hcl(passport: PassportBuild, input: &str) -> IResult<&str, PassportBuild> {
    map(
        tuple((
            tag("hcl:"),
            fold_many1(alt((alphanumeric1, tag("#"))), String::new(), |mut a, b| {
                a.push_str(b);
                a
            }),
        )),
        |(_, hcl)| {
            passport.borrow_mut().hcl = Some(String::from(hcl));
            passport.clone()
        },
    )(input)
}

fn ecl(passport: PassportBuild, input: &str) -> IResult<&str, PassportBuild> {
    map(
        tuple((
            tag("ecl:"),
            fold_many1(alt((alphanumeric1, tag("#"))), String::new(), |mut a, b| {
                a.push_str(b);
                a
            }),
        )),
        |(_, ecl)| {
            passport.borrow_mut().ecl = Some(String::from(ecl));
            passport.clone()
        },
    )(input)
}

fn pid(passport: PassportBuild, input: &str) -> IResult<&str, PassportBuild> {
    map(
        tuple((
            tag("pid:"),
            fold_many1(alt((alphanumeric1, tag("#"))), String::new(), |mut a, b| {
                a.push_str(b);
                a
            }),
        )),
        |(_, pid)| {
            passport.borrow_mut().pid = Some(String::from(pid));
            passport.clone()
        },
    )(input)
}

fn cid(passport: PassportBuild, input: &str) -> IResult<&str, PassportBuild> {
    map(tuple((tag("cid:"), alphanumeric1)), |(_, cid)| {
        passport.borrow_mut().cid = Some(String::from(cid));
        passport.clone()
    })(input)
}

fn other(passport: PassportBuild, input: &str) -> IResult<&str, PassportBuild> {
    map(
        tuple((alphanumeric1, tag(":"), alphanumeric1)),
        |(_, _, _)| passport.clone(),
    )(input)
}

fn acceptable_whitespace(input: &str) -> IResult<&str, ()> {
    map(multispace0, |_| ())(input)
}

fn passport<'a>(input: &'a str) -> Result<Passport, String> {
    let pp = Rc::new(RefCell::new(Passport::empty(&input)));
    fold_many0(
        map(
            tuple((
                alt((
                    |input: &'a str| byr(pp.clone(), &input),
                    |input: &'a str| iyr(pp.clone(), &input),
                    |input: &'a str| eyr(pp.clone(), &input),
                    |input: &'a str| hgt(pp.clone(), &input),
                    |input: &'a str| hcl(pp.clone(), &input),
                    |input: &'a str| ecl(pp.clone(), &input),
                    |input: &'a str| pid(pp.clone(), &input),
                    |input: &'a str| cid(pp.clone(), &input),
                    |input: &'a str| other(pp.clone(), &input),
                )),
                acceptable_whitespace,
            )),
            |(pp, _)| pp,
        ),
        pp.clone(),
        |a, _| a,
    )(input)
    .map_err(|e| format!("{:?}", e))?;
    Ok(Rc::try_unwrap(pp)
        .map_err(|_| String::from("Could not unwrap Rc"))?
        .into_inner())
}

#[test]
fn part2_bullshit_2() -> Result<(), String> {
    let input = "\
    pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719\
";

    let count = part2_solution(&input)?;
    assert_eq!(4, count);
    Ok(())
}

#[test]
fn part2_bullshit() -> Result<(), String> {
    let input = "\
    eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007\
";
    let count = part2_solution(&input)?;
    assert_eq!(0, count);
    Ok(())
}

#[test]
fn parse_pp() -> Result<(), String> {
    let input = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in\
";

    let count = part1_solution(&input)?;
    assert_eq!(count, 2);

    Ok(())
}

fn part1_solution(input: &str) -> Result<usize, String> {
    Ok(input
        .trim()
        .split("\n\n")
        .map(|entry: &str| {
            passport(entry).map_or_else(
                |e| {
                    println!("Invalid pp {:?}", e);
                    0
                },
                |a| if a.is_valid() { 1 } else { 0 },
            )
        })
        .fold(0, |a, b| a + b))
}

pub fn part1() -> Result<(), String> {
    let input =
        fs::read_to_string("src/day_04_input.txt").map_err(|_| String::from("bad path, buddy."))?;
    let count = part1_solution(&input.trim())?;
    println!("Part1: {}", count);
    Ok(())
}

fn part2_solution(input: &str) -> Result<usize, String> {
    Ok(input
        .trim()
        .split("\n\n")
        .map(|entry: &str| {
            passport(entry).map_or_else(
                |e| {
                    println!("Invalid pp {:?}", e);
                    0
                },
                |a| {
                    if a.is_valid() && a.is_valid_2() {
                        1
                    } else {
                        0
                    }
                },
            )
        })
        .fold(0, |a, b| a + b))
}
pub fn part2() -> Result<(), String> {
    let input =
        fs::read_to_string("src/day_04_input.txt").map_err(|_| String::from("bad path, buddy."))?;
    let count = part2_solution(&input.trim())?;
    println!("Part2: {} WRONG", count);
    Ok(())
}
