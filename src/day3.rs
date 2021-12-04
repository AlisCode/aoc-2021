use std::simd::{u16x16, Mask, SimdElement};

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

    //// dirty hack to cut irrelevant bits
    let global_mask: u16 = input
        .iter()
        .max()
        .expect("Failed to find max")
        .next_power_of_two()
        - 1;

    let g_mask = gamma_mask(&input);
    let epsilon_mask = !g_mask;

    let gamma: u16 = g_mask
        .select(pows, zeros)
        .to_array()
        .into_iter()
        .sum::<u16>()
        & global_mask;
    let epsilon: u16 = epsilon_mask
        .select(pows, zeros)
        .to_array()
        .into_iter()
        .sum::<u16>()
        & global_mask;

    gamma as u32 * epsilon as u32
}

#[aoc(day3, part2)]
fn part2(input: &Vec<u16>) -> u32 {
    let oxygen_rate = find_rate(input.clone(), BitCriteria::MostCommon) as u32;
    let co2_rate = find_rate(input.clone(), BitCriteria::LeastCommon) as u32;
    oxygen_rate * co2_rate
}

enum BitCriteria {
    MostCommon,
    LeastCommon,
}

fn gamma_mask(input: &[u16]) -> Mask<<i16 as SimdElement>::Mask, 16> {
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
    total.lanes_ge(other_count)
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
        let g_mask = gamma_mask(&input).to_array();
        let target_bit = match criteria {
            BitCriteria::MostCommon => g_mask[15 - bit],
            BitCriteria::LeastCommon => !g_mask[15 - bit],
        };
        let mask = 2u16.pow(bit as u32);
        input.retain(|x| (x & mask > 0) == target_bit);
    }
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
