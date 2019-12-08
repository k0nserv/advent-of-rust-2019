use crate::intcode_computer::Computer;
use crate::parse_custom_separated;
use itertools::iproduct;

fn run_until_halt(memory: Vec<i64>) -> i64 {
    let mut numbers: Vec<i64> = memory;
    let mut pc = 0;

    while numbers[pc] != 99 {
        let x_position = numbers[pc + 1] as usize;
        let y_position = numbers[pc + 2] as usize;
        let z_position = numbers[pc + 3] as usize;

        match numbers[pc] {
            1 => {
                numbers[z_position] = numbers[x_position] + numbers[y_position];
            }
            2 => {
                numbers[z_position] = numbers[x_position] * numbers[y_position];
            }
            _ => panic!("Unknown opcode {}", numbers[pc]),
        }

        pc += 4
    }

    numbers[0]
}

pub fn star_one(input: &str) -> isize {
    let program = parse_custom_separated::<isize>(input, ",").collect();
    let mut computer = Computer::new(program, 1);
    computer.run_until_halt_or_paused(false);

    computer.memory()[0]
}

pub fn star_two(input: &str) -> isize {
    let program: Vec<_> = parse_custom_separated::<isize>(input, ",").collect();

    let (noun, verb) = iproduct!((0..=99), (0..=99))
        .find(|&(noun, verb)| {
            let mut modified_computer = Computer::new(program.clone(), 1);
            modified_computer.memory_mut()[1] = noun;
            modified_computer.memory_mut()[2] = verb;

            modified_computer.run_until_halt_or_paused(false);

            modified_computer.memory()[0] == 19690720
        })
        .expect("There should be a noun and verb that results in the output `19690720`");

    100 * noun + verb
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one("1,9,10,3,2,3,11,0,99,30,40,50"), 3500);
    }
}
