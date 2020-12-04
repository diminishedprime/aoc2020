mod day_01;
mod day_02;

fn main() -> Result<(), String> {
    day_01::part1()?;
    day_01::part2()?;

    day_02::part1()?;
    day_02::part2()?;

    Ok(())
}
