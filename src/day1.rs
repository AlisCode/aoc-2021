#[aoc_generator(day1)]
fn ints(input: &str) -> Vec<u32> {
    input.lines().filter_map(|dep| dep.parse().ok()).collect()
}

#[aoc(day1, part1)]
pub fn part1(depths: &[u32]) -> usize {
    depths.windows(2).filter(|w| w[0] < w[1]).count()
}

#[aoc(day1, part2)]
pub fn part2(depths: &[u32]) -> usize {
    let depth_windows: Vec<u32> = depths.windows(3).map(|w| w.iter().sum()).collect();
    depth_windows.windows(2).filter(|w| w[0] < w[1]).count()
}

#[cfg(test)]
pub mod tests {

    const INPUT: &'static str = "199
200
208
210
200
207
240
269
260
263";

    use super::{ints, part1, part2};

    #[test]
    fn solve_day_1() {
        let input = ints(INPUT);
        assert_eq!(part1(&input), 7);
        assert_eq!(part2(&input), 5);
    }
}
