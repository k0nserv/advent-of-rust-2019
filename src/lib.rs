use std::str::FromStr;

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

        assert_eq!(star_one(&input), 3506577);
        assert_eq!(star_two(&input), 5256960);
    }

    #[test]
    fn solve_day02() {
        use crate::day02::{star_one, star_two};

        let input = load_file("day02.txt");

        assert_eq!(star_one(&input), 3790689);
        assert_eq!(star_two(&input), 6533);
    }

    #[test]
    fn solve_day05() {
        use crate::day05::{star_one, star_two};

        let input = load_file("day05.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day06() {
        use crate::day06::{star_one, star_two};

        let input = load_file("day06.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day07() {
        use crate::day07::{star_one, star_two};

        let input = load_file("day07.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day08() {
        use crate::day08::{star_one, star_two};

        let input = load_file("day08.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day09() {
        use crate::day09::{star_one, star_two};

        let input = load_file("day09.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day10() {
        use crate::day10::{star_one, star_two};

        let input = load_file("day10.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
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
