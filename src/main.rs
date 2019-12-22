use std::io::Read;

use ::cerebrojoder_rs::{BfInterpreter, Executor, Parser};

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    let module = BfInterpreter::parse(&buffer);

    BfInterpreter::execute(&module)
}
