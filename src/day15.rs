use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryFrom;
use std::fmt;

use crate::intcode_computer::Computer;
use crate::math::Vector2;
use crate::parse_custom_separated;

pub type Location = Vector2<isize>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    North = 1,
    South,
    West,
    East,
}
const ALL_DIRECTIONS: &'static [Direction] = &[
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
];

impl Direction {
    fn dir(&self) -> Location {
        match self {
            Direction::North => Location::new(0, 1),
            Direction::South => Location::new(0, -1),
            Direction::West => Location::new(-1, 0),
            Direction::East => Location::new(1, 0),
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }

    fn all() -> &'static [Direction] {
        ALL_DIRECTIONS
    }
}

#[derive(Eq, PartialEq)]
pub enum Status {
    BlockedByWall = 0,
    Moved,
    FoundOxygenSystem,
}

impl TryFrom<isize> for Status {
    type Error = String;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Status::BlockedByWall),
            1 => Ok(Status::Moved),
            2 => Ok(Status::FoundOxygenSystem),
            _ => Err(format!("Invalid Status {}", value)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Wall,
    OxygenSystem,
}

#[derive(Debug, PartialEq, Eq)]
enum Behavior {
    Exploring,
    Backtracking,
    OxygenSystemFound,
}

pub struct World {
    pub visited_locations: HashMap<Location, Tile>,
    pub oxygen_location: Option<Location>,
    pub droid_location: Location,
    path: VecDeque<Direction>,
    behavior: Behavior,
    explored_everything: bool,
}

impl World {
    pub fn new() -> Self {
        Self {
            visited_locations: std::iter::once((Vector2::default(), Tile::Empty)).collect(),
            oxygen_location: None,
            droid_location: Location::default(),
            path: VecDeque::new(),
            behavior: Behavior::Exploring,
            explored_everything: false,
        }
    }

    pub fn done_exploring(&self) -> bool {
        self.explored_everything
    }

    pub fn adjacent_open_locations(&self, to: Location) -> HashSet<Location> {
        Direction::all()
            .iter()
            .filter_map(|dir| {
                let new_location = to + dir.dir();

                match self.visited_locations.get(&new_location) {
                    None => None,
                    Some(tile) => match tile {
                        Tile::Wall => None,
                        _ => Some(new_location),
                    },
                }
            })
            .collect()
    }

    pub fn next_direction(&mut self) -> Direction {
        let explore_direction = Direction::all()
            .iter()
            .find(|&dir| {
                let location = self.droid_location + dir.dir();

                match self.visited_locations.get(&location) {
                    None => true,
                    Some(_) => false,
                }
            })
            .cloned();

        match explore_direction {
            // Let's explore
            Some(dir) => {
                self.behavior = Behavior::Exploring;
                dir
            }

            // Let's backtrack by walking the path that got us here backwards
            None => {
                self.behavior = Behavior::Backtracking;

                self.path.pop_back().unwrap().opposite()
            }
        }
    }

    pub fn update(&mut self, direction: Direction, status: Status) {
        let new_tile = match status {
            Status::BlockedByWall => Tile::Wall,
            Status::Moved => Tile::Empty,
            Status::FoundOxygenSystem => Tile::OxygenSystem,
        };

        let location = self.droid_location + direction.dir();

        let is_dead_end = Direction::all()
            .iter()
            .filter(|&dir| dir != &direction)
            .all(|dir| {
                let location = self.droid_location + dir.dir();
                match self.visited_locations.get(&location) {
                    None => false, // Unknown location
                    Some(&tile) => tile == Tile::Wall,
                }
            });

        let neighbours_explored = Direction::all().iter().all(|dir| {
            let location = self.droid_location + dir.dir();
            match self.visited_locations.get(&location) {
                None => false, // Unknown location
                Some(&tile) => tile != Tile::Wall,
            }
        });

        self.visited_locations
            .entry(location)
            .and_modify(|e| {
                *e = new_tile;
                // e.is_dead_end = is_dead_end;
                // e.neighbours_explored = neighbours_explored;
            })
            .or_insert_with(|| {
                let mut tile = new_tile;
                // tile.is_dead_end = is_dead_end;
                // tile.neighbours_explored = neighbours_explored;

                tile
            });

        if new_tile == Tile::OxygenSystem {
            self.oxygen_location = Some(location);
        }

        match new_tile {
            Tile::Empty | Tile::OxygenSystem => {
                // Move
                if self.behavior == Behavior::Exploring {
                    self.path.push_back(direction);
                }
                self.droid_location = self.droid_location + direction.dir()
            }
            _ => (),
        }

        self.explored_everything = self.oxygen_location.is_some() && self.path.is_empty();
    }

