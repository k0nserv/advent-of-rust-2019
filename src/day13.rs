use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;

use crate::intcode_computer::Computer;
use crate::parse_custom_separated;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty = 0,
    Wall,
    Block,
    HPaddle,
    Ball,
}

impl TryFrom<isize> for Tile {
    type Error = String;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Tile::Empty),
            1 => Ok(Tile::Wall),
            2 => Ok(Tile::Block),
            3 => Ok(Tile::HPaddle),
            4 => Ok(Tile::Ball),
            _ => Err(format!("Invalid Tile {}", value)),
        }
    }
}

type Location = (isize, isize);

pub fn star_one(input: &str) -> usize {
    let program: Vec<isize> = parse_custom_separated(input, ",").collect();
    let mut computer = Computer::new(program);
    computer.set_input(|| Some(0));
    let mut tiles: HashMap<Location, Tile> = HashMap::new();

    while !computer.is_halted() {
        computer.run_until_halt_or_paused(true);
        // X
        let x = computer.last_output().unwrap();

        computer.run_until_halt_or_paused(true);
        // Y
        let y = computer.last_output().unwrap();

        computer.run_until_halt_or_paused(true);
        // Tile
        let tile = Tile::try_from(computer.last_output().unwrap()).unwrap();

        tiles
            .entry((x, y))
            .and_modify(|e| *e = tile)
            .or_insert(tile);
    }

    tiles.values().filter(|&tile| tile == &Tile::Block).count()
}

pub fn star_two(input: &str) -> isize {
    let mut program: Vec<isize> = parse_custom_separated(input, ",").collect();
    program[0] = 2; // Add quarters
    let mut computer = Computer::new(program);
    let next_input: RefCell<isize> = RefCell::new(0);
    computer.set_input(|| Some(*next_input.borrow()));
    let mut tiles: HashMap<Location, Tile> = HashMap::new();
    let mut score = None;
    let mut paddle_target_location: Option<isize> = None;
    let mut paddle_location: Option<isize> = None;

    loop {
        computer.run_until_halt_or_paused(true);
        // X
        let x = computer.last_output().unwrap();

        if x == -1 || computer.is_halted() {
            computer.run_until_halt_or_paused(true);
            computer.run_until_halt_or_paused(true);

            score = computer.last_output();
        } else {
            computer.run_until_halt_or_paused(true);
            // Y
            let y = computer.last_output().unwrap();

            computer.run_until_halt_or_paused(true);
            // Tile
            let tile = Tile::try_from(computer.last_output().unwrap()).unwrap();

            match tile {
                Tile::Ball => {
                    paddle_target_location = Some(x);
                }
                Tile::HPaddle => {
                    paddle_location = Some(x);
                }
                _ => (),
            }

            *next_input.borrow_mut() = paddle_target_location
                .and_then(|target_location| {
                    paddle_location.map(|paddle_location| {
                        if target_location < paddle_location {
                            -1
                        } else if target_location > paddle_location {
                            1
                        } else {
                            0
                        }
                    })
                })
                .unwrap_or(0);

            tiles
                .entry((x, y))
                .and_modify(|e| *e = tile)
                .or_insert(tile);
        }

        if !tiles.values().any(|&v| v == Tile::Block) && computer.is_halted() {
            break;
        }
    }

    score.unwrap()
}
