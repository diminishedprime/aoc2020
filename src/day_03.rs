use std::fs;

struct Heading {
    horizontal: usize,
    verticle: usize,
}

impl Heading {
    fn from(horizontal: usize, verticle: usize) -> Self {
        Self {
            horizontal,
            verticle,
        }
    }
}

pub fn part1() -> Result<(), String> {
    let input =
        fs::read_to_string("src/day_03_input.txt").map_err(|_| String::from("bad path, buddy."))?;
    let collisions = part1_solution(
        &input.trim(),
        Heading {
            horizontal: 3,
            verticle: 1,
        },
    )?;
    println!("Part 1: {}", collisions);
    Ok(())
}

fn part1_solution(input: &str, heading: Heading) -> Result<usize, String> {
    let mut lines = input.split("\n");
    let mut collisions = 0;

    // Always skip the first entry
    lines.next();

    let mut horizontal_idx = 0;
    while let Some(current) = lines.nth(heading.verticle - 1) {
        horizontal_idx = (horizontal_idx + heading.horizontal) % current.len();
        if current.chars().nth(horizontal_idx) == Some('#') {
            collisions += 1;
        }
    }

    Ok(collisions)
}

pub fn part2() -> Result<(), String> {
    let input =
        fs::read_to_string("src/day_03_input.txt").map_err(|_| String::from("bad path, buddy."))?;
    let headings = vec![
        Heading::from(1, 1),
        Heading::from(3, 1),
        Heading::from(5, 1),
        Heading::from(7, 1),
        Heading::from(1, 2),
    ];
    let mut product = 1;
    for heading in headings.into_iter() {
        product = product * part1_solution(&input.trim(), heading)?;
    }
    println!("Part 1: {}", product);
    Ok(())
}

#[test]
fn part1_test_input() -> Result<(), String> {
    let input = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#\
";
    let actual = part1_solution(
        &input,
        Heading {
            horizontal: 3,
            verticle: 1,
        },
    )?;
    assert_eq!(actual, 7);
    Ok(())
}
