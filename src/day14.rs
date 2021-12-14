use itertools::Itertools;
use std::collections::HashMap;

struct Template {
    init: String,
    templates: HashMap<(char, char), char>,
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Template {
    let input: Vec<&str> = input.split("\n\n").collect();
    let init = input[0].to_string();
    let templates = input[1]
        .lines()
        .map(|t| {
            let line: Vec<&str> = t.split(" -> ").collect();
            let input: Vec<char> = line[0].chars().collect();
            let output: char = line[1].chars().next().unwrap();
            ((input[0], input[1]), output)
        })
        .collect();
    Template { init, templates }
}

#[aoc(day14, part1)]
fn part1(input: &Template) -> usize {
    solve(&input.init, &input.templates, 10)
}

#[aoc(day14, part2)]
fn part2(input: &Template) -> usize {
    solve(&input.init, &input.templates, 40)
}

fn solve(input: &str, templates: &HashMap<(char, char), char>, step: usize) -> usize {
    let first = input.chars().tuple_windows().counts();
    let maps = std::iter::successors(Some(first), |pairs: &HashMap<(char, char), usize>| {
        let new_pairs = pairs
            .iter()
            .flat_map(|(&(a, b), &count)| {
                [
                    ((a, templates[&(a, b)]), count),
                    ((templates[&(a, b)], b), count),
                ]
            })
            .into_grouping_map()
            .sum();
        Some(new_pairs)
    })
    .skip(step)
    .next()
    .expect("Failed to compute");
    let map = maps
        .iter()
        .map(|(&(a, _), &c)| (a, c))
        .into_grouping_map()
        .sum();
    map.values().max().unwrap() - map.values().min().unwrap() + 1
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn should_work() {
        let data = parse(INPUT);
        assert_eq!(part1(&data), 1588);
        assert_eq!(part2(&data), 2188189693529);
    }
}
