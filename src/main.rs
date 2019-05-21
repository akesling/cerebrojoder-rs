use std::io::{self, Read};

const HEAPSIZE: usize = 30000;
const STACKSIZE: usize = 30000;

#[derive(Copy, Clone)]
enum Instruction {
    Backward,
    Forward,
    Subtract,
    Add,
    Land,
    Jump,
    Write,
    Read,
    Nop,
}

impl Instruction {
    fn get_instruction(character: char) -> Instruction {
        match character {
            '<' => Instruction::Backward,
            '>' => Instruction::Forward,
            '-' => Instruction::Subtract,
            '+' => Instruction::Add,
            '[' => Instruction::Land,
            ']' => Instruction::Jump,
            '.' => Instruction::Write,
            ',' => Instruction::Read,
            _ => Instruction::Nop,
        }
    }
}

fn read_char() -> u8 {
    std::io::stdin()
        .bytes() 
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as u8).unwrap()
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut data_segment: [i8; HEAPSIZE] = [0; HEAPSIZE];
    let mut code_segment: [Instruction; HEAPSIZE] = [Instruction::Nop; HEAPSIZE];
    let mut jump_lookup: [usize; HEAPSIZE] = [0; HEAPSIZE];
    let mut stack_lookup: [usize; STACKSIZE] = [0; STACKSIZE];

    let code_length;
    {
        // Precompute jumps and remove comments
        let mut stack_index: usize = 0;
        let mut code_counter = 0;
        for code_char in buffer.chars() {
            match code_char {
                '<' | '>'| '+'| '-'| '.'| ',' => {
                    code_segment[code_counter] = Instruction::get_instruction(code_char);
                    code_counter = code_counter + 1;
                },
                '[' => {
                    stack_lookup[stack_index] = code_counter;
                    stack_index = stack_index + 1;

                    code_segment[code_counter] = Instruction::get_instruction(code_char);
                    code_counter = code_counter + 1;
                },
                ']' => {
                    stack_index = stack_index - 1;

                    jump_lookup[stack_lookup[stack_index]] = code_counter;
                    jump_lookup[code_counter] = stack_lookup[stack_index];

                    code_segment[code_counter] = Instruction::get_instruction(code_char);
                    code_counter = code_counter + 1;
                },
                _ => (),
            }
        }
        code_length = code_counter + 1;
    }

    // Execute
    let mut code_ptr = 0;
    let mut data_ptr = 0;
    while code_ptr < code_length {
        match code_segment[code_ptr] {
            Instruction::Backward => data_ptr = data_ptr - 1,
            Instruction::Forward => data_ptr = data_ptr + 1,
            Instruction::Add => data_segment[data_ptr] = data_segment[data_ptr] + 1,
            Instruction::Subtract => data_segment[data_ptr] = data_segment[data_ptr] - 1,
            Instruction::Land => {
                if data_segment[data_ptr] == 0 {
                    code_ptr = jump_lookup[code_ptr];
                }
            },
            Instruction::Jump => {
                if data_segment[data_ptr] != 0 {
                    code_ptr = jump_lookup[code_ptr];
                }
            },
            Instruction::Write => {
                print!("{}", data_segment[data_ptr] as u8 as char);
            },
            Instruction::Read => data_segment[data_ptr] = read_char() as i8,
            Instruction::Nop => (),
        }
        code_ptr = code_ptr + 1;
    }

    Ok(())
}
