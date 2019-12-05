use crate::DigitIterator;
use core::ops::RangeInclusive;

fn is_valid_password_part1(password: usize) -> bool {
    let digits: Vec<_> = DigitIterator::new(password).collect();
    let has_adjacent_numbers = digits.windows(2).any(|v| v[0] == v[1]);
    let decreases = digits.windows(2).all(|v| v[1] <= v[0]);

    has_adjacent_numbers && decreases
}

fn is_valid_password_part2(password: usize) -> bool {
    let last_idx = (password as f64).log10().floor() as usize;

    let digits: Vec<_> = DigitIterator::new(password).collect();
    let decreases = digits.windows(2).all(|v| v[1] <= v[0]);
    let sequence_lengths = digits
        .into_iter()
        .enumerate()
        .fold(
            (vec![], 0, None),
            |(sequence_lengths, current_length, last_digit), (idx, d)| {
                if last_digit.map(|last| last != d).unwrap_or(false) {
                    // Digit differs from last, start of new sequence
                    let mut new_lengths = sequence_lengths.clone();
                    new_lengths.push(current_length);

                    (new_lengths, 1, Some(d))
                } else if idx == last_idx {
                    // At the end of digits, special case to record current sequence
                    let mut new_lengths = sequence_lengths.clone();
                    new_lengths.push(current_length + 1);

                    (new_lengths, 0, None)
                } else {
                    // In the middle of sequence
                    (sequence_lengths, current_length + 1, Some(d))
                }
            },
        )
        .0;

    sequence_lengths.into_iter().any(|length| length == 2) && decreases
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
