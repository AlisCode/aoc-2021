use std::{collections::HashSet, str::FromStr};

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

enum FoldInstruction {
    X(i32),
    Y(i32),
}

impl FromStr for FoldInstruction {
    type Err = std::convert::Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.replace("fold along ", "");
        let input: Vec<&str> = input.split("=").collect();
        let value = input[1].parse().unwrap();
        match input[0] {
            "x" => Ok(FoldInstruction::X(value)),
            "y" => Ok(FoldInstruction::Y(value)),
            _ => panic!("Unknown axis"),
        }
    }
}

impl FromStr for Point {
    type Err = std::convert::Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input: Vec<&str> = input.split(",").collect();
        Ok(Point {
            x: input[0].parse().unwrap(),
            y: input[1].parse().unwrap(),
        })
    }
}

struct Paper {
    points: Vec<Point>,
    instructions: Vec<FoldInstruction>,
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Paper {
    let input: Vec<&str> = input.split("\n\n").collect();

    let points = input[0].lines().map(|l| l.parse().unwrap()).collect();
    let instructions = input[1].lines().map(|l| l.parse().unwrap()).collect();

    Paper {
        points,
        instructions,
    }
}

#[aoc(day13, part1)]
fn part1(input: &Paper) -> usize {
    let mut points = input.points.clone();
    do_fold(&mut points, &input.instructions[0]);
    points
        .iter()
        .filter_map(|p| {
            if p.x >= 0 && p.y >= 0 {
                Some((p.x, p.y))
            } else {
                None
            }
        })
        .collect::<HashSet<_>>()
        .len()
}

#[aoc(day13, part2)]
fn part2(input: &Paper) -> String {
    let mut points = input.points.clone();
    input.instructions.iter().for_each(|instr| {
        do_fold(&mut points, instr);
    });
    display_code(&points)
}

fn display_code(points: &Vec<Point>) -> String {
    let (max_x, max_y) = points
        .iter()
        .fold((0, 0), |max, p| (max.0.max(p.x), max.1.max(p.y)));
    let points = points.iter().map(|p| (p.x, p.y)).collect::<HashSet<_>>();
    let mut output = String::new();
    for y in 0..=max_y {
        output += "\n";
        for x in 0..=max_x {
            if points.contains(&(x, y)) {
                output += "█";
            } else {
                output += " ";
            }
        }
    }
    output
}

fn do_fold(points: &mut Vec<Point>, fold: &FoldInstruction) {
    for p in points.iter_mut().filter(|p| match fold {
        FoldInstruction::X(i) => p.x > *i,
        FoldInstruction::Y(i) => p.y > *i,
    }) {
        match fold {
            FoldInstruction::X(i) => p.x = i - (p.x - i),
            FoldInstruction::Y(i) => p.y = i - (p.y - i),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    const OUTPUT_PART_2: &'static str = "
█████
█   █
█   █
█   █
█████";

    #[test]
    fn should_work() {
        let data = parse(INPUT);
        assert_eq!(part1(&data), 17);
        assert_eq!(part2(&data), OUTPUT_PART_2);
    }
}
