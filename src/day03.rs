use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use crate::parse_custom_separated;

type Point = (i64, i64);
const CENTER: Point = (0, 0);

#[derive(Debug)]
enum Direction {
    R,
    U,
    L,
    D,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        match lower.as_ref() {
            "r" => Ok(Direction::R),
            "u" => Ok(Direction::U),
            "l" => Ok(Direction::L),
            "d" => Ok(Direction::D),
            _ => Err(format!("Invalid direction {}", s)),
        }
    }
}

#[derive(Debug)]
struct Step {
    direction: Direction,
    steps: usize,
}

impl Step {
    fn dir(&self) -> Point {
        match self.direction {
            Direction::R => (1, 0),
            Direction::U => (0, -1),
            Direction::L => (-1, 0),
            Direction::D => (0, 1),
        }
    }
}

impl FromStr for Step {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = s[0..1].parse()?;
        match s[1..].parse() {
            Ok(steps) => Ok(Self { direction, steps }),
            Err(_) => Err(format!("Unable to parse step {}", s)),
        }
    }
}

type Path = Vec<Step>;

fn manhattan_distance(origin: &Point, point: &Point) -> usize {
    ((origin.0 as i64 - point.0 as i64).abs() + (origin.1 as i64 - point.1 as i64).abs()) as usize
}

#[derive(Debug)]
struct OccupiedPoint {
    location: Point,
    distance: usize,
}

impl OccupiedPoint {
    fn new(location: Point, distance: usize) -> Self {
        Self { location, distance }
    }
}

impl Hash for OccupiedPoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.location.hash(state);
    }
}

impl PartialEq for OccupiedPoint {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location
    }
}
impl Eq for OccupiedPoint {}

fn wire(paths: &[Path]) -> Vec<HashSet<OccupiedPoint>> {
    let wires: Vec<_> = paths
        .iter()
        .map(|path| {
            let mut locations_occupied = HashSet::<OccupiedPoint>::new();
            let mut location = CENTER;
            let mut distance: usize = 0;

            for step in path {
                let steps = step.steps as i64;
                let dir = step.dir();
                for n in 0..steps {
                    locations_occupied.insert(OccupiedPoint::new(
                        (location.0 + dir.0 * n, location.1 + dir.1 * n),
                        distance + n as usize,
                    ));
                }

                location = (location.0 + steps * dir.0, location.1 + steps * dir.1);

                distance += steps as usize;
            }

            locations_occupied
        })
        .collect();

    wires
}

pub fn star_one(input: &str) -> usize {
    let paths: Vec<Path> = input
        .lines()
        .map(|line| parse_custom_separated(line, ",").collect())
        .collect();

    let wires: Vec<_> = wire(&paths);
    assert!(wires.len() == 2, "This only works with two wires");
    let wires = &wires;
    let intersections = wires[0].intersection(&wires[1]);

    intersections
        .filter_map(|point| {
            if &point.location == &CENTER {
                None
            } else {
                Some(manhattan_distance(&CENTER, &point.location))
            }
        })
        .min()
        .expect("There should be at least one intersection")
}

pub fn star_two(input: &str) -> usize {
    let paths: Vec<Path> = input
        .lines()
        .map(|line| parse_custom_separated(line, ",").collect())
        .collect();

    let wires: Vec<_> = wire(&paths);
    assert!(wires.len() == 2, "This only works with two wires");
    let wires = &wires;

    wires[0]
        .iter()
        .filter(|&point| point.location != CENTER)
        .filter_map(|point| {
            wires[1]
                .get(point)
                .map(|other| OccupiedPoint::new(point.location, point.distance + other.distance))
        })
        .min_by(|point, other| point.distance.cmp(&other.distance))
        .map(|p| p.distance)
        .expect("There should be at least one intersection")
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
            159
        );
        assert_eq!(
            star_one(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
            610
        );
        assert_eq!(
            star_two(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            410
        );
    }
}
