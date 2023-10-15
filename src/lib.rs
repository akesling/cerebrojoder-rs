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

pub trait Compiler<SourceInstructionType, TargetInstructionType> {
    fn compile(module: &Module<SourceInstructionType>) -> Module<TargetInstructionType>;
}

pub trait Executor<TargetInstructionType> {
    fn execute(module: &Module<TargetInstructionType>) -> std::io::Result<()>;
}

#[inline]
fn read_char() -> u8 {
    std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as u8)
        .unwrap()
}

#[inline]
fn take_one(buffer: &str) -> (&str, &str) {
    (&buffer[0..1], &buffer[1..])
}

#[inline]
fn take_all(buffer: &str, character: char) -> (&str, &str) {
    let to_compare = character.to_string();
    let mut partition = 1;
    while buffer[partition - 1..partition] == to_compare && partition < std::u8::MAX as usize {
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
                    '<' | '>' | '+' | '-' | '.' | ',' => {
                        let (_head, _tail) = take_all(tail, code_char);
                        tail = _tail;

                        code_segment[code_counter] =
                            Instruction::get_instruction(code_char, _head.len() as u8);
                        code_counter = code_counter + 1;
                    }
                    // TODO(Alex): Investigate simple loop unrolling.
                    '[' => {
                        stack_lookup[stack_index] = code_counter;
                        stack_index = stack_index + 1;

                        let (_head, _tail) = take_one(tail);
                        tail = _tail;

                        code_segment[code_counter] = Instruction::get_instruction(code_char, 1);
                        code_counter = code_counter + 1;
                    }
                    ']' => {
                        stack_index = stack_index - 1;

                        jump_lookup[stack_lookup[stack_index]] = code_counter;
                        jump_lookup[code_counter] = stack_lookup[stack_index];

                        let (_head, _tail) = take_one(tail);
                        tail = _tail;

                        code_segment[code_counter] = Instruction::get_instruction(code_char, 1);
                        code_counter = code_counter + 1;
                    }
                    _ => {
                        let (_head, _tail) = take_one(tail);
                        tail = _tail;
                    }
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

impl Executor<Instruction> for BfInterpreter {
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
                Instruction::Subtract(v) => {
                    data_segment[data_ptr] = data_segment[data_ptr] - v as i8
                }
                Instruction::Land => {
                    if data_segment[data_ptr] == 0 {
                        code_ptr = module.jump_lookup[code_ptr];
                    }
                }
                Instruction::Jump => {
                    if data_segment[data_ptr] != 0 {
                        code_ptr = module.jump_lookup[code_ptr];
                    }
                }
                Instruction::Write => {
                    print!("{}", data_segment[data_ptr] as u8 as char);
                }
                Instruction::Read => data_segment[data_ptr] = read_char() as i8,
                Instruction::Nop => (),
            }
            code_ptr = code_ptr + 1;
        }

        Ok(())
    }
}

// pub struct x86Jit;
// type X86 = usize;

// impl Compiler<Instruction, X86> for x86Jit {
//     fn compile(_module: &Module<Instruction>) -> Module<X86> {
//         todo!("WasmJit compiler not yet implemented.");

//         Module {
//             code_segment: [0; HEAPSIZE],
//             code_length: 0,
//             jump_lookup: [0; HEAPSIZE],
//         }
//     }
// }

// impl Executor<X86> for x86Jit {
//     fn execute(module: &Module<X86>) -> std::io::Result<()> {
//         panic!("Not yet implemented")
//         // // 8bit signed rollover is necessary for the mandelbrot implementation, so we use i8 for the
//         // // data segment.
//         // let mut data_segment: [i8; HEAPSIZE] = [0; HEAPSIZE];

