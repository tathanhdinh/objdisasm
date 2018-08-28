extern crate ansi_term;
extern crate capstone;
extern crate strum;
extern crate tabwriter;

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
    let arg = args::DisasmArg::new()?;
    let mut dm = disasm::Disassembler::new(&arg)?;
    dm.disasm(&arg.assembly, arg.address)
}
