use std::collections::HashSet;

struct Heightmap {
    heights: Vec<Vec<i32>>,
    width: i32,
    height: i32,
}

struct Point {
    x: i32,
    y: i32,
    height: i32,
}

impl Heightmap {
    pub fn get(&self, x: i32, y: i32) -> Option<i32> {
        self.heights
            .get(y as usize)
            .and_then(|l| l.get(x as usize))
            .cloned()
    }

    pub fn neighbors(&self, x: i32, y: i32) -> impl Iterator<Item = Point> + '_ {
        [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)]
            .into_iter()
            .filter_map(|(x, y)| self.get(x, y).map(|height| Point { x, y, height }))
    }
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Heightmap {
    let heights: Vec<Vec<i32>> = input
        .lines()
        .map(|l| l.chars().map(|x| x.to_digit(10).unwrap() as i32).collect())
        .collect();
    let width = heights[0].len() as i32;
    let height = heights.len() as i32;

    Heightmap {
        heights,
        width,
        height,
    }
}

#[aoc(day9, part1)]
fn part1(input: &Heightmap) -> i32 {
    (0..input.width)
        .flat_map(|x| (0..input.height).map(move |y| (x, y)))
        .filter_map(|(x, y)| {
            let height = input.get(x, y).expect("Failed to get a valid height");
            if input.neighbors(x, y).all(|p| p.height > height) {
                Some(height + 1)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day9, part2)]
fn part2(input: &Heightmap) -> usize {
    // Compute and store all basin sizes in a vec
    let mut explored: HashSet<(i32, i32)> = HashSet::default();
    let mut basin_sizes: Vec<usize> = Vec::new();
    for x in 0..input.width {
        for y in 0..input.height {
            if explored.contains(&(x, y)) {
                continue;
            }
            let basin = basin_size(input, &mut explored, (x, y));
            basin_sizes.push(basin - 1); // -1 because starting point is not considered a part of a basin
        }
    }

    // Then retrieve the top 3 sizes and multiply them
    // This is not the greatest in terms of performance, but is at least readable
    basin_sizes.sort();
    basin_sizes.iter().rev().take(3).cloned().product()
}

/// Gets the size of the basin that the position at (x,y) is a part of.
/// Conveniently adds (x,y) to the "explored" set
fn basin_size(input: &Heightmap, explored: &mut HashSet<(i32, i32)>, (x, y): (i32, i32)) -> usize {
    // doing this first avoids endless recursion of a point A adding its neighbor B that in turns
    // adds point A and so on
    explored.insert((x, y));
    input
        .neighbors(x, y)
        .filter_map(|p| {
            let pos = (p.x, p.y);
            if p.height != 9 && !explored.contains(&pos) {
                Some(basin_size(input, explored, pos))
            } else {
                None
            }
        })
        .sum::<usize>()
        + 1
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn should_work() {
        let data = parse(INPUT);
        assert_eq!(part1(&data), 15);
        assert_eq!(part2(&data), 1134);
    }
}
