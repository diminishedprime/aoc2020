use std::fs;

pub fn part1() -> Result<(), String> {
    let input =
        fs::read_to_string("src/day_01_input.txt").map_err(|_| String::from("bad path, buddy."))?;
    let product = part1_solution(&input)?;
    println!("part 1: {}", product);
    Ok(())
}

pub fn part2() -> Result<(), String> {
    let input =
        fs::read_to_string("src/day_01_input.txt").map_err(|_| String::from("bad path, buddy."))?;
    let product = part2_solution(&input)?;
    println!("part 2: {}", product);
    Ok(())
}

pub fn part1_solution(input: &str) -> Result<usize, String> {
    let numbers = input
        .trim()
        .split("\n")
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| String::from("Why you giving me not ints, buddy?"))?;
    let numbers = numbers.iter();

    for (idx, first) in numbers.clone().enumerate() {
        for second in numbers.clone().skip(idx + 1) {
            if first + second == 2020 {
                return Ok(first * second);
            }
        }
    }
    Err(String::from("No solution was found for the given input"))
}

pub fn part2_solution(input: &str) -> Result<usize, String> {
    let numbers = input
        .trim()
        .split("\n")
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| String::from("Why you giving me not ints, buddy?"))?;
    let numbers = numbers.iter();

    for (idx, first) in numbers.clone().enumerate() {
        let numbers = numbers.clone().enumerate().skip(idx + 1);
        for (idx, second) in numbers.clone() {
            for (_, third) in numbers.clone().skip(idx + 1) {
                if first + second + third == 2020 {
                    return Ok(first * second * third);
                }
            }
        }
    }
    Err(String::from("No solution was found for the given input"))
}

#[test]
fn part1_test_input() -> Result<(), String> {
    let input = "\
1721
979
366
299
675
1456\
";
    let solution = part1_solution(&input)?;
    assert_eq!(solution, 514579);
    Ok(())
}
