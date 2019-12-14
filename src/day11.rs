use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::f64::consts::PI;

use crate::intcode_computer::Computer;
use crate::parse_custom_separated;

#[derive(Debug, Copy, Clone)]
enum Color {
    Black = 0,
    White,
}

impl TryFrom<isize> for Color {
    type Error = String;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Color::Black),
            1 => Ok(Color::White),
            _ => Err(format!("Invalid color {}", value)),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::Black
    }
}

type Location = (isize, isize);

pub fn star_one(input: &str) -> usize {
    let mut direction = PI / 2.0;
    let mut current_location = RefCell::new((0, 0));
    let mut grid = RefCell::new(HashMap::<Location, Color>::new());
    let program = parse_custom_separated::<isize>(input, ",").collect();
    let mut computer = Computer::new(program);
    computer.set_input(|| {
        grid.borrow()
            .get(&current_location.borrow())
            .map(|&color| color as isize)
            .or(Some(Color::default() as isize))
    });

    while !computer.is_halted() {
        computer.run_until_halt_or_paused(true);
        let new_paint = Color::try_from(computer.last_output().unwrap())
            .expect("Should be able to parse value as color");

        grid.borrow_mut()
            .entry(current_location.borrow().clone())
            .and_modify(|entry| *entry = new_paint)
            .or_insert(new_paint);

        computer.run_until_halt_or_paused(true);
        match computer.last_output() {
            Some(0) => direction += PI / 2.0,
            Some(1) => direction -= PI / 2.0,
            _ => panic!("Invalid direction"),
        }

        current_location.replace_with(|location| {
            (
                location.0 + direction.cos() as isize,
                location.1 + direction.sin() as isize,
            )
        });
    }

    {
        let grid = grid.borrow();
        grid.len()
    }
}

pub fn star_two(input: &str) -> String {
    let mut direction = PI / 2.0;
    let current_location = RefCell::new((0, 0));
    let grid = RefCell::new(HashMap::<Location, Color>::new());
    grid.borrow_mut().insert((0, 0), Color::White);
    let program = parse_custom_separated::<isize>(input, ",").collect();
    let mut computer = Computer::new(program);
    computer.set_input(|| {
        grid.borrow()
            .get(&current_location.borrow())
            .map(|&color| color as isize)
            .or(Some(Color::default() as isize))
    });

    while !computer.is_halted() {
        computer.run_until_halt_or_paused(true);
        let new_paint = Color::try_from(computer.last_output().unwrap())
            .expect("Should be able to parse value as color");

        grid.borrow_mut()
            .entry(current_location.borrow().clone())
            .and_modify(|entry| *entry = new_paint)
            .or_insert(new_paint);

        computer.run_until_halt_or_paused(true);
        match computer.last_output() {
            Some(0) => direction -= PI / 2.0,
            Some(1) => direction += PI / 2.0,
            _ => panic!("Invalid direction"),
        }

        current_location.replace_with(|location| {
            (
                location.0 + direction.cos().round() as isize,
                location.1 + direction.sin().round() as isize,
            )
        });
    }

    let (max, min) = grid.borrow().keys().fold(
        (
            (std::isize::MIN, std::isize::MIN),
            (std::isize::MAX, std::isize::MAX),
        ),
        |(current_max, current_min), location| {
            let mut new_max = current_max;
            let mut new_min = current_min;

            if location.0 < current_min.0 {
                new_min.0 = location.0
            }

            if location.1 < current_min.1 {
                new_min.1 = location.1
            }

            if location.0 > current_max.0 {
                new_max.0 = location.0
            }

            if location.1 > current_max.1 {
                new_max.1 = location.1
            }

            (new_max, new_min)
        },
    );

    let borrowed_grid = &grid.borrow();
    (min.1 - 1..=max.1 + 1)
        .rev()
        .map(|y| {
            (min.0 - 1..=max.0 + 1)
                .rev()
                .map(move |x| {
                    borrowed_grid
                        .get(&(x, y))
                        .map(|color| match color {
                            Color::Black => '.',
                            Color::White => '#',
                        })
                        .unwrap_or('.')
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}
