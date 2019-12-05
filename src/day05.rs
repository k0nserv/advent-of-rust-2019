use crate::{parse_custom_separated, DigitIterator};

#[derive(Debug)]
enum Parameter {
    Immediate(isize),
    Position(usize),
}

impl Parameter {
    fn new(mode: usize, value: isize) -> Self {
        match mode {
            0 => Self::Position(value as usize),
            1 => Self::Immediate(value),
            _ => panic!("Unsupported paramter mode: `{}`", mode),
        }
    }

    fn value(&self, memory: &[isize]) -> isize {
        match self {
            &Self::Immediate(value) => value,
            &Self::Position(position) => memory[position],
        }
    }
}

#[derive(Debug)]
enum Op {
    Add,
    Multiply,
}

#[derive(Debug)]
enum JumpCondition {
    IfTrue,
    IfFalse,
}

#[derive(Debug)]
enum ComparisonOp {
    LessThan,
    Equal,
}

#[derive(Debug)]
enum Instruction {
    Op(Op, Parameter, Parameter, usize), // Used for Add and Multiply
    Input(usize),                        // 3
    Output(Parameter),                   // 4
    ConditionalJump(JumpCondition, Parameter, Parameter), // Used for JumpIfTrue and JumpIfFalse
    Compare(ComparisonOp, Parameter, Parameter, usize), // Used for LessThan and Equals
    Halt,                                // 99
}

impl Instruction {
    fn should_halt(&self) -> bool {
        match self {
            Self::Halt => true,
            _ => false,
        }
    }
    fn length(&self) -> usize {
        match self {
            Self::Op(_, _, _, _) | Self::Compare(_, _, _, _) => 4,
            Self::ConditionalJump(_, _, _) => 3,
            Self::Input(_) | Self::Output(_) => 2,
            Self::Halt => 1,
        }
    }

    fn parse(input: &[isize]) -> Self {
        let opcode = input[0];
        assert!(opcode >= 0);

        let mut digits = DigitIterator::new(opcode as usize);
        let ones_digit = digits.next().expect(&format!(
            "Opcode should have at least a ones digit faile for opcode: {}",
            opcode
        ));
        let _ = digits.next().unwrap_or(0);
        let hundreds_digit = digits.next().unwrap_or(0);
        let thousands_digit = digits.next().unwrap_or(0);
        let ten_thousands_digit = digits.next().unwrap_or(0);
        assert!(
            ten_thousands_digit == 0,
            "Expected ten_thousands_digit to be 0 was `{}` in opcode `{}`",
            ten_thousands_digit,
            opcode
        );

        match ones_digit {
            1 => Self::Op(
                Op::Add,
                Parameter::new(hundreds_digit, input[1]),
                Parameter::new(thousands_digit, input[2]),
                input[3] as usize,
            ),
            2 => Self::Op(
                Op::Multiply,
                Parameter::new(hundreds_digit, input[1]),
                Parameter::new(thousands_digit, input[2]),
                input[3] as usize,
            ),
            3 => Self::Input(input[1] as usize),
            4 => Self::Output(Parameter::new(hundreds_digit, input[1])),
            5 => Self::ConditionalJump(
                JumpCondition::IfTrue,
                Parameter::new(hundreds_digit, input[1]),
                Parameter::new(thousands_digit, input[2]),
            ),
            6 => Self::ConditionalJump(
                JumpCondition::IfFalse,
                Parameter::new(hundreds_digit, input[1]),
                Parameter::new(thousands_digit, input[2]),
            ),
            7 => Self::Compare(
                ComparisonOp::LessThan,
                Parameter::new(hundreds_digit, input[1]),
                Parameter::new(thousands_digit, input[2]),
                input[3] as usize,
            ),
            8 => Self::Compare(
                ComparisonOp::Equal,
                Parameter::new(hundreds_digit, input[1]),
                Parameter::new(thousands_digit, input[2]),
                input[3] as usize,
            ),
            9 => Self::Halt,
            _ => panic!("Invalid opcode `{}`", opcode),
        }
    }

    fn execute(&self, memory: &mut [isize], input: isize) -> (Option<usize>, Option<isize>) {
        match self {
            Self::Op(op, arg1, arg2, arg3) => {
                let a1 = arg1.value(memory);
                let a2 = arg2.value(memory);

                match op {
                    Op::Add => memory[*arg3] = a1 + a2,
                    Op::Multiply => memory[*arg3] = a1 * a2,
                }

                (None, None)
            }
            Self::ConditionalJump(condition, arg1, arg2) => {
                let a1 = arg1.value(memory);
                let a2 = arg2.value(memory);

                match condition {
                    JumpCondition::IfTrue => {
                        if a1 != 0 {
                            (Some(a2 as usize), None)
                        } else {
                            (None, None)
                        }
                    }
                    JumpCondition::IfFalse => {
                        if a1 == 0 {
                            (Some(a2 as usize), None)
                        } else {
                            (None, None)
                        }
                    }
                }
            }
            Self::Compare(op, arg1, arg2, arg3) => {
                let a1 = arg1.value(memory);
                let a2 = arg2.value(memory);

                match op {
                    ComparisonOp::LessThan => memory[*arg3] = if a1 < a2 { 1 } else { 0 },
                    ComparisonOp::Equal => memory[*arg3] = if a1 == a2 { 1 } else { 0 },
                }

                (None, None)
            }
            Self::Input(arg1) => {
                memory[*arg1] = input;
                (None, None)
            }
            Self::Output(arg1) => {
                let a1 = arg1.value(memory);

                (None, Some(a1))
            }
            Self::Halt => (None, None),
        }
    }
}

fn run_until_halt(input: &str, input_id: isize) -> Option<isize> {
    let mut raw: Vec<isize> = parse_custom_separated::<isize>(input, ",").collect();
    let mut current_index = 0;
    let mut last_output = None;

    while current_index < raw.len() {
        let parsed_instruction = if current_index < raw.len() - 4 {
            Instruction::parse(&raw[current_index..current_index + 4])
        } else {
            Instruction::parse(&raw[current_index..])
        };

        if parsed_instruction.should_halt() {
            break;
        }

        let (new_pc, new_last_output) = parsed_instruction.execute(&mut raw, input_id);
        last_output = new_last_output;
        current_index = new_pc.unwrap_or_else(|| current_index + parsed_instruction.length());

        assert!(last_output.unwrap_or(0) == 0 || raw[current_index] == 99);
    }

    last_output
}

pub fn star_one(input: &str) -> isize {
    run_until_halt(input, 1).expect("There should be an output")
}

pub fn star_two(input: &str) -> isize {
    run_until_halt(input, 5).expect("There should be an output")
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
}
