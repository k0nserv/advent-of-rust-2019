use std::cell::RefCell;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::io::Write;
use std::time::Duration;

use ansi_term::Color::{Black, Blue, Green, Red, White, Yellow};
use ansi_term::{ANSIByteStrings, Style};

use advent_of_rust_2019::day15::{Direction, Location, Status, Tile, World};
use advent_of_rust_2019::intcode_computer::Computer;
use advent_of_rust_2019::{load_file, parse_custom_separated};

fn print_world(
    world: &World,
    done: bool,
    path: Option<&HashSet<Location>>,
    oxygen: Option<&HashSet<Location>>,
) {
    let (max, min) = world.known_bounds();

    let lines: Vec<_> = std::iter::once(Style::new().paint("\x1B[2J\n".as_bytes()))
        .chain((min.y..=max.y).rev().flat_map(|y| {
            (min.x..=max.x)
                .map(move |x| {
                    let location = Location::new(x, y);

                    match oxygen {
                        Some(oxygen) => {
                            if oxygen.contains(&location) {
                                return Blue.paint("\u{2588}".as_bytes());
                            }
                        }
                        None => (),
                    }

                    match path {
                        Some(path) => {
                            if path.contains(&location) {
                                return Red.paint("\u{2588}".as_bytes());
                            }
                        }
                        None => (),
                    }

                    if location == world.droid_location {
                        Green.paint("\u{2588}".as_bytes())
                    } else if location == Location::default() {
                        Yellow.paint("\u{2588}".as_bytes())
                    } else {
                        match world.visited_locations.get(&location) {
                            None => {
                                if done {
                                    Black.paint("\u{2588}".as_bytes())
                                } else {
                                    Black.dimmed().paint("\u{2588}".as_bytes())
                                }
                            }
                            Some(&tile) => match tile {
                                Tile::Empty => White.paint("\u{2588}".as_bytes()),
                                Tile::Wall => Black.paint("\u{2588}".as_bytes()),
                                Tile::OxygenSystem => Blue.paint("\u{2588}".as_bytes()),
                            },
                        }
                    }
                })
                .chain(std::iter::once(Style::new().paint("\n".as_bytes())))
        }))
        .collect();

    std::io::stdout().flush().unwrap();
    let string = ANSIByteStrings(&lines)
        .write_to(&mut std::io::stdout())
        .unwrap();
}

fn explore_world(input: &str, sleep_duration: Duration) -> World {
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
        print_world(&world, world.done_exploring(), None, None);
        std::thread::sleep(sleep_duration);

        if world.done_exploring() {
            break;
        }

        *next_input.borrow_mut() = world.next_direction();
    }

    world
}

fn main() {
    let file = load_file("day15.txt");

    let world = explore_world(&file, Duration::from_millis(25));
    let mut path = world
        .shortest_path(Default::default(), world.oxygen_location.unwrap())
        .unwrap();
    path.reverse();
    std::thread::sleep(Duration::from_millis(500));

    for i in 1..path.len() {
        let partial_path: HashSet<Location> = path.iter().take(i).cloned().collect();
        print_world(&world, world.done_exploring(), Some(&partial_path), None);
        std::thread::sleep(Duration::from_millis(25));
    }

    std::thread::sleep(Duration::from_millis(500));

    let mut oxidized: HashSet<Location> = std::iter::once(
        world
            .oxygen_location
            .expect("Oxygen location should have been found after exploring the world"),
    )
    .collect();
    let mut oxygen_edge = oxidized.clone();
    let mut minutes = 0;

    loop {
        print_world(&world, world.done_exploring(), None, Some(&oxidized));
        std::thread::sleep(Duration::from_millis(25));
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
    }
}
