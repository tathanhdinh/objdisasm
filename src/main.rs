extern crate capstone;
extern crate strum;

#[macro_use]
extern crate structopt;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate maplit;

#[macro_use]
mod error;
mod args;
mod disasm;

fn main() -> error::Result<()> {
    // println!("Hello, world!");
    let cfg = args::DisasmArg::new()?;
    let dm = disasm::Disassembler::new(&cfg)?;
    Ok(())
}
