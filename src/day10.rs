use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::f64::consts::PI;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Degree {
    radian: isize,
}

impl Degree {
    fn new(radians: f64) -> Self {
        Self {
            radian: (radians * 1000.0).floor() as isize,
        }
    }
}

impl PartialOrd for Degree {
    fn partial_cmp(&self, other: &Degree) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Degree {
    fn cmp(&self, other: &Degree) -> Ordering {
        self.radian.cmp(&other.radian)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn distances_to(&self, other: &Point) -> f64 {
        let x1 = self.x as f64;
        let x2 = other.x as f64;
        let y1 = self.y as f64;
        let y2 = other.y as f64;

        ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt()
    }
}

fn fuzzy_cmp(a: f64, b: f64, tolerance: f64) -> bool {
    a >= b - tolerance && a <= b + tolerance
}

pub fn star_one(input: &str) -> ((isize, isize), usize) {
    let asteroids: Vec<_> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some(Point::new(x as isize, y as isize)),
                _ => None,
            })
        })
        .collect();

    asteroids
        .iter()
        .map(|asteroid| {
            let angles_to_others: Vec<_> = {
                let mut angles_to_others: Vec<_> = asteroids
                    .iter()
                    .filter(|a| a != &asteroid)
                    .map(|a| {
                        let x = a.x - asteroid.x;
                        let y = a.y - asteroid.y;
                        let angle = (y as f64).atan2(x as f64);

                        (a, Degree::new(angle))
                    })
                    .collect();
                angles_to_others.sort_by(|a, b| a.1.cmp(&b.1));

                angles_to_others
            };

            let visible_count = angles_to_others
                .into_iter()
                .group_by(|v| v.1)
                .into_iter()
                .map(|(key, group)| (key, group.count()))
                .count();

            ((asteroid.x, asteroid.y), visible_count)
        })
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
}

pub fn star_two(input: &str, laser_location: (isize, isize)) -> isize {
    let mut asteroids: Vec<_> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some((Point::new(x as isize, y as isize), true)),
                _ => None,
            })
        })
        .collect();
    let laser_location = Point::new(laser_location.0, laser_location.1);

    let angles_to_others: Vec<_> = {
        let up = Point::new(-1, 0);
        let angles_to_others_unique: HashSet<_> = std::iter::once(up)
            .chain(
                asteroids
                    .iter()
                    .filter(|&(a, _)| a != &laser_location)
                    .map(|&(a, _)| Point::new(a.x - laser_location.x, a.y - laser_location.y)),
            )
            .map(|direction| {
                let angle = (direction.y as f64).atan2(direction.x as f64);

                Degree::new(angle + PI / 2.0)
            })
            .collect();
        let mut angles_to_others: Vec<_> = angles_to_others_unique.into_iter().collect();

        angles_to_others.sort_by(|a, b| a.cmp(&b));

        angles_to_others
    };

    angles_to_others
        .into_iter()
        .cycle()
        .filter_map(|laser_angle| {
            dbg!(laser_angle);
            let hit: Option<Point> = {
                let mut in_laser_path: Vec<_> = asteroids
                    .iter()
                    .filter(|&(_, exists)| *exists)
                    .filter(|&(a, _)| {
                        let x = a.x - laser_location.x;
                        let y = a.y - laser_location.y;
                        let angle = (y as f64).atan2(x as f64);

                        laser_angle == Degree::new(angle)
                    })
                    .map(|&(a, _)| a)
                    .collect();

                in_laser_path.sort_by(|a, b| {
                    a.distances_to(&laser_location)
                        .partial_cmp(&b.distances_to(&laser_location))
                        .unwrap()
                });

                in_laser_path.into_iter().nth(0)
            };

            match hit {
                Some(location) => {
                    let (idx, (a, _)): (usize, &(Point, bool)) = {
                        asteroids
                            .iter()
                            .enumerate()
                            .find(|&(_, (a, _))| a == &location)
                            .unwrap()
                    };
                    let location = *a;
                    asteroids[idx] = (location.clone(), false);

                    Some(location)
                }
                None => None,
            }
        })
        .inspect(|location| {
            dbg!(location);
        })
        .nth(0)
        .map(|location| location.x * 100 + location.y)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const SMALLEST_TEST_CASE: &'static str = "
.#..#
.....
#####
....#
...##
";

    const SMALL_TEST_CASE: &'static str = "
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
";
    const LARGE_TEST_CASE: &'static str = "
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
";

    #[test]
    fn test_star_one_smallest() {
        assert_eq!(star_one(SMALLEST_TEST_CASE), ((3, 4), 8));
    }

    #[test]
    fn test_star_one_small() {
        assert_eq!(star_one(SMALL_TEST_CASE), ((5, 8), 33));
    }

    #[test]
    fn test_star_one_large() {
        assert_eq!(star_one(LARGE_TEST_CASE), ((11, 13), 210));
    }

    #[test]
    fn test_star_two_large() {
        assert_eq!(star_two(LARGE_TEST_CASE, (11, 13)), 820);
    }
}