//         // // Execute
//         // let mut code_ptr: usize = 0;
//         // let mut data_ptr: usize = 0;
//         // while code_ptr < module.code_length {
//         //     match module.code_segment[code_ptr] {
//         //         Instruction::Backward(v) => data_ptr = data_ptr - v as usize,
//         //         Instruction::Forward(v) => data_ptr = data_ptr + v as usize,
//         //         Instruction::Add(v) => data_segment[data_ptr] = data_segment[data_ptr] + v as i8,
//         //         Instruction::Subtract(v) => {
//         //             data_segment[data_ptr] = data_segment[data_ptr] - v as i8
//         //         }
//         //         Instruction::Land => {
//         //             if data_segment[data_ptr] == 0 {
//         //                 code_ptr = module.jump_lookup[code_ptr];
//         //             }
//         //         }
//         //         Instruction::Jump => {
//         //             if data_segment[data_ptr] != 0 {
//         //                 code_ptr = module.jump_lookup[code_ptr];
//         //             }
//         //         }
//         //         Instruction::Write => {
//         //             print!("{}", data_segment[data_ptr] as u8 as char);
//         //         }
//         //         Instruction::Read => data_segment[data_ptr] = read_char() as i8,
//         //         Instruction::Nop => (),
//         //     }
//         //     code_ptr = code_ptr + 1;
//         // }

//         // Ok(())
//     }
// }

pub struct WasmJit;

impl WasmJit {
    fn translate(
        &mut self,
        source: &[Instruction],
        jump_lookup: &[usize],
        jump_offset: usize,
        module: &walrus::Module,
        data_ptr: walrus::LocalId,
        data_segment: walrus::MemoryId,
        get_char: walrus::FunctionId,
        put_char: walrus::FunctionId,
    ) -> Vec<walrus::ir::Instr> {
        use walrus::ir;

        let mut output: Vec<ir::Instr> = Vec::with_capacity(source.len());
        let mut cursor = 0;
        while cursor < source.len() {
            let mut translation: Vec<ir::Instr> = match source[cursor] {
                // Instruction::Backward(v) => data_ptr = data_ptr - v as usize,
                Instruction::Backward(v) => {
                    vec![
                        (ir::LocalGet { local: data_ptr }).into(),
                        (ir::Const {
                            value: ir::Value::I32(i32::from(v)),
                        })
                        .into(),
                        (ir::Binop {
                            op: ir::BinaryOp::I32Sub,
                        })
                        .into(),
                        (ir::LocalSet { local: data_ptr }).into(),
                    ]
                }
                // Instruction::Forward(v) => data_ptr = data_ptr + v as usize,
                Instruction::Forward(v) => {
                    vec![
                        (ir::LocalGet { local: data_ptr }).into(),
                        (ir::Const {
                            value: ir::Value::I32(i32::from(v)),
                        })
                        .into(),
                        (ir::Binop {
                            op: ir::BinaryOp::I32Add,
                        })
                        .into(),
                        (ir::LocalSet { local: data_ptr }).into(),
                    ]
                }
                // Instruction::Add(v) => data_segment[data_ptr] = data_segment[data_ptr] + v as i8,
                Instruction::Add(v) => {
                    vec![
                        (ir::LocalGet { local: data_ptr }).into(),
                        (ir::Load {
                            memory: data_segment,
                            kind: ir::LoadKind::I32_8 {
                                kind: ir::ExtendedLoad::ZeroExtend,
                            },
                            arg: ir::MemArg {
                                align: 1,
                                offset: 0,
                            },
                        })
                        .into(),
                        (ir::Const {
                            value: ir::Value::I32(i32::from(v)),
                        })
                        .into(),
                        (ir::Binop {
                            op: ir::BinaryOp::I32Add,
                        })
                        .into(),
                        (ir::Store {
                            memory: data_segment,
                            kind: ir::StoreKind::I32_8 { atomic: false },
                            arg: ir::MemArg {
                                align: 1,
                                offset: 0,
                            },
                        })
                        .into(),
                    ]
                }
                // Instruction::Subtract(v) => {
                //     data_segment[data_ptr] = data_segment[data_ptr] - v as i8
                // }
                Instruction::Subtract(v) => {
                    vec![
                        (ir::LocalGet { local: data_ptr }).into(),
                        (ir::Load {
                            memory: data_segment,
                            kind: ir::LoadKind::I32_8 {
                                kind: ir::ExtendedLoad::ZeroExtend,
                            },
                            arg: ir::MemArg {
                                align: 1,
                                offset: 0,
                            },
                        })
                        .into(),
                        (ir::Const {
                            value: ir::Value::I32(i32::from(v)),
                        })
                        .into(),
                        (ir::Binop {
                            op: ir::BinaryOp::I32Sub,
                        })
                        .into(),
                        (ir::Store {
                            memory: data_segment,
                            kind: ir::StoreKind::I32_8 { atomic: false },
                            arg: ir::MemArg {
                                align: 1,
                                offset: 0,
                            },
                        })
                        .into(),
                    ]
                }
                // Instruction::Land => {
                //     if data_segment[data_ptr] == 0 {
                //         code_ptr = module.jump_lookup[code_ptr];
                //     }
                // }
                Instruction::Land => {
                    let inner_source = &source[cursor + 1..jump_lookup[cursor - jump_offset]];
                    let inner_wasm = self.translate(
                        inner_source,
                        jump_lookup,
                        jump_offset + cursor,
                        module,
                        data_ptr,
                        data_segment,
                        get_char,
                        put_char,
                    );
                    cursor += inner_source.len();
                    vec![
                        // Block,
                        // Loop,
                        // Check-skip (block)
                        // ... code ...
                        // Check-repeat (loop)
                        // End Loop,
                        // End Block,
                    ]
                }
                // Instruction::Jump => {
                //     if data_segment[data_ptr] != 0 {
                //         code_ptr = module.jump_lookup[code_ptr];
                //     }
                // }
                Instruction::Jump => vec![],
                // Instruction::Write => {
                //     print!("{}", data_segment[data_ptr] as u8 as char);
                // }
                Instruction::Write => {
                    vec![
                        (ir::LocalGet { local: data_ptr }).into(),
                        (ir::Load {
                            memory: data_segment,
                            kind: ir::LoadKind::I32_8 {
                                kind: ir::ExtendedLoad::ZeroExtend,
                            },
                            arg: ir::MemArg {
                                align: 1,
                                offset: 0,
                            },
                        })
                        .into(),
                        (ir::Call { func: todo!() }).into(),
                    ]
                }
                // Instruction::Read => data_segment[data_ptr] = read_char() as i8,
                Instruction::Read => todo!(),
                // Instruction::Nop => (),
                Instruction::Nop => vec![],
            };

            cursor += 1;
            output.append(&mut translation);
        }
        output
    }
}

