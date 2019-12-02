use crate::parse_lines;

fn fuel(mass: i64) -> i64 {
    mass / 3 - 2
}

fn recursive_fuel(mass: i64, acc: i64) -> i64 {
    let new_mass = fuel(mass);

    if new_mass > 0 {
        recursive_fuel(new_mass, acc + new_mass)
    } else {
        acc
    }
}

pub fn star_one(input: &str) -> i64 {
    parse_lines(input).map(fuel).sum()
}

pub fn star_two(input: &str) -> i64 {
    parse_lines(input).map(|m| recursive_fuel(m, 0)).sum()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one("12\n12"), 4);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two("14\n1969\n100756"), 2 + 966 + 50346);
    }
}
