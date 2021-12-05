use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub struct BingoInfo {
    pub numbers: Vec<u32>,
    pub boards: Vec<BingoBoard>,
}

#[derive(Debug, Clone)]
pub struct BingoBoard {
    pub rows: Vec<Vec<Option<u32>>>,
    pub winner: bool,
}

impl BingoBoard {
    pub fn call(&mut self, number: u32) {
        for r in &mut self.rows {
            for v in r.iter_mut() {
                if let Some(x) = v {
                    if number == *x {
                        *v = None;
                    }
                }
            }
            if r.iter().all(|x| x.is_none()) {
                self.winner = true;
                return;
            }
        }

        let mut is_winner = false;
        for c in self.columns() {
            if c.iter().all(|x| x.is_none()) {
                is_winner = true;
            }
        }
        self.winner = is_winner;
    }

    pub fn columns(&self) -> impl Iterator<Item = [Option<u32>; 5]> + '_ {
        (0..5).map(|i| {
            [
                self.rows[0][i],
                self.rows[1][i],
                self.rows[2][i],
                self.rows[3][i],
                self.rows[4][i],
            ]
        })
    }

    pub fn calc_score(&self) -> u32 {
        self.rows
            .iter()
            .map(|r| r.iter().filter_map(|x| *x).sum::<u32>())
            .sum()
    }
}

impl FromStr for BingoBoard {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let rows: Vec<Vec<Option<u32>>> = input
            .lines()
            .map(|l| {
                l.split(" ")
                    // yes, clippy will complain about .filter(..).map(..) but this makes it better
                    // for error handling
                    .filter(|x| !x.is_empty())
                    .map(|x| Some(x.parse().unwrap()))
                    .collect()
            })
            .collect();
        Ok(BingoBoard {
            rows,
            winner: false,
        })
    }
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Result<BingoInfo, ParseIntError> {
    let lines: Vec<&str> = input.split("\n\n").collect();

    let numbers: Vec<u32> = lines[0]
        .split(",")
        .map(|x| x.parse())
        .collect::<Result<_, _>>()?;
    let boards: Vec<BingoBoard> = lines
        .iter()
        .skip(1)
        .map(|x| x.parse())
        .collect::<Result<_, _>>()?;

    Ok(BingoInfo { numbers, boards })
}

#[aoc(day4, part1)]
fn part1(input: &BingoInfo) -> u32 {
    let mut boards = input.boards.clone();

    for n in &input.numbers {
        for b in &mut boards {
            b.call(*n);
            if b.winner {
                return b.calc_score() * n;
            }
        }
    }

    panic!("should have returned a score");
}

#[aoc(day4, part2)]
fn part2(input: &BingoInfo) -> u32 {
    let mut boards = input.boards.clone();
    let mut remaining_boards = input.boards.len();

    for n in &input.numbers {
        for b in &mut boards {
            if b.winner {
                continue;
            }
            b.call(*n);
            if b.winner {
                remaining_boards -= 1;
                if remaining_boards == 0 {
                    return b.calc_score() * n;
                }
            }
        }
    }

    panic!("should have returned a score");
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::num::ParseIntError;

    const INPUT: &'static str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn should_parse() -> Result<(), ParseIntError> {
        let _bingo_info: BingoInfo = parse(INPUT)?;
        Ok(())
    }

    #[test]
    fn should_solve() -> Result<(), ParseIntError> {
        let bingo_info = parse(INPUT)?;
        assert_eq!(part1(&bingo_info), 4512);
        assert_eq!(part2(&bingo_info), 1924);
        Ok(())
    }
}
