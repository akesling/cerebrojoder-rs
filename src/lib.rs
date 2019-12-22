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
