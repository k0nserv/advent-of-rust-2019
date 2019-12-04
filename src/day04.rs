use core::ops::RangeInclusive;
use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone)]
struct DigitIterator {
    power: f64,
    number: f64,
    done: bool,
}

impl DigitIterator {
    fn new(number: usize) -> Self {
        Self {
            done: false,
            number: number as f64,
            power: 0.0,
        }
    }
}

impl Iterator for DigitIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.number < 1.0 {
            return None;
        }
        let digit = self.number % 10_f64;
        self.number = (self.number / 10_f64).floor();

        Some(digit as usize)
    }
}

fn is_valid_password_part1(password: usize) -> bool {
    let last_idx = (password as f64).log10().floor() as usize;
    let digits: Vec<_> = DigitIterator::new(password).collect();
    let iterator = DigitIterator::new(password)
        .zip(DigitIterator::new(password).cycle().skip(1))
        .enumerate();
    let has_adjacent_numbers = digits.windows(2).any(|v| v[0] == v[1]);
    let decreases = iterator
        .clone()
        .all(|(idx, (digit, next_digit))| next_digit <= digit || idx >= last_idx);

    let result = has_adjacent_numbers && decreases;

    result
}

fn is_valid_password_part2(password: usize) -> bool {
    let last_idx = (password as f64).log10().floor() as usize;
    let iterator = DigitIterator::new(password)
        .zip(DigitIterator::new(password).cycle().skip(1))
        .enumerate();

    let sequence_lengths = DigitIterator::new(password).enumerate().fold(
        (vec![], 0, None),
        |(sequence_lengths, current_length, last_digit), (idx, d)| {
            if last_digit.map(|last| last != d).unwrap_or(false) {
                let mut new_lengths = sequence_lengths.clone();
                new_lengths.push(current_length);

                (new_lengths, 1, Some(d))
            } else if idx == last_idx {
                let mut new_lengths = sequence_lengths.clone();
                new_lengths.push(current_length + 1);

                (new_lengths, 0, None)
            } else {
                (sequence_lengths, current_length + 1, Some(d))
            }
        },
    );

    let decreases = iterator
        .clone()
        .all(|(idx, (digit, next_digit))| next_digit <= digit || idx >= last_idx);

    sequence_lengths.0.iter().any(|&length| length == 2) && decreases
}

pub fn star_one(input: RangeInclusive<usize>) -> usize {
    input
        .into_iter()
        .filter(|&password| is_valid_password_part1(password))
        .count()
}

pub fn star_two(input: RangeInclusive<usize>) -> usize {
    input
        .into_iter()
        .filter(|&password| is_valid_password_part2(password))
        .count()
}

#[cfg(test)]
mod tests {
    use super::{is_valid_password_part2, DigitIterator};

    fn collect_digits(number: usize) -> Vec<usize> {
        DigitIterator::new(number).collect()
    }

    #[test]
    fn test_digit_iterator() {
        assert_eq!(collect_digits(1000), vec![0, 0, 0, 1]);
        assert_eq!(collect_digits(1), vec![1]);
        assert_eq!(collect_digits(10), vec![0, 1]);
        assert_eq!(collect_digits(12345), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_is_valid_password_part2() {
        assert_eq!(is_valid_password_part2(558999), true);
        assert_eq!(is_valid_password_part2(577899), true);
    }
}
