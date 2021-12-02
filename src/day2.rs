use std::str::FromStr;
use thiserror::Error;

#[derive(Debug)]
enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

#[derive(Debug, Error)]
enum InstructionParseError {
    #[error("Wrong format")]
    WrongFormat(String),
    #[error("Wrong instruction {0}")]
    WrongInstruction(String),
    #[error("Failed to parse int")]
    ParseIntError(#[from] std::num::ParseIntError),
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let inst: Vec<&str> = input.split(" ").collect();
        if inst.len() != 2 {
            return Err(InstructionParseError::WrongFormat(input.to_string()));
        }
        match inst[0] {
            "forward" => Ok(Instruction::Forward(inst[1].parse()?)),
            "down" => Ok(Instruction::Down(inst[1].parse()?)),
            "up" => Ok(Instruction::Up(inst[1].parse()?)),
            x => Err(InstructionParseError::WrongInstruction(x.to_string())),
        }
    }
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Result<Vec<Instruction>, InstructionParseError> {
    input.lines().map(|x| Instruction::from_str(x)).collect()
}

#[aoc(day2, part1)]
fn part1(instructions: &[Instruction]) -> i32 {
    let mut x = 0;
    let mut y = 0;

    instructions.iter().for_each(|i| match i {
        Instruction::Forward(xx) => x += xx,
        Instruction::Up(yy) => y -= yy,
        Instruction::Down(yy) => y += yy,
    });

    x * y
}

#[aoc(day2, part2)]
fn part2(instructions: &[Instruction]) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    instructions.iter().for_each(|i| match i {
        Instruction::Forward(xx) => {
            x += xx;
            y += aim * xx;
        }
        Instruction::Up(yy) => {
            aim -= yy;
        }
        Instruction::Down(yy) => {
            aim += yy;
        }
    });

    x * y
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn should_move_correctly() -> Result<(), InstructionParseError> {
        let instructions = parse(INPUT)?;
        assert_eq!(part1(&instructions), 150);

        Ok(())
    }

    #[test]
    fn should_aim_correctly() -> Result<(), InstructionParseError> {
        let instructions = parse(INPUT)?;
        assert_eq!(part2(&instructions), 900);

        Ok(())
    }
}
