struct ChitonsMap {
    chitons: Vec<Vec<i32>>,
    width: i32,
    height: i32,
}

impl ChitonsMap {
    pub fn neighbors(&self, pos: &(i32, i32)) -> impl Iterator<Item = ((i32, i32), i32)> + '_ {
        [
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 - 1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 + 1),
        ]
        .into_iter()
        .filter(|(x, y)| *x >= 0 && *x <= self.width && *y >= 0 && *y <= self.height)
        .map(|(x, y)| ((x, y), self.chitons[y as usize][x as usize]))
    }

    pub fn neighbors_two(&self, pos: &(i32, i32)) -> impl Iterator<Item = ((i32, i32), i32)> + '_ {
        [
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 - 1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 + 1),
        ]
        .into_iter()
        .filter(|(x, y)| {
            *x >= 0 && *x <= (self.width + 1) * 5 - 1 && *y >= 0 && *y <= (self.height + 1) * 5 - 1
        })
        .map(|(x, y)| {
            let mapped_x = x % (self.width + 1);
            let mapped_y = y % (self.height + 1);
            let ix = x / (self.width + 1);
            let iy = y / (self.height + 1);
            let risk = self.chitons[mapped_y as usize][mapped_x as usize] + ix + iy;
            let risk = risk % 10 + risk / 10;
            ((x, y), risk)
        })
    }
}

#[aoc_generator(day15)]
fn parse(input: &str) -> ChitonsMap {
    let chitons: Vec<Vec<i32>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect();
    let width = chitons[0].len() as i32 - 1;
    let height = chitons.len() as i32 - 1;
    ChitonsMap {
        chitons,
        width,
        height,
    }
}

#[aoc(day15, part1)]
fn part1(input: &ChitonsMap) -> i32 {
    let (_, cost) = pathfinding::directed::dijkstra::dijkstra(
        &(0, 0),
        |pos| input.neighbors(pos),
        |(x, y)| *x == input.width && *y == input.height,
    )
    .expect("Failed to find path");
    cost
}

#[aoc(day15, part2)]
fn part2(input: &ChitonsMap) -> i32 {
    let (_, cost) = pathfinding::directed::dijkstra::dijkstra(
        &(0, 0),
        |pos| input.neighbors_two(pos),
        |(x, y)| *x == (input.width + 1) * 5 - 1 && *y == (input.height + 1) * 5 - 1,
    )
    .expect("Failed to find path");
    cost
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn should_work() {
        let data = parse(INPUT);
        assert_eq!(part1(&data), 40);
        assert_eq!(part2(&data), 315);
    }
}
