use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasher, Hash, Hasher};
use std::ops::Add;
use std::str::FromStr;

use crate::parse_lines;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Vector {
    x: isize,
    y: isize,
    z: isize,
}

impl Vector {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Vector { x, y, z }
    }

    fn axis(&self, axis: usize) -> isize {
        match axis {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!(format!("Invalid axis {}", axis)),
        }
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Self {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<&Vector> for Vector {
    type Output = Vector;

    fn add(self, other: &Vector) -> Self {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl FromStr for Vector {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let s = input.trim().trim_start_matches("<").trim_end_matches(">");
        let values: HashMap<&str, isize> = s
            .split(",")
            .map(|part| part.trim())
            .filter(|part| part.len() > 1)
            .map(|part| {
                let mut values = part.split("=");

                (
                    values.next().unwrap(),
                    values.next().unwrap().parse::<isize>().unwrap(),
                )
            })
            .collect();

        Ok(Self {
            x: *values.get("x").unwrap(),
            y: *values.get("y").unwrap(),
            z: *values.get("z").unwrap(),
        })
    }
}

impl Default for Vector {
    fn default() -> Self {
        Vector { x: 0, y: 0, z: 0 }
    }
}

fn timestemp(positions: &[Vector], velocities: &[Vector]) -> (Vec<Vector>, Vec<Vector>) {
    let new_velocities: Vec<_> = positions
        .iter()
        .zip(velocities.iter())
        .enumerate()
        .map(|(idx, (position, velocity))| {
            *velocity
                + positions.iter().enumerate().fold(
                    Vector::default(),
                    |mut acc, (other_idx, other_position)| {
                        if idx == other_idx {
                            acc
                        } else {
                            acc.x += (other_position.x - position.x).min(1).max(-1);
                            acc.y += (other_position.y - position.y).min(1).max(-1);
                            acc.z += (other_position.z - position.z).min(1).max(-1);

                            acc
                        }
                    },
                )
        })
        .collect();

    let new_positions = positions
        .iter()
        .zip(new_velocities.iter())
        .map(|(p, v)| Vector::new(p.x + v.x, p.y + v.y, p.z + v.z))
        .collect();

    (new_positions, new_velocities)
}

fn energy(positions: &[Vector], velocities: &[Vector]) -> isize {
    positions
        .into_iter()
        .zip(velocities.into_iter())
        .map(|(p, v)| {
            let potential = p.x.abs() + p.y.abs() + p.z.abs();
            let kinetic = v.x.abs() + v.y.abs() + v.z.abs();

            kinetic * potential
        })
        .sum()
}

pub fn star_one(input: &str, num_steps: usize) -> isize {
    let mut positions: Vec<_> = parse_lines::<Vector>(input).collect();
    let mut velocities: Vec<_> = positions.iter().map(|_| Vector::default()).collect();

    for _ in 0..num_steps {
        let update = timestemp(&positions, &velocities);
        positions = update.0;
        velocities = update.1;
    }

    energy(&positions, &velocities)
}

pub fn star_two(input: &str) -> Vec<usize> {
    let original_positions: Vec<_> = parse_lines::<Vector>(input).collect();
    let original_velocities: Vec<_> = original_positions
        .iter()
        .map(|_| Vector::default())
        .collect();

    let mut positions: Vec<_> = original_positions.clone();
    let mut velocities: Vec<_> = original_velocities.clone();
    let mut periods: Vec<Option<usize>> = (0..3).map(|_| None).collect();

    let mut steps = 0;
    while periods.iter().any(Option::is_none) {
        let update = timestemp(&positions, &velocities);
        positions = update.0;
        velocities = update.1;
        steps += 1;

        for a in 0..3 {
            if periods[a].is_none()
                && velocities.iter().all(|v| v.axis(a) == 0)
                && positions
                    .iter()
                    .zip(original_positions.iter())
                    .all(|(p, op)| p.axis(a) == op.axis(a))
            {
                periods[a] = Some(steps);
            }
        }
    }

    periods.into_iter().map(|period| period.unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    const TEST_INPUT_SMALL: &'static str = "
<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
    const TEST_INPUT_LARGE: &'static str = "
<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";

    #[test]
    fn test_star_one_small() {
        assert_eq!(star_one(TEST_INPUT_SMALL, 10), 179);
    }

    #[test]
    fn test_star_two_small() {
        assert_eq!(star_two(TEST_INPUT_SMALL), vec![18, 28, 44]);
    }

    #[test]
    fn test_star_two_large() {
        assert_eq!(star_two(TEST_INPUT_LARGE), vec![2028, 5898, 4702]);
    }
}
