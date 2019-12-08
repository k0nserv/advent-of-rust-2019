use crate::intcode_computer::Computer;
use crate::parse_custom_separated;

pub fn star_one(input: &str) -> isize {
    let program = parse_custom_separated::<isize>(input, ",").collect();
    let mut computer = Computer::new(program, 1);
    computer.run_until_halt_or_paused(false);
    computer.output().expect("There should be an output")
}

pub fn star_two(input: &str) -> isize {
    let program = parse_custom_separated::<isize>(input, ",").collect();
    let mut computer = Computer::new(program, 5);
    computer.run_until_halt_or_paused(false);
    computer.output().expect("There should be an output")
}
