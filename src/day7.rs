use std::num::ParseIntError;

#[aoc_generator(day7)]
fn parse(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.split(",").map(|x| x.parse()).collect()
}

#[aoc(day7, part1)]
fn part1(input: &[i32]) -> i32 {
    let mut prev = i32::MAX;
    for x in 0.. {
        let new_fuel = input.iter().map(|x| (target - x).abs()).sum();
        if new_fuel > prev {
            return prev;
        }
        prev = new_fuel
    }

    unreachable!()
}

#[aoc(day7, part2)]
fn part2(input: &[i32]) -> i32 {
    let mut prev = i32::MAX;
    for x in 0.. {
        let new_fuel = input
            .iter()
            .map(|x| {
                let n = (target - x).abs();
                (n * (n + 1)) / 2
            })
            .sum();
        if new_fuel > prev {
            return prev;
        }
        prev = new_fuel
    }

    unreachable!()
}

fn calc_fuel(input: &[i32], target: i32) -> i32 {}

fn calc_fuel_second(input: &[i32], target: i32) -> i32 {}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::num::ParseIntError;

    const INPUT: &'static str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn should_work() -> Result<(), ParseIntError> {
        let data = parse(INPUT)?;
        assert_eq!(part1(&data), 37);
        assert_eq!(part2(&data), 168);
        Ok(())
    }
}
