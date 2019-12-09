use crate::intcode_computer::Computer;
use crate::parse_custom_separated;

use itertools::iproduct;

use std::cell::RefCell;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::rc::Rc;

fn thruster_signal(program: Vec<isize>, phase_settings: &[isize], feedback: bool) -> isize {
    let mut computers: Vec<_> = phase_settings
        .iter()
        .map(|setting| Computer::new(program.clone(), *setting))
        .collect();
    let last_outputs: Rc<RefCell<Vec<Option<isize>>>> =
        Rc::new(RefCell::new(phase_settings.iter().map(|_| None).collect()));
    let has_sent_initial_signal = RefCell::new(false);

    let outputs = Rc::clone(&last_outputs);
    computers[0].set_dynamic_input(Box::new(move || {
        let mut has_sent_initial_signal = has_sent_initial_signal.borrow_mut();
        if feedback {
            if *has_sent_initial_signal {
                let borrowed_outputs = outputs.borrow();

                borrowed_outputs[borrowed_outputs.len() - 1]
            } else {
                *has_sent_initial_signal = true;

                Some(0)
            }
        } else {
            Some(0)
        }
    }));
    for idx in 1..phase_settings.len() {
        let outputs = Rc::clone(&last_outputs);
        computers[idx].set_dynamic_input(Box::new(move || outputs.borrow()[idx - 1]));
    }

    let mut all_halted = false;
    while !all_halted {
        let unhalted_computers = computers
            .iter_mut()
            .enumerate()
            .filter(|(_, computer)| !computer.is_halted());

        for (idx, computer) in unhalted_computers {
            computer.run_until_halt_or_paused(true);
            last_outputs.borrow_mut()[idx] = computer.last_output();
        }

        all_halted = computers.iter().all(|computer| computer.is_halted())
    }

    {
        let max_thruster_signal = last_outputs.borrow()[phase_settings.len() - 1].unwrap_or(0);

        max_thruster_signal
    }
}

fn find_max_thruster_signal(
    program: Vec<isize>,
    phaser_range: RangeInclusive<isize>,
    feedback: bool,
) -> isize {
    iproduct!(
        phaser_range.clone(),
        phaser_range.clone(),
        phaser_range.clone(),
        phaser_range.clone(),
        phaser_range.clone()
    )
    .filter_map(|(p1, p2, p3, p4, p5)| {
        let phase_setting = [p1, p2, p3, p4, p5];
        let unique_settings: HashSet<_> = phase_setting.into_iter().collect();

        if unique_settings.len() < phase_setting.len() {
            None
        } else {
            let result = thruster_signal(program.clone(), &phase_setting, feedback);

            Some(result)
        }
    })
    .max()
    .expect("There should be a max thruster signal")
}

pub fn star_one(input: &str) -> isize {
    let program: Vec<_> = parse_custom_separated::<isize>(input, ",").collect();
    find_max_thruster_signal(program, 0..=4, false)
}

pub fn star_two(input: &str) -> isize {
    let program: Vec<_> = parse_custom_separated::<isize>(input, ",").collect();
    find_max_thruster_signal(program, 5..=9, true)
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two, thruster_signal};
    use crate::parse_custom_separated;
    const TEST_CASES_PART_1: [(&str, [isize; 5], isize); 3] = [
            ("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0", [0,1,2,3,4], 54321),
            ("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", [4, 3, 2, 1, 0], 43210),
            ("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0", [1,0,4,3,2], 65210),
        ];

    const TEST_CASES_PART_2: [(&str, [isize; 5], isize); 2] = [
            ("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5", [9,8,7,6,5], 139629729),
            ("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10", [9,7,8,5,6], 18216),
        ];

    #[test]
    fn test_thruster_signal() {
        for (input, phase_settings, max_thruster_signal) in &TEST_CASES_PART_1 {
            let program = parse_custom_separated::<isize>(input, ",").collect();
            assert_eq!(
                thruster_signal(program, phase_settings, false,),
                *max_thruster_signal
            );
        }
    }

    #[test]
    fn test_thruster_signal_feedback_mode() {
        for (input, phase_settings, max_thruster_signal) in &TEST_CASES_PART_2 {
            let program = parse_custom_separated::<isize>(input, ",").collect();
            assert_eq!(
                thruster_signal(program, phase_settings, true),
                *max_thruster_signal
            );
        }
    }

    #[test]
    fn test_star_one() {
        for (input, phase_settings, max_thruster_signal) in &TEST_CASES_PART_1 {
            assert_eq!(
                star_one(input),
                *max_thruster_signal,
                "Expected max thruster signal of {} for program `{}`",
                max_thruster_signal,
                input
            );
        }
    }

    #[test]
    fn test_star_two() {
        for (input, phase_settings, max_thruster_signal) in &TEST_CASES_PART_2 {
            assert_eq!(
                star_two(input),
                *max_thruster_signal,
                "Expected max thruster signal of {} for program `{}`",
                max_thruster_signal,
                input
            );
        }
    }
}
