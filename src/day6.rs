use std::num::ParseIntError;

#[aoc_generator(day6)]
fn parse(input: &str) -> Result<Vec<usize>, ParseIntError> {
    input.split(",").map(|x| x.parse()).collect()
}

#[aoc(day6, part1)]
fn part1(input: &[usize]) -> usize {
    fishes_for_day(input, 80)
}

#[aoc(day6, part2)]
fn part2(input: &[usize]) -> usize {
    fishes_for_day(input, 256)
}

fn fishes_for_day(input: &[usize], days: usize) -> usize {
    let mut fishes: [usize; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    input.iter().for_each(|x| fishes[*x] += 1);
    (0..days).for_each(|_| {
        let newborns = fishes[0];
        (0..=7).for_each(|i| fishes[i] = fishes[i + 1]);
        fishes[6] += newborns;
        fishes[8] = newborns;
    });
    fishes.into_iter().sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "3,4,3,1,2";

    #[test]
    fn should_work() -> Result<(), ParseIntError> {
        let data = parse(INPUT)?;
        assert_eq!(part1(&data), 5934);
        assert_eq!(part2(&data), 26984457539);
        Ok(())
    }
}
