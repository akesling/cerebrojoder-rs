// These "unused" imports are used in all conditional compilation targets.
#[allow(unused_imports)]
use std::io::Read;

#[allow(unused_imports)]
use ::cerebrojoder_rs::{BfInterpreter, Executor, Parser};

#[cfg(feature = "interpreter")]
fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    let module = BfInterpreter::parse(&buffer);

    BfInterpreter::execute(&module)
}

#[cfg(feature = "wasmer-backend")]
use ::cerebrojoder_rs::{Compiler, WasmJit};

#[cfg(feature = "wasmer-backend")]
fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    let ir_module = BfInterpreter::parse(&buffer);
    let wasm_module = WasmJit::compile(&ir_module);

    WasmJit::execute(&wasm_module)
}
