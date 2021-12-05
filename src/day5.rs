use std::collections::HashMap;
use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, Clone)]
struct LineSegment {
    pub p1: (i32, i32),
    pub p2: (i32, i32),
}

impl LineSegment {
    fn is_horizontal_or_vertical(&self) -> bool {
        self.p1.0 == self.p2.0 || self.p1.1 == self.p2.1
    }

    fn is_diagonal(&self) -> bool {
        let abs_x = (self.p1.0 - self.p2.0).abs();
        let abs_y = (self.p1.1 - self.p2.1).abs();
        abs_x == abs_y
    }
}

impl FromStr for LineSegment {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let points: Vec<&str> = input.split(" -> ").collect();

        let p1: Vec<i32> = points[0]
            .split(",")
            .map(|x| x.parse())
            .collect::<Result<_, _>>()?;
        let p1 = (p1[0], p1[1]);

        let p2: Vec<i32> = points[1]
            .split(",")
            .map(|x| x.parse())
            .collect::<Result<_, _>>()?;
        let p2 = (p2[0], p2[1]);

        Ok(LineSegment { p1, p2 })
    }
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Result<Vec<LineSegment>, ParseIntError> {
    input.lines().map(|x| x.parse()).collect()
}

#[aoc(day5, part1)]
fn part1(input: &[LineSegment]) -> usize {
    let mut lines_map: HashMap<(i32, i32), usize> = HashMap::default();
    input
        .iter()
        .filter(|s| s.is_horizontal_or_vertical())
        .for_each(|s| {
            if s.is_horizontal_or_vertical() {
                let min_x = s.p1.0.min(s.p2.0);
                let max_x = s.p1.0.max(s.p2.0);
                let min_y = s.p1.1.min(s.p2.1);
                let max_y = s.p1.1.max(s.p2.1);
                for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        let entry = lines_map.entry((x, y)).or_insert(0);
                        *entry += 1;
                    }
                }
            }
        });

    lines_map.values().filter(|x| **x >= 2).count()
}

#[aoc(day5, part2)]
fn part2(input: &[LineSegment]) -> usize {
    let mut lines_map: HashMap<(i32, i32), usize> = HashMap::default();
    input.iter().for_each(|s| {
        if s.is_horizontal_or_vertical() {
            let min_x = s.p1.0.min(s.p2.0);
            let max_x = s.p1.0.max(s.p2.0);
            let min_y = s.p1.1.min(s.p2.1);
            let max_y = s.p1.1.max(s.p2.1);
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    let entry = lines_map.entry((x, y)).or_insert(0);
                    *entry += 1;
                }
            }
        } else if s.is_diagonal() {
            let minx_point = if s.p1.0 < s.p2.0 { s.p1 } else { s.p2 };
            let maxx_point = if s.p1.0 < s.p2.0 { s.p2 } else { s.p1 };
            let delta_y = if maxx_point.1 - minx_point.1 > 0 {
                1
            } else {
                -1
            };
            let mut y = minx_point.1;
            for x in minx_point.0..=maxx_point.0 {
                let entry = lines_map.entry((x, y)).or_insert(0);
                *entry += 1;
                y += delta_y;
            }
        }
    });

    lines_map.values().filter(|x| **x >= 2).count()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn should_work() -> Result<(), ParseIntError> {
        let input = parse(INPUT)?;
        assert_eq!(part1(&input), 5);
        assert_eq!(part2(&input), 12);
        Ok(())
    }
}
