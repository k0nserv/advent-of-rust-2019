use crate::DigitIterator;

#[derive(Debug)]
enum Parameter {
    Immediate(isize),
    Position(usize),
    Relative(isize),
}

impl Parameter {
    fn new(mode: usize, value: isize) -> Self {
        match mode {
            0 => Self::Position(value as usize),
            1 => Self::Immediate(value),
            2 => Self::Relative(value),
            _ => panic!("Unsupported paramter mode: `{}`", mode),
        }
    }

    fn value(&self, memory: &[isize], relative_base_offset: usize) -> isize {
        match self {
            &Self::Immediate(value) => value,
            &Self::Position(position) => memory[position],
            &Self::Relative(_) => memory[self.address(relative_base_offset)],
        }
    }

    fn address(&self, relative_base_offset: usize) -> usize {
        match self {
            &Self::Position(position) => position,
            &Self::Relative(position) => ((relative_base_offset as isize) + position) as usize,
            &Self::Immediate(_) => {
                panic!("Attempted to use an immediate mode paramter for addressing")
            }
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
    Op(Op, Parameter, Parameter, Parameter), // Used for Add and Multiply
    Input(Parameter),                        // 3
    Output(Parameter),                       // 4
    ConditionalJump(JumpCondition, Parameter, Parameter), // Used for JumpIfTrue and JumpIfFalse
    Compare(ComparisonOp, Parameter, Parameter, Parameter), // Used for LessThan and Equals
    RelativeBaseAdjust(Parameter),           // 9
    Halt,                                    // 99
}

impl Instruction {
    fn should_halt(&self) -> bool {
        match self {
            Self::Halt => true,
            _ => false,
        }
    }

    fn is_input(&self) -> bool {
        match self {
            Self::Input(_) => true,
            _ => false,
        }
    }

    fn is_output(&self) -> bool {
        match self {
            Self::Output(_) => true,
            _ => false,
        }
    }

    fn length(&self) -> usize {
        match self {
            Self::Op(_, _, _, _) | Self::Compare(_, _, _, _) => 4,
            Self::ConditionalJump(_, _, _) => 3,
            Self::Input(_) | Self::Output(_) | Self::RelativeBaseAdjust(_) => 2,
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
        let tens_digit = digits.next().unwrap_or(0);
        let hundreds_digit = digits.next().unwrap_or(0);
        let thousands_digit = digits.next().unwrap_or(0);
        let ten_thousands_digit = digits.next().unwrap_or(0);

        match (tens_digit, ones_digit) {
            (_, 1) => Self::Op(
                Op::Add,
                Parameter::new(hundreds_digit, input[1]),
                Parameter::new(thousands_digit, input[2]),
                Parameter::new(ten_thousands_digit, input[3]),
            ),
            (_, 2) => Self::Op(
                Op::Multiply,
                Parameter::new(hundreds_digit, input[1]),
                Parameter::new(thousands_digit, input[2]),
                Parameter::new(ten_thousands_digit, input[3]),
            ),
            (_, 3) => {
                assert!(
                    hundreds_digit != 1,
                    "Immediate mode is not compatible with the Input opcode"
                );

                Self::Input(Parameter::new(hundreds_digit, input[1]))
            }
            (_, 4) => Self::Output(Parameter::new(hundreds_digit, input[1])),
            (_, 5) => Self::ConditionalJump(
                JumpCondition::IfTrue,
                Parameter::new(hundreds_digit, input[1]),
                Parameter::new(thousands_digit, input[2]),
            ),
            (_, 6) => Self::ConditionalJump(
                JumpCondition::IfFalse,
                Parameter::new(hundreds_digit, input[1]),
                Parameter::new(thousands_digit, input[2]),
            ),
            (_, 7) => Self::Compare(
                ComparisonOp::LessThan,
                Parameter::new(hundreds_digit, input[1]),
                Parameter::new(thousands_digit, input[2]),
                Parameter::new(ten_thousands_digit, input[3]),
            ),
            (_, 8) => Self::Compare(
                ComparisonOp::Equal,
                Parameter::new(hundreds_digit, input[1]),
                Parameter::new(thousands_digit, input[2]),
                Parameter::new(ten_thousands_digit, input[3]),
            ),
            (0, 9) => Self::RelativeBaseAdjust(Parameter::new(hundreds_digit, input[1])),
            (9, 9) => Self::Halt,
            _ => panic!("Invalid opcode `{}`", opcode),
        }
    }
}

enum ExecutionState {
    Halt,
    Pause,
    Continue,
}

pub fn input_with_initial_value<F, T>(
    intial_value: T,
    mut dynamic_value: F,
) -> impl FnMut() -> Option<T>
where
    T: Copy,
    F: FnMut() -> Option<T>,
{
    let mut has_yielded_initial_value = false;

    let closure = move || {
        if has_yielded_initial_value {
            dynamic_value()
        } else {
            has_yielded_initial_value = true;

            Some(intial_value)
        }
    };

    closure
}

const EXTENDED_MEMORY: [isize; 10_000] = [0; 10_000];

pub struct Computer<F>
where
    F: FnMut() -> Option<isize>,
{
    input: Option<F>,
    outputs: Vec<isize>,
    program: Vec<isize>,
    did_halt: bool,
    ip: usize,
    relative_base_offset: usize,
}

impl<F> Computer<F>
where
    F: FnMut() -> Option<isize>,
{
    pub fn new(program: Vec<isize>) -> Self {
        let mut program = program;
        program.extend(&EXTENDED_MEMORY[..]);

        Self {
            input: None,
            outputs: vec![],
            program,
            did_halt: false,
            ip: 0,
            relative_base_offset: 0,
        }
    }

    pub fn with_input(program: Vec<isize>, input: F) -> Self {
        let mut program = program;
        program.extend(&EXTENDED_MEMORY[..]);

        Self {
            input: Some(input),
            outputs: vec![],
            program,
            did_halt: false,
            ip: 0,
            relative_base_offset: 0,
        }
    }

    pub fn is_halted(&self) -> bool {
        self.did_halt
    }

    pub fn set_input(&mut self, input: F) {
        self.input = Some(input);
    }

    pub fn run_until_halt_or_paused(&mut self, stop_on_output: bool) {
        while self.ip < self.program.len() {
            let parsed_instruction = if self.ip < self.program.len() - 4 {
                Instruction::parse(&self.program[self.ip..self.ip + 4])
            } else {
                Instruction::parse(&self.program[self.ip..])
            };

            match self.execute_instruction(&parsed_instruction, stop_on_output) {
                ExecutionState::Halt => {
                    self.did_halt = true;
                    break;
                }
                ExecutionState::Pause => {
                    break;
                }
                ExecutionState::Continue => (),
            }
        }
    }

    pub fn last_output(&self) -> Option<isize> {
        self.outputs.last().map(|v| *v)
    }

    pub fn all_outputs(&self) -> &[isize] {
        &self.outputs
    }

    pub fn memory(&self) -> &[isize] {
        &self.program
    }

    pub fn memory_mut(&mut self) -> &mut [isize] {
        &mut self.program
    }

    fn read_input(&mut self) -> Option<isize> {
        self.input.as_mut().and_then(|input| input())
    }

    fn execute_instruction(
        &mut self,
        instruction: &Instruction,
        stop_on_output: bool,
    ) -> ExecutionState {
        let relative_base_offset = self.relative_base_offset;

        let (new_ip, new_state) = match instruction {
            Instruction::Op(op, arg1, arg2, arg3) => {
                let a1 = arg1.value(&self.program, relative_base_offset);
                let a2 = arg2.value(&self.program, relative_base_offset);
                let a3 = arg3.address(relative_base_offset);

                match op {
                    Op::Add => self.program[a3] = a1 + a2,
                    Op::Multiply => self.program[a3] = a1 * a2,
                }

                (None, None)
            }
            Instruction::ConditionalJump(condition, arg1, arg2) => {
                let a1 = arg1.value(&self.program, relative_base_offset);
                let a2 = arg2.value(&self.program, relative_base_offset);

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
            Instruction::Compare(op, arg1, arg2, arg3) => {
                let a1 = arg1.value(&self.program, relative_base_offset);
                let a2 = arg2.value(&self.program, relative_base_offset);
                let a3 = arg3.address(relative_base_offset);

                match op {
                    ComparisonOp::LessThan => self.program[a3] = if a1 < a2 { 1 } else { 0 },
                    ComparisonOp::Equal => self.program[a3] = if a1 == a2 { 1 } else { 0 },
                }

                (None, None)
            }
            Instruction::Input(arg1) => {
                let next_input = self.read_input();

                match next_input {
                    None => {
                        // We need to wait
                        (None, Some(ExecutionState::Pause))
                    }
                    Some(input) => {
                        let a1 = arg1.address(relative_base_offset);
                        self.program[a1 as usize] = input;

                        (None, None)
                    }
                }
            }
            Instruction::Output(arg1) => {
                let a1 = arg1.value(&self.program, relative_base_offset);

                self.outputs.push(a1);

                if stop_on_output {
                    (None, Some(ExecutionState::Pause))
                } else {
                    (None, None)
                }
            }
            Instruction::RelativeBaseAdjust(arg1) => {
                let a1 = arg1.value(&self.program, relative_base_offset);
                self.relative_base_offset = ((self.relative_base_offset as isize) + a1) as usize;

                (None, None)
            }
            Instruction::Halt => (Some(self.ip), Some(ExecutionState::Halt)),
        };

        self.ip = new_ip.unwrap_or_else(|| self.ip + instruction.length());

        new_state.unwrap_or(ExecutionState::Continue)
    }
}
