use std::io::Read;

pub const HEAPSIZE: usize = 30000;
pub const CODESIZE: usize = HEAPSIZE;
pub const STACKSIZE: usize = 30000;

#[derive(Copy, Clone)]
pub enum Instruction {
    Backward(u8),
    Forward(u8),
    Subtract(u8),
    Add(u8),
    Land,
    Jump,
    Write,
    Read,
    Nop,
}

impl Instruction {
    #[inline]
    pub fn get_instruction(character: char, value: u8) -> Instruction {
        match character {
            '<' => Instruction::Backward(value),
            '>' => Instruction::Forward(value),
            '-' => Instruction::Subtract(value),
            '+' => Instruction::Add(value),
            '[' => Instruction::Land,
            ']' => Instruction::Jump,
            '.' => Instruction::Write,
            ',' => Instruction::Read,
            _ => Instruction::Nop,
        }
    }
}

pub struct Module<InstructionType> {
    pub code_segment: [InstructionType; CODESIZE],
    pub code_length: usize,
    pub jump_lookup: [usize; HEAPSIZE],
}

pub trait Parser<TargetInstructionType> {
    fn parse(buffer: &str) -> Module<TargetInstructionType>;
}

#[inline]
pub fn read_char() -> u8 {
    std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as u8).unwrap()
}

#[inline]
fn take_one(buffer: &str) -> (&str, &str) {
    (&buffer[0..1], &buffer[1..])
}

#[inline]
fn take_all(buffer: &str, character: char) -> (&str, &str) {
    let to_compare = character.to_string();
    let mut partition = 1;
    while buffer[partition-1..partition] == to_compare && partition < std::u8::MAX as usize {
        partition = partition + 1;
    }
    partition = partition - 1;
    (&buffer[0..partition], &buffer[partition..])
}


pub struct BfInterpreter;
impl Parser<Instruction> for BfInterpreter {
    fn parse(buffer: &str) -> Module<Instruction> {
        let mut code_segment: [Instruction; HEAPSIZE] = [Instruction::Nop; HEAPSIZE];
        let mut jump_lookup: [usize; HEAPSIZE] = [0; HEAPSIZE];
        let mut stack_lookup: [usize; STACKSIZE] = [0; STACKSIZE];

        let code_length;
        {
            // Precompute jumps and remove comments
            let mut stack_index: usize = 0;
            let mut code_counter = 0;

            // TODO(Alex): Is there a better way to do this?  Maybe a stream of some form?
            let mut tail: &str = buffer;
            while !tail.is_empty() {
                let code_char = tail.chars().next().unwrap();
                match code_char {
                    '<' | '>'| '+'| '-'| '.'| ',' => {
                        let (_head, _tail) = take_all(tail, code_char);
                        tail = _tail;

                        code_segment[code_counter] = Instruction::get_instruction(code_char, _head.len() as u8);
                        code_counter = code_counter + 1;
                    },
                    // TODO(Alex): Investigate simple loop unrolling.
                    '[' => {
                        stack_lookup[stack_index] = code_counter;
                        stack_index = stack_index + 1;

                        let (_head, _tail) = take_one(tail);
                        tail = _tail;

                        code_segment[code_counter] = Instruction::get_instruction(code_char, 1);
                        code_counter = code_counter + 1;
                    },
                    ']' => {
                        stack_index = stack_index - 1;

                        jump_lookup[stack_lookup[stack_index]] = code_counter;
                        jump_lookup[code_counter] = stack_lookup[stack_index];

                        let (_head, _tail) = take_one(tail);
                        tail = _tail;

                        code_segment[code_counter] = Instruction::get_instruction(code_char, 1);
                        code_counter = code_counter + 1;
                    },
                    _ => {
                        let (_head, _tail) = take_one(tail);
                        tail = _tail;
                    },
                }
            }
            code_length = code_counter + 1;
        }

        Module {
            code_segment,
            code_length,
            jump_lookup,
        }
    }
}

