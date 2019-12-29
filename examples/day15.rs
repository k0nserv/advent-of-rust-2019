use std::cell::RefCell;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::io::Write;
use std::time::Duration;

use ansi_term::Color::{Black, Blue, Green, Red, White, Yellow};
use ansi_term::{ANSIByteString, ANSIByteStrings, Style};

use advent_of_rust_2019::day15::{Direction, Location, Status, Tile, World};
use advent_of_rust_2019::intcode_computer::Computer;
use advent_of_rust_2019::{load_file, parse_custom_separated};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Sprite {
    Unknown,
    Empty,
    Wall,
    Oxygen,
    Droid,
    Origin,
    PathSegment,
}

impl Sprite {
    fn as_ansi(&self) -> ANSIByteString {
        match self {
            Sprite::Oxygen => Blue.paint("\u{2588}".as_bytes()),
            Sprite::PathSegment => Red.paint("\u{2588}".as_bytes()),
            Sprite::Droid => Green.paint("\u{2588}".as_bytes()),
            Sprite::Origin => Yellow.paint("\u{2588}".as_bytes()),
            Sprite::Wall => Black.paint("\u{2588}".as_bytes()),
            Sprite::Unknown => Black.dimmed().paint("\u{2588}".as_bytes()),
            Sprite::Empty => White.paint("\u{2588}".as_bytes()),
        }
    }
}

fn world_to_sprite_map(
    world: &World,
    done: bool,
    path: Option<&HashSet<Location>>,
    oxygen: Option<&HashSet<Location>>,
) -> Vec<Vec<Sprite>> {
    let (max, min) = world.known_bounds((80, 60));

    (min.y..=max.y)
        .rev()
        .map(|y| {
            (min.x..=max.x)
                .map(move |x| {
                    let location = Location::new(x, y);

                    match oxygen {
                        Some(oxygen) => {
                            if oxygen.contains(&location) {
                                return Sprite::Oxygen;
                            }
                        }
                        None => (),
                    }

                    match path {
                        Some(path) => {
                            if path.contains(&location) {
                                return Sprite::PathSegment;
                            }
                        }
                        None => (),
                    }

                    if location == world.droid_location {
                        Sprite::Droid
                    } else if location == Location::default() {
                        Sprite::Origin
                    } else {
                        match world.visited_locations.get(&location) {
                            None => {
                                if done {
                                    Sprite::Wall
                                } else {
                                    Sprite::Unknown
                                }
                            }
                            Some(&tile) => match tile {
                                Tile::Empty => Sprite::Empty,
                                Tile::Wall => Sprite::Wall,
                                Tile::OxygenSystem => Sprite::Oxygen,
                            },
                        }
                    }
                })
                .collect()
        })
        .collect()
}

fn diff_sprites(current: &[Vec<Sprite>], new: &[Vec<Sprite>]) -> Vec<((usize, usize), Sprite)> {
    (0..new.len())
        .flat_map(|y| {
            (0..new[y].len()).filter_map(move |x| {
                let old = current.get(y).and_then(|inner| inner.get(x));

                if old
                    .map(|&old_sprite| old_sprite == new[y][x])
                    .unwrap_or(true)
                {
                    Some(((x, y), new[y][x]))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn print_diffs(diffs: &[((usize, usize), Sprite)]) {
    let tmp_style = Style::new();

    let draw: Vec<_> = diffs
        .iter()
        .flat_map(|((x, y), sprite)| {
            std::iter::once(
                tmp_style.paint(format!("\x1B[{};{}H", y + 1, x + 1).as_bytes().to_owned()),
            )
            .chain(std::iter::once(sprite.as_ansi()))
        })
        .collect();

    ANSIByteStrings(&draw)
        .write_to(&mut std::io::stdout())
        .unwrap();
}

fn clear_screen() {
    print!("\x1B[2J");
}

fn print_world(sprites: &[Vec<Sprite>]) {
    let lines: Vec<_> = std::iter::once(Style::new().paint("\x1B[2J\x1B[1;1H".as_bytes()))
        .chain(sprites.iter().flat_map(|row| {
            row.iter()
                .map(|sprite| sprite.as_ansi())
                .chain(std::iter::once(Style::new().paint("\n".as_bytes())))
        }))
        .collect();

    std::io::stdout().flush().unwrap();
    let string = ANSIByteStrings(&lines)
        .write_to(&mut std::io::stdout())
        .unwrap();
}

fn explore_world(input: &str, sleep_duration: Duration) -> (World, Vec<Vec<Sprite>>) {
    let program: Vec<isize> = parse_custom_separated(input, ",").collect();
    let next_input: RefCell<Direction> = RefCell::new(Direction::North);
    let mut world = World::new();
    let mut computer = Computer::new(program);
    computer.set_input(|| Some((*next_input.borrow()) as isize));
    let mut last_sprite_map: Vec<Vec<Sprite>> = vec![];

    loop {
        computer.run_until_halt_or_paused(true);
        let status = Status::try_from(computer.last_output().unwrap())
            .expect("Expected to be able to parse status");

        let direction = *next_input.borrow();
        world.update(direction, status);

        let sprites = world_to_sprite_map(&world, world.done_exploring(), None, None);
        let diff = diff_sprites(&last_sprite_map, &sprites);
        print_diffs(&diff);
        last_sprite_map = sprites;
        std::thread::sleep(sleep_duration);

        if world.done_exploring() {
            break;
        }

        *next_input.borrow_mut() = world.next_direction();
    }

    (world, last_sprite_map)
}

fn main() {
    clear_screen();
    let file = load_file("day15.txt");
    let speed = Duration::from_millis(16);

    let (world, mut last_sprite_map) = explore_world(&file, speed);
    let mut path = world
        .shortest_path(Default::default(), world.oxygen_location.unwrap())
        .unwrap();
    path.reverse();
    std::thread::sleep(Duration::from_millis(500));

    for i in 1..path.len() {
        let partial_path: HashSet<Location> = path.iter().take(i).cloned().collect();
        let sprites =
            world_to_sprite_map(&world, world.done_exploring(), Some(&partial_path), None);
        let diff = diff_sprites(&last_sprite_map, &sprites);
        print_diffs(&diff);
        last_sprite_map = sprites;
        std::thread::sleep(speed);
    }

    // Prevent off by one
    let sprites = world_to_sprite_map(
        &world,
        world.done_exploring(),
        Some(&path.iter().cloned().collect()),
        None,
    );
    let diff = diff_sprites(&last_sprite_map, &sprites);
    print_diffs(&diff);
    last_sprite_map = sprites;

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
        let sprites = world_to_sprite_map(&world, world.done_exploring(), None, Some(&oxidized));
        let diff = diff_sprites(&last_sprite_map, &sprites);
        print_diffs(&diff);
        last_sprite_map = sprites;
        std::thread::sleep(speed);
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

    // Prevent off by one
    let sprites = world_to_sprite_map(&world, world.done_exploring(), None, Some(&oxidized));
    let diff = diff_sprites(&last_sprite_map, &sprites);
    print_diffs(&diff);
    last_sprite_map = sprites;
}
