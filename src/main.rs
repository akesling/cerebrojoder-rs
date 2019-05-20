use std::io::{self, Read};

const HEAPSIZE: usize = 30000;
const STACKSIZE: usize = 30000;

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
    let mut code_segment: [u8; HEAPSIZE] = [0; HEAPSIZE];
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
                    code_segment[code_counter] = code_char as u8;
                    code_counter = code_counter + 1;
                },
                '[' => {
                    stack_lookup[stack_index] = code_counter;
                    stack_index = stack_index + 1;

                    code_segment[code_counter] = code_char as u8;
                    code_counter = code_counter + 1;
                },
                ']' => {
                    stack_index = stack_index - 1;

                    jump_lookup[stack_lookup[stack_index]] = code_counter;
                    jump_lookup[code_counter] = stack_lookup[stack_index];

                    code_segment[code_counter] = code_char as u8;
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
        match code_segment[code_ptr].clone() as char {
            '<' => data_ptr = data_ptr - 1,
            '>' => data_ptr = data_ptr + 1,
            '+' => data_segment[data_ptr] = data_segment[data_ptr] + 1,
            '-' => data_segment[data_ptr] = data_segment[data_ptr] - 1,
            '[' => {
                if data_segment[data_ptr] == 0 {
                    code_ptr = jump_lookup[code_ptr];
                }
            },
            ']' => {
                if data_segment[data_ptr] != 0 {
                    code_ptr = jump_lookup[code_ptr];
                }
            },
            '.' => {
                print!("{}", data_segment[data_ptr] as u8 as char);
            },
            ',' => data_segment[data_ptr] = read_char() as i8,
            _ => (),
        }
        code_ptr = code_ptr + 1;
    }

    Ok(())
}
