use crate::intcode_computer::Computer;
use crate::parse_custom_separated;

fn yield_one() -> Option<isize> {
    Some(1)
}
pub fn star_one(input: &str) -> isize {
    let program = parse_custom_separated::<isize>(input, ",").collect();
    let mut computer = Computer::with_input(program, yield_one);
    computer.run_until_halt_or_paused(false);
    computer.last_output().expect("There should be an output")
}

pub fn star_two(input: &str) -> isize {
    let program = parse_custom_separated::<isize>(input, ",").collect();
    let mut computer = Computer::with_input(program, || Some(5));
    computer.run_until_halt_or_paused(false);
    computer.last_output().expect("There should be an output")
}
