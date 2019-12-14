use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code, unused_imports)]
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod intcode_computer;

#[derive(Debug, Copy, Clone)]
pub struct DigitIterator {
    initial_value_is_zero: bool,
    number: f64,
}

impl DigitIterator {
    fn new(number: usize) -> Self {
        Self {
            initial_value_is_zero: number == 0,
            number: number as f64,
        }
    }
}

impl Iterator for DigitIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.number < 1.0 && !self.initial_value_is_zero {
            return None;
        }

        if self.initial_value_is_zero {
            self.initial_value_is_zero = false;

            Some(0)
        } else {
            let digit = self.number % 10_f64;
            self.number = (self.number / 10_f64).floor();

            Some(digit as usize)
        }
    }
}

fn time<F>(label: &str, closure: F)
where
    F: Fn(),
{
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    closure();
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let time = end - start;
    println!(
        "Time taken for {}: {}s and {}ns",
        label,
        time.as_secs(),
        time.subsec_nanos()
    );
}

/// Parse lines of text into custom types.
///
/// Each line is treated as parsable after trimming.
///
/// **Note:** Panics if any parsing fails
fn parse_lines<T>(input: &str) -> impl Iterator<Item = T> + '_
where
    T: FromStr + std::fmt::Debug,
    <T as FromStr>::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(str::trim)
        .filter(|l| l.len() > 0)
        .map(|l| {
            l.parse().expect(&format!(
                "Expected to be able to parse `{:?}` as `{:?}`",
                l,
                std::any::type_name::<T>()
            ))
        })
}

/// Parse whitespace separated custom types.
///
/// Each unit separated by whitespace is treated as parsable after trimming.
///
/// **Note:** Panics if any parsing fails
fn parse_whitespace_separated<T>(input: &str) -> impl Iterator<Item = T> + '_
where
    T: FromStr + std::fmt::Debug,
    <T as FromStr>::Err: std::fmt::Debug,
{
    input
        .split_whitespace()
        .map(str::trim)
        .filter(|l| l.len() > 0)
        .map(|l| {
            l.parse().expect(&format!(
                "Expected to be able to parse `{:?}` as `{:?}`",
                l,
                std::any::type_name::<T>()
            ))
        })
}

/// Parse custom separator separated custom types.
///
/// Each unit separated by a specific separator is treated as parsable after trimming.
///
/// **Note:** Panics if any parsing fails
fn parse_custom_separated<'a, T>(input: &'a str, separator: &'a str) -> impl Iterator<Item = T> + 'a
where
    T: FromStr + std::fmt::Debug,
    <T as FromStr>::Err: std::fmt::Debug,
{
    input
        .split(separator)
        .map(str::trim)
        .filter(|l| l.len() > 0)
        .map(|l| {
            l.parse().expect(&format!(
                "Expected to be able to parse `{:?}` as `{:?}`",
                l,
                std::any::type_name::<T>()
            ))
        })
}

#[cfg(test)]
mod tests {
    use super::time;
    use std::fs::File;
    use std::io::Read;

    fn load_file(path: &str) -> String {
        let mut input = String::new();
        let mut f = File::open(path).expect("Unable to open file");
        f.read_to_string(&mut input).expect("Unable to read string");

        input
    }

    #[test]
    fn test_parse_lines() {
        use crate::parse_lines;

        let expected: Vec<usize> = vec![12_usize, 24, 301, 123123];

        assert_eq!(
            parse_lines::<usize>("12\n 24   \n 301 \n 123123 \n").collect::<Vec<_>>(),
            expected
        );
    }

    #[test]
    fn test_parse_whitespace_separated() {
        use crate::parse_whitespace_separated;

        let expected: Vec<usize> = vec![12_usize, 24, 301, 123123];

        assert_eq!(
            parse_whitespace_separated::<usize>("12   24   \n301 \t 123123 \n").collect::<Vec<_>>(),
            expected
        );
    }

    #[test]
    fn test_parse_custom_separated() {
        use crate::parse_custom_separated;

        let expected: Vec<usize> = vec![12_usize, 24, 301, 123123];

        assert_eq!(
            parse_custom_separated::<usize>("12, 24,    301, \t 123123, ", ",").collect::<Vec<_>>(),
            expected
        );
    }

    #[test]
    fn solve_day01() {
        use crate::day01::{star_one, star_two};

        let input = load_file("day01.txt");

        time("Day 01, Part 1", || assert_eq!(star_one(&input), 3506577));
        time("Day 01, Part 2", || assert_eq!(star_two(&input), 5256960));
    }

