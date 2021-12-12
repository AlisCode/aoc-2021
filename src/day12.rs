use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Cave {
    Big(String),
    Small(String),
}

impl FromStr for Cave {
    type Err = std::convert::Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.chars().next().unwrap().is_lowercase() {
            Ok(Cave::Small(input.to_string()))
        } else {
            Ok(Cave::Big(input.to_string()))
        }
    }
}

pub struct CaveMap {
    links: HashMap<Cave, HashSet<Cave>>,
}

impl CaveMap {
    fn non_visited_neighbors(
        &self,
        visited: &HashSet<Cave>,
        visit_flag: bool,
        cave: &Cave,
    ) -> HashSet<Cave> {
        if visit_flag {
            return self.links[cave].clone();
        }
        &self.links[cave] - visited
    }
}

#[aoc_generator(day12)]
fn parse(input: &str) -> CaveMap {
    let mut links = HashMap::<Cave, HashSet<Cave>>::default();
    let start = Cave::Small("start".to_string());
    input.lines().for_each(|l| {
        let caves: Vec<Cave> = l.split("-").map(|c| c.parse().unwrap()).collect();

        if caves[1] != start {
            let cave_0 = links.entry(caves[0].clone()).or_default();
            cave_0.insert(caves[1].clone());
        }

        if caves[0] != start {
            let cave_1 = links.entry(caves[1].clone()).or_default();
            cave_1.insert(caves[0].clone());
        }
    });
    CaveMap { links }
}

#[derive(Debug)]
pub struct CavePath {
    head: Cave,
    closed: HashSet<Cave>,
    can_visit_twice: bool,
}

#[aoc(day12, part1)]
fn part1(input: &CaveMap) -> usize {
    let mut open: Vec<CavePath> = Vec::new();
    let mut total_path = 0;

    open.push(CavePath {
        head: Cave::Small("start".to_string()),
        closed: HashSet::default(),
        can_visit_twice: true,
    });
    loop {
        if open.len() == 0 {
            break;
        }
        let CavePath {
            head,
            mut closed,
            can_visit_twice,
        } = open.pop().unwrap();
        match head {
            Cave::Small(x) if x == "end" => {
                total_path += 1;
                continue;
            }
            Cave::Small(_) => {
                closed.insert(head.clone());
            }
            Cave::Big(_) => (),
        }
        input
            .non_visited_neighbors(&closed, false, &head)
            .into_iter()
            .for_each(|c| {
                open.push(CavePath {
                    head: c,
                    closed: closed.clone(),
                    can_visit_twice,
                })
            });
    }

    total_path
}

#[aoc(day12, part2)]
fn part2(input: &CaveMap) -> usize {
    let mut open: Vec<CavePath> = Vec::new();
    let mut total_path = 0;
    let closed = HashSet::default();
    open.push(CavePath {
        head: Cave::Small("start".to_string()),
        closed,
        can_visit_twice: true,
    });
    loop {
        if open.len() == 0 {
            break;
        }
        let CavePath {
            head,
            mut closed,
            can_visit_twice,
        } = open.pop().unwrap();
        match head {
            Cave::Small(x) if x == "end" => {
                total_path += 1;
                continue;
            }
            Cave::Small(_) => {
                closed.insert(head.clone());
            }
            Cave::Big(_) => (),
        }
        input
            .non_visited_neighbors(&closed, can_visit_twice, &head)
            .into_iter()
            .for_each(|c| {
                let can_visit_twice = can_visit_twice && !closed.contains(&c);
                open.push(CavePath {
                    head: c,
                    closed: closed.clone(),
                    can_visit_twice,
                })
            });
    }

    total_path
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const INPUT_B: &'static str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    #[test]
    fn should_work() {
        let data = parse(INPUT);
        assert_eq!(part1(&data), 10);
        assert_eq!(part2(&data), 36);

        let data = parse(INPUT_B);
        assert_eq!(part2(&data), 103);
    }
}
