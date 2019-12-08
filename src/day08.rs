use crate::{parse_custom_separated, DigitIterator};

fn digit_count(value: usize, counted_digit: usize) -> usize {
    let iterator = DigitIterator::new(value);
    iterator.fold(
        0,
        |acc, digit| if digit == counted_digit { acc + 1 } else { acc },
    )
}

pub fn star_one(input: &str, layer_width: usize, layer_height: usize) -> usize {
    let numbers: Vec<_> = parse_custom_separated::<usize>(input, "").collect();
    let layer_pixel_width = layer_width * layer_height;

    let least_zeros = numbers
        .chunks(layer_pixel_width)
        .map(|layer| {
            (
                layer,
                layer
                    .iter()
                    .fold(0, |acc, &value| acc + digit_count(value, 0)),
            )
        })
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap();

    least_zeros
        .0
        .iter()
        .fold(0, |acc, &value| acc + digit_count(value, 1))
        * least_zeros
            .0
            .iter()
            .fold(0, |acc, &value| acc + digit_count(value, 2))
}

pub fn star_two(input: &str, layer_width: usize, layer_height: usize) -> String {
    let numbers: Vec<_> = parse_custom_separated::<usize>(input, "").collect();
    let layer_pixel_width = layer_width * layer_height;
    let layers: Vec<_> = numbers.chunks(layer_pixel_width).collect();
    assert!(layers.windows(2).all(|ls| ls[0].len() == ls[1].len()));

    let layers = &layers;
    let result: Vec<String> = (0..layer_height)
        .map(|y| {
            (0..layer_width)
                .map(move |x| {
                    let normalized_index = (x % layer_width) + (y * layer_width);

                    layers
                        .iter()
                        .find_map(|layer| {
                            if layer[normalized_index] == 2 {
                                None
                            } else {
                                match layer[normalized_index] {
                                    0 => Some('0'),
                                    1 => Some('1'),
                                    _ => panic!("Invalid pixel value {}", layer[normalized_index]),
                                }
                            }
                        })
                        .unwrap()
                })
                .collect()
        })
        .collect();

    result.join("\n")
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one("123456789012", 3, 2), 1);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two("0222112222120000", 2, 2), "01\n10");

        // 020
        // 102

        // 120
        // 002

        // 010
        // 002

        // 000
        // 001
        assert_eq!(star_two("020102120002010002000001", 3, 2), "010\n101")
    }
}
