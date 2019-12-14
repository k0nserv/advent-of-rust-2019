use crate::intcode_computer::Computer;
use crate::parse_custom_separated;

pub fn star_one(input: &str) -> isize {
    let program: Vec<_> = parse_custom_separated::<isize>(input, ",").collect();
    let mut computer = Computer::with_input(program, || Some(1));

    computer.run_until_halt_or_paused(false);

    computer
        .last_output()
        .expect("The program should have at least one output")
}

pub fn star_two(input: &str) -> isize {
    let program: Vec<_> = parse_custom_separated::<isize>(input, ",").collect();
    let mut computer = Computer::with_input(program, || Some(2));

    computer.run_until_halt_or_paused(false);

    computer
        .last_output()
        .expect("The program should have at least one output")
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use crate::intcode_computer::Computer;
    use crate::parse_custom_separated;
    const TEST_PROGRAM_1: &'static [isize] = &[
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];
    const TEST_PROGRAM_2: &'static [isize] = &[1102, 34915192, 34915192, 7, 4, 7, 99, 0];
    const TEST_PROGRAM_3: &'static [isize] = &[104, 1125899906842624, 99];

    #[test]
    fn test_extended_intcode_program1() {
        let program = Vec::from(TEST_PROGRAM_1);
        let mut computer = Computer::with_input(program, || Some(0));

        computer.run_until_halt_or_paused(false);

        assert_eq!(computer.all_outputs(), TEST_PROGRAM_1);
    }

    #[test]
    fn test_extended_intcode_program2() {
        let program = Vec::from(TEST_PROGRAM_2);
        let mut computer = Computer::with_input(program, || Some(0));

        computer.run_until_halt_or_paused(false);

        assert_eq!(computer.last_output(), Some(1219070632396864));
    }

    #[test]
    fn test_extended_intcode_program3() {
        let program = Vec::from(TEST_PROGRAM_3);
        let mut computer = Computer::with_input(program, || Some(0));

        computer.run_until_halt_or_paused(false);

        assert_eq!(computer.last_output(), Some(1125899906842624));
    }
}
