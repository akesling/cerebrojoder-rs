use std::io::{self, Read};

const HEAPSIZE: usize = 30000;

static mut DATA: [u8; HEAPSIZE] = [0; HEAPSIZE];
static mut CODE: [u8; HEAPSIZE] = [0; HEAPSIZE];
static mut JUMPS: [usize; HEAPSIZE] = [0; HEAPSIZE];

fn read_char() -> u8 {
    std::io::stdin()
        .bytes() 
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as u8).unwrap()
}

fn main() -> io::Result<()> {
    // TODO(Alex): Make this all safe.
    unsafe {
        io::stdin().read(&mut CODE)?;


        // TODO(Alex): Remove the necessity to count this manually.
        let mut code_length = 0;

        // Precompute jumps
        let mut stack_index: usize = 0;
        for (code_counter, code_char) in CODE.iter().enumerate() {
            match code_char.clone() as char {
                '<' | '>'| '+'| '-'| '.'| ',' => (),
                '[' => {
                    stack_index = code_counter;
                    stack_index = stack_index + 1;
                },
                ']' => {
                    stack_index = stack_index - 1;
                    JUMPS[stack_index] = code_counter;
                    JUMPS[code_counter] = stack_index;
                },
                _ => (),
            }
            code_length = code_counter + 1;
        }

        // Execute
        let mut code_ptr = 0;
        let mut data_ptr = 0;
        while code_ptr < code_length {
            match CODE[code_ptr].clone() as char {
                '<' => data_ptr = data_ptr - 1,
                '>' => data_ptr = data_ptr + 1,
                '+' => DATA[data_ptr] = DATA[data_ptr] + 1,
                '-' => DATA[data_ptr] = DATA[data_ptr] - 1,
                '[' => {
                    if DATA[data_ptr] != 0 {
                        code_ptr = JUMPS[code_ptr];
                    }
                },
                ']' => {
                    if DATA[data_ptr] == 0 {
                        code_ptr = JUMPS[code_ptr];
                    }
                },
                '.' => print!("{}", DATA[data_ptr] as char),
                ',' => DATA[data_ptr] = read_char(),
                _ => (),
            }
            code_ptr = code_ptr + 1;
        }
    }
    Ok(())
}
