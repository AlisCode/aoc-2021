use std::collections::HashSet;

struct OctopusMap {
    octopuses: Vec<Vec<u8>>,
}

fn all_positions() -> impl Iterator<Item = (i32, i32)> {
    (0..10).flat_map(|x| (0..10).map(move |y| (x, y)))
}

fn neighbors(pos: (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
    (pos.0 - 1..=pos.0 + 1)
        .flat_map(move |x| (pos.1 - 1..=pos.1 + 1).map(move |y| (x, y)))
        .filter(move |(x, y)| {
            !(*x == pos.0 && *y == pos.1) && (*x >= 0 && *x <= 9) && (*y >= 0 && *y <= 9)
        })
}

impl OctopusMap {
    pub fn step(&mut self) -> usize {
        for l in &mut self.octopuses {
            for o in l {
                *o += 1;
            }
        }
        let mut flashes = HashSet::default();
        loop {
            let flashes_len = flashes.len();
            self.flashes(&mut flashes);
            if flashes.len() == flashes_len {
                break;
            }
        }
        for l in &mut self.octopuses {
            for o in l {
                if *o >= 10 {
                    *o = 0;
                }
            }
        }
        flashes.len()
    }

    pub fn flashes(&mut self, flashes: &mut HashSet<(i32, i32)>) {
        for pos in all_positions() {
            if flashes.contains(&pos) {
                continue;
            }
            let octopus = self.octopuses[pos.1 as usize][pos.0 as usize];
            if octopus >= 10 {
                flashes.insert(pos);
                neighbors(pos).for_each(|p| self.octopuses[p.1 as usize][p.0 as usize] += 1);
            }
        }
    }
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

#[aoc(day11, part1)]
fn part1(input: &Vec<Vec<u8>>) -> usize {
    let mut map = OctopusMap {
        octopuses: input.clone(),
    };
    (0..100).map(|_| map.step()).sum()
}

#[aoc(day11, part2)]
fn part2(input: &Vec<Vec<u8>>) -> usize {
    let mut map = OctopusMap {
        octopuses: input.clone(),
    };
    (1..).find(|_| map.step() == 100).unwrap()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn should_work() {
        let data = parse(INPUT);
        assert_eq!(part1(&data), 1656);
        assert_eq!(part2(&data), 195);
    }
}
