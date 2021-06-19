#[derive(Debug, Eq, PartialEq)]
pub enum OpCode {
    Sum = 1,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

impl From<i64> for OpCode {
    fn from(value: i64) -> OpCode {
        match value {
            1 => Self::Sum,
            2 => Self::Mul,
            3 => Self::Input,
            4 => Self::Output,
            5 => Self::JumpIfTrue,
            6 => Self::JumpIfFalse,
            7 => Self::LessThan,
            8 => Self::Equals,
            99 => Self::Halt,
            _ => panic!("unknown value {}", value),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParameterMode {
    Position,
    Immediate,
}

impl From<i64> for ParameterMode {
    fn from(n: i64) -> Self {
        match n {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("invalid parameter mode"),
        }
    }
}

pub struct IntCode {
    program: Vec<i64>,
    idx: usize,
    register: i64,
    halt: bool,
}

pub fn new(program: Vec<i64>) -> IntCode {
    IntCode {
        program,
        idx: 0,
        register: 0,
        halt: false,
    }
}

fn decode_instruction(instruction: i64) -> (OpCode, Vec<ParameterMode>) {
    let opcode = instruction % 100;
    let param_modes = vec![
        ((instruction / 100) % 10).into(),
        ((instruction / 1000) % 10).into(),
        ((instruction / 10000) % 10).into(),
    ];

    (opcode.into(), param_modes)
}

impl IntCode {
    pub fn do_step(&mut self) {
        let cmd = self.program[self.idx];
        dbg!(cmd);
        let (op_code, params) = decode_instruction(cmd);
        match op_code {
            OpCode::Sum => self.sum(&params),
            OpCode::Mul => self.mul(&params),
            OpCode::Input => self.input(&params),
            OpCode::Output => self.output(&params),
            OpCode::JumpIfTrue => self.jump_if_true(&params),
            OpCode::JumpIfFalse => self.jump_if_false(&params),
            OpCode::LessThan => self.less_than(&params),
            OpCode::Equals => self.equals(&params),
            OpCode::Halt => self.halt(),
        }
    }

    pub fn run(&mut self) {
        while !self.halt {
            self.do_step();
        }
    }

    pub fn set_input(&mut self, value: i64) {
        self.register = value;
    }

    pub fn get_output(&self) -> i64 {
        self.register
    }

    pub fn first(&self) -> i64 {
        self.program[0]
    }

    fn read_value(&self, offset: usize, parameter_modes: &[ParameterMode]) -> i64 {
        if self.idx > self.program.len() {
            return 0;
        }

        let n = self.program[self.idx + offset + 1];
        match parameter_modes[offset] {
            ParameterMode::Position => self.program[n as usize],
            ParameterMode::Immediate => n,
        }
    }

    fn read_dest(&self, offset: usize, parameter_modes: &[ParameterMode]) -> i64 {
        if self.idx > self.program.len() {
            return 0;
        }

        let n = self.program[self.idx + offset + 1];
        match parameter_modes[offset] {
            ParameterMode::Position => n,
            ParameterMode::Immediate => panic!("invalid mode for dest"),
        }
    }

    fn operands3(&self, parameter_modes: &[ParameterMode]) -> (i64, i64, i64) {
        let param1 = self.read_value(0, parameter_modes);
        let param2 = self.read_value(1, parameter_modes);
        let dest = self.read_dest(2, parameter_modes);
        (param1, param2, dest)
    }

    fn operands2(&self, parameter_modes: &[ParameterMode]) -> (i64, i64) {
        let param1 = self.read_value(0, parameter_modes);
        let param2 = self.read_value(1, parameter_modes);
        (param1, param2)
    }

    fn halt(&mut self) {
        self.halt = true;
    }

    fn sum(&mut self, parameter_modes: &[ParameterMode]) {
        let (n1, n2, dest) = self.operands3(parameter_modes);
        self.program[dest as usize] = n1 + n2;
        self.idx += 4;
    }

    fn mul(&mut self, parameter_modes: &[ParameterMode]) {
        let (n1, n2, dest) = self.operands3(parameter_modes);
        self.program[dest as usize] = n1 * n2;
        self.idx += 4;
    }

    fn input(&mut self, parameter_modes: &[ParameterMode]) {
        let addr = self.read_dest(0, parameter_modes);
        self.program[addr as usize] = self.register;
        self.idx += 2;
    }

    fn output(&mut self, parameter_modes: &[ParameterMode]) {
        let n = self.read_value(0, parameter_modes);
        self.register = n;
        self.idx += 2;
    }

    fn jump_if_true(&mut self, parameter_modes: &[ParameterMode]) {
        let (n1, n2) = self.operands2(parameter_modes);
        self.idx = if n1 != 0 { n2 as usize } else { self.idx + 3 };
    }

    fn jump_if_false(&mut self, parameter_modes: &[ParameterMode]) {
        let (n1, n2) = self.operands2(parameter_modes);
        self.idx = if n1 == 0 { n2 as usize } else { self.idx + 3 };
    }

    fn less_than(&mut self, parameter_modes: &[ParameterMode]) {
        let (n1, n2, dest) = self.operands3(parameter_modes);
        self.program[dest as usize] = if n1 < n2 { 1 } else { 0 };
        self.idx += 4;
    }

    fn equals(&mut self, parameter_modes: &[ParameterMode]) {
        let (n1, n2, dest) = self.operands3(parameter_modes);
        self.program[dest as usize] = if n1 == n2 { 1 } else { 0 };
        self.idx += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation() {
        assert_eq!(
            decode_instruction(1002),
            (
                OpCode::Mul,
                vec![
                    ParameterMode::Position,
                    ParameterMode::Immediate,
                    ParameterMode::Position,
                ]
            )
        );

        assert_eq!(
            decode_instruction(1),
            (
                OpCode::Sum,
                vec![
                    ParameterMode::Position,
                    ParameterMode::Position,
                    ParameterMode::Position,
                ]
            )
        );

        assert_eq!(
            decode_instruction(104),
            (
                OpCode::Output,
                vec![
                    ParameterMode::Immediate,
                    ParameterMode::Position,
                    ParameterMode::Position,
                ]
            )
        );
    }
}
