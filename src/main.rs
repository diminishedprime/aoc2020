mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

fn main() -> Result<(), String> {
    println!("Day 01");
    day_01::part1()?;
    day_01::part2()?;
    println!();

    println!("Day 02");
    day_02::part1()?;
    day_02::part2()?;
    println!();

    println!("Day 03");
    day_03::part1()?;
    day_03::part2()?;
    println!();

    println!("Day 04");
    day_04::part1()?;
    day_04::part2()?;
    println!();

    println!("Day 05");
    day_05::part1()?;
    day_05::part2()?;
    println!();

    Ok(())
}
