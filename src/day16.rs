use core::mem;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::hash::{BuildHasher, Hash, Hasher};

use crate::parse_custom_separated;

const PATTERN: &[isize] = &[0, 1, 0, -1];

// Repeated 2x: 0, 0, 1, 1, 0, 0, -1, -1
// Actual 2x: 0, 1, 1, 0, 0, -1, -1, 0, 0
//
// Repeated 3x:  0, 0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1
// Actual 3x:    0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1, 0, 0, 0
// Pattern idx:  0, 0, 1, 1, 1, 2, 2, 2,  3,  3,  3, 0, 0, 0,
//

#[inline(always)]
fn calculate_pattern_digit(repeat_count: usize, idx: usize) -> isize {
    let cycle_size = repeat_count * PATTERN.len();
    let is_first_cycle = idx < cycle_size - 1;

    if is_first_cycle {
        if idx < (repeat_count - 1) {
            PATTERN[0]
        } else {
            let adjusted_idx = idx - (repeat_count - 1);
            // dbg!(
            //     repeat_count,
            //     idx,
            //     adjusted_idx,
            //     index_in_cycle,
            //     cycle_size,
            //     is_first_cycle
            // );

            PATTERN[adjusted_idx / repeat_count + 1]
        }

    // PATTERN
    //     .iter()
    //     .flat_map(|&pattern_digit| (0..repeat_count).map(move |_| pattern_digit))
    //     .cycle()
    //     .skip(1)
    //     .nth(index_in_cycle)
    //     .unwrap()
    } else {
        let index_in_cycle = (idx - (cycle_size - 1)) % cycle_size;
        let pattern_index = index_in_cycle / repeat_count;
        // dbg!(
        //     pattern_index,
        //     repeat_count,
        //     idx,
        //     index_in_cycle,
        //     cycle_size,
        //     is_first_cycle
        // );

        PATTERN[pattern_index]
    }
}

fn phase_with_offset(digits: Vec<isize>, offset: usize) -> Vec<isize> {
    (0..digits.len())
        .map(|idx| {
            // let pattern = PATTERN
            //     .iter()
            //     .flat_map(|&pattern_digit| (0..idx + 1).map(move |_| pattern_digit))
            //     .cycle()
            //     .skip(1);

            // (pattern
            //     .zip(digits.iter())
            //     .map(|(p, &d)| p * d)
            //     .sum::<isize>()
            //     % 10)
            //     .abs()
            (digits
                .iter()
                .enumerate()
                .map(|(didx, d)| d * calculate_pattern_digit(offset + idx + 1, offset + didx))
                .sum::<isize>()
                % 10)
                .abs()
        })
        .collect()
}

fn calculate_hash(random_state: &RandomState, digits: &[isize]) -> u64 {
    let mut hasher = random_state.build_hasher();
    digits.hash(&mut hasher);

    hasher.finish()
}

pub fn star_one(input: &str) -> Vec<isize> {
    let mut digits: Vec<_> = parse_custom_separated::<isize>(input, "").collect();

    for _ in 0..100 {
        digits = phase_with_offset(digits, 0);
    }

    digits.into_iter().take(8).collect()
}

pub fn star_two(input: &str) -> Vec<isize> {
    let original_digits: Vec<_> = parse_custom_separated::<isize>(input, "").collect();
    let offset_str: String = input.chars().take(7).collect();
    let offset = offset_str.parse::<usize>().unwrap();
    let total_length = original_digits.len() * 10_000;
    let length = total_length - offset + 1;
    let mut digits: Vec<_> = (0..length)
        .map(|idx| original_digits[(offset + idx - 1) % original_digits.len()])
        .collect();

    for _ in 0..100 {
        let mut sums = Vec::with_capacity(digits.len());
        sums.push(digits[length - 1]);
        for i in (0..length - 1).rev() {
            sums.push(digits[i] + sums.last().unwrap());
        }
        digits = sums.into_iter().rev().map(|s| (s % 10).abs()).collect();
    }
    digits.into_iter().skip(1).take(8).collect()
}

#[cfg(test)]
mod tests {
    use super::{calculate_pattern_digit, star_one, star_two};

    #[test]
    fn test_star_one() {
        // assert_eq!(star_one("12345678"), 24176176);
        assert_eq!(
            star_one("80871224585914546619083218645595"),
            [2, 4, 1, 7, 6, 1, 7, 6]
        );
        assert_eq!(
            star_one("19617804207202209144916044189917"),
            [7, 3, 7, 4, 5, 4, 1, 8]
        );
        assert_eq!(
            star_one("69317163492948606335995924319873"),
            [5, 2, 4, 3, 2, 1, 3, 3]
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two("03036732577212944063491565474664"),
            [8, 4, 4, 6, 2, 0, 2, 6]
        );
    }

    #[test]
    fn test_calculate_pattern_digit() {
        assert_eq!(calculate_pattern_digit(1, 0), 1);
        assert_eq!(calculate_pattern_digit(8, 0), 0);
        assert_eq!(calculate_pattern_digit(3, 2), 1);
        assert_eq!(calculate_pattern_digit(3, 11), 0);
        assert_eq!(calculate_pattern_digit(4, 7), 0);
        assert_eq!(calculate_pattern_digit(7, 5), 0);
        assert_eq!(calculate_pattern_digit(7, 6), 1);
        assert_eq!(calculate_pattern_digit(15, 39), 0);
        assert_eq!(calculate_pattern_digit(15, 39), 0);
        assert_eq!(calculate_pattern_digit(5973181, 5973181), 1);
        assert_eq!(calculate_pattern_digit(5973181, 5973180), 1);
        assert_eq!(calculate_pattern_digit(5973181, 5973179), 0);
    }
}