    pub fn known_bounds(&self) -> (Location, Location) {
        let max = Location::new(
            self.visited_locations
                .keys()
                .max_by_key(|l| l.x)
                .unwrap()
                .x
                .max(self.droid_location.x)
                .max(5),
            self.visited_locations
                .keys()
                .max_by_key(|l| l.y)
                .unwrap()
                .y
                .max(self.droid_location.y)
                .max(5),
        );

        let min = Location::new(
            self.visited_locations
                .keys()
                .min_by_key(|l| l.x)
                .unwrap()
                .x
                .min(self.droid_location.x)
                .min(-5),
            self.visited_locations
                .keys()
                .min_by_key(|l| l.y)
                .unwrap()
                .y
                .min(self.droid_location.y)
                .min(-5),
        );

        (max, min)
    }

    pub fn shortest_path(&self, from: Location, to: Location) -> Option<Vec<Location>> {
        // A Star

        let mut open: HashSet<Location> = std::iter::once(from).collect();
        let mut came_from: HashMap<Location, Location> = HashMap::default();
        let mut g_score: HashMap<Location, usize> = std::iter::once((from, 0)).collect();
        let mut f_score: HashMap<Location, usize> =
            std::iter::once((from, from.manhattan_distance(to) as usize)).collect();

        while !open.is_empty() {
            let current = open
                .iter()
                .filter_map(|l| f_score.get(l).map(|score| (l, score)))
                .min_by_key(|&(_, score)| score)
                .map(|(&l, _)| l)
                .unwrap();

            if current == to {
                let mut path = vec![current];

                let mut current = Some(current);

                while current.is_some() {
                    current = came_from.get(&current.unwrap()).map(|c| c.clone());
                    if current.is_some() {
                        path.push(current.unwrap());
                    }
                }

                return Some(path);
            }

            open.remove(&current);

            let neighbors = Direction::all().iter().filter_map(|dir| {
                let location = current + dir.dir();

                match self.visited_locations.get(&location) {
                    Some(&tile) => {
                        if tile != Tile::Wall {
                            Some(location)
                        } else {
                            None
                        }
                    }
                    None => None,
                }
            });

            for neighbor in neighbors {
                let tenative_score = g_score.get(&current).unwrap_or(&std::usize::MAX) + 1;

                if tenative_score < *g_score.get(&neighbor).unwrap_or(&std::usize::MAX) {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tenative_score);
                    f_score.insert(
                        neighbor,
                        tenative_score + neighbor.manhattan_distance(to) as usize,
                    );

                    if !open.contains(&neighbor) {
                        open.insert(neighbor);
                    }
                }
            }
        }

        None
    }
}

impl fmt::Debug for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (max, min) = self.known_bounds();

        let lines: Vec<_> = (min.y..=max.y)
            .rev()
            .map(|y| {
                (min.x..=max.x)
                    .map(move |x| {
                        let location = Location::new(x, y);

                        if location == self.droid_location {
                            "D"
                        } else if location == Location::default() {
                            "X"
                        } else {
                            match self.visited_locations.get(&location) {
                                None => "?",
                                Some(tile) => match tile {
                                    Tile::Empty => ".",
                                    Tile::Wall => "#",
                                    Tile::OxygenSystem => "O",
                                },
                            }
                        }
                    })
                    .collect::<String>()
            })
            .collect();

        write!(f, "\n{}\n", lines.join("\n"))
    }
}

fn explore_world(input: &str) -> World {
    let program: Vec<isize> = parse_custom_separated(input, ",").collect();
    let next_input: RefCell<Direction> = RefCell::new(Direction::North);
    let mut world = World::new();
    let mut computer = Computer::new(program);
    computer.set_input(|| Some((*next_input.borrow()) as isize));

    loop {
        computer.run_until_halt_or_paused(true);
        let status = Status::try_from(computer.last_output().unwrap())
            .expect("Expected to be able to parse status");

        let direction = *next_input.borrow();
        world.update(direction, status);

        if world.explored_everything {
            break;
        }

        *next_input.borrow_mut() = world.next_direction();
    }

    world
}

pub fn star_one(input: &str) -> usize {
    let world = explore_world(input);

    let oxygen_location = world.oxygen_location.unwrap();

    let path = world
        .shortest_path(Location::default(), oxygen_location)
        .unwrap();

    path.len() - 1
}

pub fn star_two(input: &str) -> usize {
    let world = explore_world(input);
    let mut oxidized: HashSet<Location> = std::iter::once(
        world
            .oxygen_location
            .expect("Oxygen location should have been found after exploring the world"),
    )
    .collect();
    let mut oxygen_edge = oxidized.clone();
    let mut minutes = 0;

    loop {
        let adjacent: HashSet<Location> = oxygen_edge
            .iter()
            .flat_map(|&location| world.adjacent_open_locations(location).into_iter())
            .collect();

        let previous_oxidized = oxidized.len();
        oxygen_edge = adjacent.difference(&oxidized).cloned().collect();
        oxidized = oxidized.union(&adjacent).cloned().collect();

        if previous_oxidized == oxidized.len() {
            break;
        }

        minutes += 1;
    }

    minutes
}
