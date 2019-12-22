#[cfg(feature = "interpreter")]
use std::io::Read;

#[cfg(feature = "interpreter")]
use ::cerebrojoder_rs::{BfInterpreter, Executor, Parser};

#[cfg(feature = "interpreter")]
fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    let module = BfInterpreter::parse(&buffer);

    BfInterpreter::execute(&module)
}

#[cfg(feature = "wasmer-backend")]
fn main() -> std::io::Result<()> {
    println!("Not yet implemented!");

    Ok(())
}