// Not yet implemented;
type WASM = u8;

impl Compiler<Instruction, WASM> for WasmJit {
    fn compile(module: &Module<Instruction>) -> Module<WASM> {
        use walrus::ir;
        println!("WasmJit compiler not yet implemented.");

        let mut wasm_module = walrus::Module::default();
        // TODO(alex): Add import for getting a character
        // TODO(alex): Add import for printing a character

        let put_char_type = wasm_module.types.add(&[], &[]);
        let get_char_type = wasm_module.types.add(&[], &[]);
        // let put_char_import = wasm_module.imports.add("brainfuck", "output_byte",

        let data_segment = wasm_module
            .memories
            .add_local(false, 0, Some(HEAPSIZE as u32));
        let data_ptr = wasm_module.locals.add(walrus::ValType::I32);
        let wasm_instructions: Vec<ir::Instr> = module
            .code_segment
            .iter()
            .enumerate()
            .flat_map(|(code_ptr, inst)| {
                match inst {
                    // Instruction::Backward(v) => data_ptr = data_ptr - v as usize,
                    Instruction::Backward(v) => {
                        vec![
                            (ir::LocalGet { local: data_ptr }).into(),
                            (ir::Const {
                                value: ir::Value::I32(i32::from(*v)),
                            })
                            .into(),
                            (ir::Binop {
                                op: ir::BinaryOp::I32Sub,
                            })
                            .into(),
                            (ir::LocalSet { local: data_ptr }).into(),
                        ]
                    }
                    // Instruction::Forward(v) => data_ptr = data_ptr + v as usize,
                    Instruction::Forward(v) => {
                        vec![
                            (ir::LocalGet { local: data_ptr }).into(),
                            (ir::Const {
                                value: ir::Value::I32(i32::from(*v)),
                            })
                            .into(),
                            (ir::Binop {
                                op: ir::BinaryOp::I32Add,
                            })
                            .into(),
                            (ir::LocalSet { local: data_ptr }).into(),
                        ]
                    }
                    // Instruction::Add(v) => data_segment[data_ptr] = data_segment[data_ptr] + v as i8,
                    Instruction::Add(v) => {
                        vec![
                            (ir::LocalGet { local: data_ptr }).into(),
                            (ir::Load {
                                memory: data_segment,
                                kind: ir::LoadKind::I32_8 {
                                    kind: ir::ExtendedLoad::ZeroExtend,
                                },
                                arg: ir::MemArg {
                                    align: 1,
                                    offset: 0,
                                },
                            })
                            .into(),
                            (ir::Const {
                                value: ir::Value::I32(i32::from(*v)),
                            })
                            .into(),
                            (ir::Binop {
                                op: ir::BinaryOp::I32Add,
                            })
                            .into(),
                            (ir::Store {
                                memory: data_segment,
                                kind: ir::StoreKind::I32_8 { atomic: false },
                                arg: ir::MemArg {
                                    align: 1,
                                    offset: 0,
                                },
                            })
                            .into(),
                        ]
                    }
                    // Instruction::Subtract(v) => {
                    //     data_segment[data_ptr] = data_segment[data_ptr] - v as i8
                    // }
                    Instruction::Subtract(v) => {
                        vec![
                            (ir::LocalGet { local: data_ptr }).into(),
                            (ir::Load {
                                memory: data_segment,
                                kind: ir::LoadKind::I32_8 {
                                    kind: ir::ExtendedLoad::ZeroExtend,
                                },
                                arg: ir::MemArg {
                                    align: 1,
                                    offset: 0,
                                },
                            })
                            .into(),
                            (ir::Const {
                                value: ir::Value::I32(i32::from(*v)),
                            })
                            .into(),
                            (ir::Binop {
                                op: ir::BinaryOp::I32Sub,
                            })
                            .into(),
                            (ir::Store {
                                memory: data_segment,
                                kind: ir::StoreKind::I32_8 { atomic: false },
                                arg: ir::MemArg {
                                    align: 1,
                                    offset: 0,
                                },
                            })
                            .into(),
                        ]
                    }
                    // Instruction::Land => {
                    //     if data_segment[data_ptr] == 0 {
                    //         code_ptr = module.jump_lookup[code_ptr];
                    //     }
                    // }
                    Instruction::Land => {
                        vec![
                            // Block,
                            // Loop,
                            // Check-skip (block)
                        ]
                    }
                    // Instruction::Jump => {
                    //     if data_segment[data_ptr] != 0 {
                    //         code_ptr = module.jump_lookup[code_ptr];
                    //     }
                    // }
                    Instruction::Jump => {
                        vec![
                            // Check-repeat (loop)
                            // End Loop,
                            // End Block,
                        ]
                    }
                    // Instruction::Write => {
                    //     print!("{}", data_segment[data_ptr] as u8 as char);
                    // }
                    Instruction::Write => {
                        vec![
                            (ir::LocalGet { local: data_ptr }).into(),
                            (ir::Load {
                                memory: data_segment,
                                kind: ir::LoadKind::I32_8 {
                                    kind: ir::ExtendedLoad::ZeroExtend,
                                },
                                arg: ir::MemArg {
                                    align: 1,
                                    offset: 0,
                                },
                            })
                            .into(),
                            (ir::Call { func: todo!() }).into(),
                        ]
                    }
                    // Instruction::Read => data_segment[data_ptr] = read_char() as i8,
                    Instruction::Read => todo!(),
                    // Instruction::Nop => (),
                    Instruction::Nop => vec![],
                }
            })
            .collect();

        Module {
            code_segment: todo!(),
            code_length: 0,
            jump_lookup: [0; HEAPSIZE],
        }
    }
}

impl Executor<WASM> for WasmJit {
    fn execute(_module: &Module<WASM>) -> std::io::Result<()> {
        println!("WasmJit executor not yet implemented.");

        Ok(())
    }
}
