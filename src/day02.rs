fn parse(input: &str) -> impl Iterator<Item = i64> + '_ {
    input
        .split(",")
        .map(str::trim)
        .filter(|v| v.len() > 0)
        .map(|value| value.parse().expect("Expect only parsable numbers"))
}

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

pub fn star_one(input: &str) -> i64 {
    let numbers: Vec<i64> = parse(input).collect();
    run_until_halt(numbers)
}

pub fn star_two(input: &str) -> i64 {
    let program: Vec<i64> = parse(input).collect();

    let (noun, verb) = (0..=99)
        .flat_map(|noun| (0..=99).map(move |verb| (noun, verb)))
        .find(|&(noun, verb)| {
            let mut modified_program = program.clone();
            modified_program[1] = noun;
            modified_program[2] = verb;

            run_until_halt(modified_program) == 19690720
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