    #[test]
    fn solve_day02() {
        use crate::day02::{star_one, star_two};

        let input = load_file("day02.txt");

        time("Day 02, Part 1", || assert_eq!(star_one(&input), 3790689));
        time("Day 02, Part 2", || assert_eq!(star_two(&input), 6533));
    }

    #[test]
    fn solve_day03() {
        use crate::day03::{star_one, star_two};

        let input = load_file("day03.txt");

        time("Day 03, Part 1", || assert_eq!(star_one(&input), 865));
        time("Day 03, Part 2", || assert_eq!(star_two(&input), 35038));
    }

    #[test]
    fn solve_day04() {
        use crate::day04::{star_one, star_two};
        use core::ops::RangeInclusive;

        time("Day 04, Part 1", || {
            assert_eq!(star_one(RangeInclusive::new(136760, 595730)), 1873)
        });
        time("Day 04, Part 2", || {
            assert_eq!(star_two(RangeInclusive::new(136760, 595730)), 1264)
        });
    }

    #[test]
    fn solve_day05() {
        use crate::day05::{star_one, star_two};

        let input = load_file("day05.txt");

        time("Day 05, Part 1", || assert_eq!(star_one(&input), 8332629));
        time("Day 05, Part 2", || assert_eq!(star_two(&input), 8805067));
    }

    #[test]
    fn solve_day06() {
        use crate::day06::{star_one, star_two};

        let input = load_file("day06.txt");

        time("Day 06, Part 1", || assert_eq!(star_one(&input), 300598));
        time("Day 06, Part 2", || assert_eq!(star_two(&input), 520));
    }

    #[test]
    fn solve_day07() {
        use crate::day07::{star_one, star_two};

        let input = load_file("day07.txt");

        time("Day 07, Part 1", || assert_eq!(star_one(&input), 46014));
        time("Day 07, Part 2", || assert_eq!(star_two(&input), 19581200));
    }

    #[test]
    fn solve_day08() {
        use crate::day08::{star_one, star_two};

        let input = load_file("day08.txt");

        time("Day 08, Part 1", || {
            assert_eq!(star_one(&input, 25, 6), 2975)
        });
        time("Day 08, Part 2", || {
            assert_eq!(star_two(&input, 25, 6), "1111010010111001001011110\n1000010010100101001010000\n1110011110100101001011100\n1000010010111001001010000\n1000010010101001001010000\n1111010010100100110011110")
        });
    }

    #[test]
    fn solve_day09() {
        use crate::day09::{star_one, star_two};

        let input = load_file("day09.txt");

        time("Day 09, Part 1", || {
            assert_eq!(star_one(&input), 2453265701)
        });
        time("Day 09, Part 2", || assert_eq!(star_two(&input), 80805));
    }

    #[test]
    fn solve_day10() {
        use crate::day10::{star_one, star_two};

        let input = load_file("day10.txt");

        assert_eq!(star_one(&input), ((23, 19), 278));
        assert_eq!(star_two(&input, (23, 19)), 1);
    }

    #[test]
    fn solve_day11() {
        use crate::day11::{star_one, star_two};

        let input = load_file("day11.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day12() {
        use crate::day12::{star_one, star_two};

        let input = load_file("day12.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day13() {
        use crate::day13::{star_one, star_two};

        let input = load_file("day13.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day14() {
        use crate::day14::{star_one, star_two};

        let input = load_file("day14.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day15() {
        use crate::day15::{star_one, star_two};

        let input = load_file("day15.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day16() {
        use crate::day16::{star_one, star_two};

        let input = load_file("day16.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day17() {
        use crate::day17::{star_one, star_two};

        let input = load_file("day17.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day18() {
        use crate::day18::{star_one, star_two};

        let input = load_file("day18.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day19() {
        use crate::day19::{star_one, star_two};

        let input = load_file("day19.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day20() {
        use crate::day20::{star_one, star_two};

        let input = load_file("day20.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day21() {
        use crate::day21::{star_one, star_two};

        let input = load_file("day21.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day22() {
        use crate::day22::{star_one, star_two};

        let input = load_file("day22.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day23() {
        use crate::day23::{star_one, star_two};

        let input = load_file("day23.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day24() {
        use crate::day24::{star_one, star_two};

        let input = load_file("day24.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
}
