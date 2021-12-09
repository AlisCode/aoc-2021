use std::collections::HashMap;

struct Note {
    inputs: Vec<String>,
    outputs: Vec<String>,
}

#[derive(Debug)]
struct Mappings(HashMap<char, usize>);

impl Note {
    fn compute_mappings(&self) -> Mappings {
        Mappings(
            "abcdefg"
                .chars()
                .map(|x| {
                    (
                        x,
                        self.inputs
                            .iter()
                            .map(|i| i.chars().filter(|c| x == *c).count())
                            .sum(),
                    )
                })
                .collect(),
        )
    }
}

impl Mappings {
    pub fn get_digit(&self, input: &str) -> char {
        let val: usize = input.chars().map(|x| self.0[&x]).sum();
        match val {
            42 => '0',
            17 => '1',
            34 => '2',
            39 => '3',
            30 => '4',
            37 => '5',
            41 => '6',
            25 => '7',
            49 => '8',
            45 => '9',
            _ => panic!("Unknown digit"),
        }
    }
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<Note> {
    input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(" | ").collect();
            let inputs = parts[0].split(" ").map(|x| x.to_string()).collect();
            let outputs = parts[1].split(" ").map(|x| x.to_string()).collect();
            Note { inputs, outputs }
        })
        .collect()
}

#[aoc(day8, part1)]
fn part1(input: &[Note]) -> usize {
    input
        .iter()
        .map(|i| {
            i.outputs
                .iter()
                .filter(|i| match i.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .count()
        })
        .sum()
}

#[aoc(day8, part2)]
fn part2(input: &[Note]) -> u32 {
    input
        .iter()
        .map(|note| {
            let mappings = note.compute_mappings();
            note.outputs
                .iter()
                .map(|o| mappings.get_digit(o))
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn should_work() {
        let data = parse(INPUT);
        assert_eq!(part1(&data), 26);
        assert_eq!(part2(&data), 61229);
    }
}
