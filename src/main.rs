use std::io::Read;

use ::cerebrojoder_rs::{Instruction, Module, HEAPSIZE, read_char, BfInterpreter, Parser};

fn execute(module: &Module<Instruction>) -> std::io::Result<()> {
    // 8bit signed rollover is necessary for the mandelbrot implementation, so we use i8 for the
    // data segment.
    let mut data_segment: [i8; HEAPSIZE] = [0; HEAPSIZE];

    // Execute
    let mut code_ptr: usize = 0;
    let mut data_ptr: usize = 0;
    while code_ptr < module.code_length {
        match module.code_segment[code_ptr] {
            Instruction::Backward(v) => data_ptr = data_ptr - v as usize,
            Instruction::Forward(v) => data_ptr = data_ptr + v as usize,
            Instruction::Add(v) => data_segment[data_ptr] = data_segment[data_ptr] + v as i8,
            Instruction::Subtract(v) => data_segment[data_ptr] = data_segment[data_ptr] - v as i8,
            Instruction::Land => {
                if data_segment[data_ptr] == 0 {
                    code_ptr = module.jump_lookup[code_ptr];
                }
            },
            Instruction::Jump => {
                if data_segment[data_ptr] != 0 {
                    code_ptr = module.jump_lookup[code_ptr];
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

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    let module = BfInterpreter::parse(&buffer);

    execute(&module)
}
