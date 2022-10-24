use std::cell::RefCell;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::io::Write;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use ansi_term::Color::{Black, Blue, Green, Red, White, Yellow};
use ansi_term::{ANSIByteString, ANSIByteStrings, Style};
use thread_priority::*;

use advent_of_rust_2019::day15::{Direction, Location, Status, Tile, World};
use advent_of_rust_2019::intcode_computer::Computer;
use advent_of_rust_2019::{load_file, parse_custom_separated};

const RENDER_TICK_RATE: Duration = Duration::from_millis(16);
const SIMULATION_TICK_RATE: Duration = Duration::from_millis(4);

const CLS: &[u8] = "\x1B[2J".as_bytes();
const SCS: &[u8] = "\x1B[?25h".as_bytes();
const HCS: &[u8] = "\x1B[?25l".as_bytes();
const BSU: &[u8] = "\x1B[?2026h".as_bytes();
const ESU: &[u8] = "\x1B[?2026l".as_bytes();

type Diff = Vec<((usize, usize), Sprite)>;

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

fn diff_sprites(current: &[Vec<Sprite>], new: &[Vec<Sprite>]) -> Diff {
    (0..new.len())
        .flat_map(|y| {
            (0..new[y].len()).filter_map(move |x| {
                let old = current.get(y).and_then(|inner| inner.get(x));

                if old
                    .map(|&old_sprite| old_sprite != new[y][x])
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

fn print_diffs<IO: Write>(
    diffs: &[((usize, usize), Sprite)],
    io: &mut IO,
    supports_synchronized_output: bool,
) {
    let tmp_style = Style::new();

    let draw_commands = diffs.iter().flat_map(|((x, y), sprite)| {
        std::iter::once(tmp_style.paint(format!("\x1B[{};{}H", y + 1, x + 1).as_bytes().to_owned()))
            .chain(std::iter::once(sprite.as_ansi()))
    });

    let draw: Vec<_> = if supports_synchronized_output {
        std::iter::once(tmp_style.paint(BSU))
            .chain(draw_commands)
            .chain(std::iter::once(tmp_style.paint(ESU)))
            .collect()
    } else {
        draw_commands.collect()
    };

    ANSIByteStrings(&draw).write_to(io).unwrap();
    io.flush().unwrap();
}

fn clear_screen<IO: Write>(io: &mut IO) {
    io.write(CLS).expect("Failed to clear screen");
}

fn set_cursor_visiblity<IO: Write>(visible: bool, io: &mut IO) {
    if visible {
        io.write(SCS).expect("Failed to set cursor visibility");
    } else {
        io.write(HCS).expect("Failed to set cursor visibility");
    }
}

fn explore_world(
    input: &str,
    sleep_duration: Duration,
    tx: mpsc::Sender<Diff>,
) -> (World, Vec<Vec<Sprite>>, mpsc::Sender<Diff>) {
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
        tx.send(diff).expect("Render thread unexpectedly gone");
        last_sprite_map = sprites;
        std::thread::sleep(sleep_duration);

        if world.done_exploring() {
            break;
        }

        *next_input.borrow_mut() = world.next_direction();
    }

    (world, last_sprite_map, tx)
}

fn run_render_thread(rx: mpsc::Receiver<Diff>) {
    set_thread_priority_and_policy(
        thread_native_id(),
        ThreadPriority::Crossplatform(ThreadPriorityValue::try_from(40).unwrap()),
        ThreadSchedulePolicy::Realtime(RealtimeThreadSchedulePolicy::Fifo),
    )
    .expect("Failed to set thread priority");

    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    clear_screen(&mut handle);
    set_cursor_visiblity(false, &mut handle);

    loop {
        match rx.try_recv() {
            Ok(commands) => print_diffs(&commands, &mut handle, true),
            Err(mpsc::TryRecvError::Empty) => {
                /* No action needed, currently rendered view is up to date */
            }
            Err(mpsc::TryRecvError::Disconnected) => break,
        }

        thread::sleep(RENDER_TICK_RATE);
    }

    set_cursor_visiblity(true, &mut handle);
}

fn main() {
    let file = load_file("day15.txt");

    let (tx, rx) = mpsc::channel::<Diff>();
    let render_thread = thread::spawn(move || run_render_thread(rx));

    let (world, mut last_sprite_map, tx) = explore_world(&file, SIMULATION_TICK_RATE, tx);
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
        tx.send(diff).expect("Render thread unexpectedly gone");
        last_sprite_map = sprites;
        std::thread::sleep(SIMULATION_TICK_RATE);
    }

    // Prevent off by one
    let sprites = world_to_sprite_map(
        &world,
        world.done_exploring(),
        Some(&path.iter().cloned().collect()),
        None,
    );
    let diff = diff_sprites(&last_sprite_map, &sprites);
    tx.send(diff).expect("Render thread unexpectedly gone");
    last_sprite_map = sprites;

    std::thread::sleep(Duration::from_millis(500));

    let mut oxidized: HashSet<Location> = std::iter::once(
        world
            .oxygen_location
            .expect("Oxygen location should have been found after exploring the world"),
    )
    .collect();
    let mut oxygen_edge = oxidized.clone();

    loop {
        let sprites = world_to_sprite_map(&world, world.done_exploring(), None, Some(&oxidized));
        let diff = diff_sprites(&last_sprite_map, &sprites);
        tx.send(diff).expect("Render thread unexpectedly gone");
        last_sprite_map = sprites;
        std::thread::sleep(SIMULATION_TICK_RATE);
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
    tx.send(diff).expect("Render thread unexpectedly gone");

    // Sleep one second to show off the finished thing
    std::thread::sleep(Duration::from_secs(1));

    // Drop tx, stops render thread
    drop(tx);
    render_thread.join().expect("Failed to join render thread");
}
