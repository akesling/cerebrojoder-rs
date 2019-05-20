use std::io::{self, Read};

const HEAPSIZE: usize = 30000;
const STACKSIZE: usize = 30000;

static mut DATA: [i8; HEAPSIZE] = [0; HEAPSIZE];
static mut CODE: [u8; HEAPSIZE] = [0; HEAPSIZE];
static mut JUMPS: [usize; HEAPSIZE] = [0; HEAPSIZE];
static mut STACK: [usize; STACKSIZE] = [0; STACKSIZE];

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
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;

        let code_length;
        {
            // Precompute jumps and remove comments
            let mut stack_index: usize = 0;
            let mut code_counter = 0;
            for code_char in buffer.chars() {
                match code_char {
                    '<' | '>'| '+'| '-'| '.'| ',' => {
                        CODE[code_counter] = code_char as u8;
                        code_counter = code_counter + 1;
                    },
                    '[' => {
                        STACK[stack_index] = code_counter;
                        stack_index = stack_index + 1;

                        CODE[code_counter] = code_char as u8;
                        code_counter = code_counter + 1;
                    },
                    ']' => {
                        stack_index = stack_index - 1;

                        JUMPS[STACK[stack_index]] = code_counter;
                        JUMPS[code_counter] = STACK[stack_index];

                        CODE[code_counter] = code_char as u8;
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
            match CODE[code_ptr].clone() as char {
                '<' => data_ptr = data_ptr - 1,
                '>' => data_ptr = data_ptr + 1,
                '+' => DATA[data_ptr] = DATA[data_ptr] + 1,
                '-' => DATA[data_ptr] = DATA[data_ptr] - 1,
                '[' => {
                    if DATA[data_ptr] == 0 {
                        code_ptr = JUMPS[code_ptr];
                    }
                },
                ']' => {
                    if DATA[data_ptr] != 0 {
                        code_ptr = JUMPS[code_ptr];
                    }
                },
                '.' => {
                    print!("{}", DATA[data_ptr] as u8 as char);
                },
                ',' => DATA[data_ptr] = read_char() as i8,
                _ => (),
            }
            code_ptr = code_ptr + 1;
        }
    }

    Ok(())
}
