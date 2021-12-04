use std::simd::u16x16;

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Result<Vec<u16>, std::num::ParseIntError> {
    input.lines().map(|x| u16::from_str_radix(x, 2)).collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[u16]) -> u32 {
    let pows = u16x16::from([
        32768, 16384, 8192, 4096, 2048, 1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1,
    ]);
    let zeros = u16x16::splat(0);
    let ones = u16x16::splat(1);
    let mut global_mask = 0; // dirty hack to cut irrelevant bits of epsilon

    let mut total = u16x16::splat(0);
    input.iter().for_each(|x| {
        global_mask |= x;
        let x = u16x16::splat(*x);
        let and = pows & x;
        let mask = and.lanes_gt(zeros);
        let add = mask.select(ones, zeros);
        total += add;
    });

    let other_count = u16x16::splat(input.len() as u16) - total;
    let gamma_mask = total.lanes_gt(other_count);
    let gamma: u16 = gamma_mask.select(pows, zeros).to_array().into_iter().sum();
    let epsilon: u16 = !gamma & global_mask;

    gamma as u32 * epsilon as u32
}

fn gamma_mask(input: &[u16]) -> [bool; 16] {
    let pows = u16x16::from([
        32768, 16384, 8192, 4096, 2048, 1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1,
    ]);
    let zeros = u16x16::splat(0);
    let ones = u16x16::splat(1);

    let mut total = u16x16::splat(0);
    input.iter().for_each(|x| {
        let x = u16x16::splat(*x);
        let and = pows & x;
        let mask = and.lanes_gt(zeros);
        let add = mask.select(ones, zeros);
        total += add;
    });
    let other_count = u16x16::splat(input.len() as u16) - total;
    total.lanes_ge(other_count).to_array()
}

enum BitCriteria {
    MostCommon,
    LeastCommon,
}

fn find_rate(mut input: Vec<u16>, criteria: BitCriteria) -> u16 {
    // dirty hack to get the bit to start with.
    // Allows this to work in normal context and tests.
    // (test = 5 bits, normal case = 12 bits)
    let mut bit = input
        .iter()
        .max()
        .expect("Failed to find max")
        .next_power_of_two()
        .log2() as usize;

    loop {
        if input.len() == 1 {
            return input[0];
        }
        bit -= 1;
        let g_mask = gamma_mask(&input);
        let target_bit = match criteria {
            BitCriteria::MostCommon => g_mask[15 - bit],
            BitCriteria::LeastCommon => !g_mask[15 - bit],
        };
        let mask = 2u16.pow(bit as u32);
        input.retain(|x| (x & mask > 0) == target_bit);
    }
}

#[aoc(day3, part2)]
fn part2(input: &Vec<u16>) -> u32 {
    let oxygen_rate = find_rate(input.clone(), BitCriteria::MostCommon) as u32;
    let co2_rate = find_rate(input.clone(), BitCriteria::LeastCommon) as u32;
    oxygen_rate * co2_rate
}

#[cfg(test)]
pub mod tests {
    use super::{parse, part1, part2};

    const INPUT: &'static str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn should_work() -> Result<(), std::num::ParseIntError> {
        let input = parse(INPUT)?;
        assert_eq!(part1(&input), 198);
        assert_eq!(part2(&input), 230);
        Ok(())
    }
}
