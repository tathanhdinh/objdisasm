use std::iter;

use capstone::{
    Arch as CsArch, Capstone, Endian as CsEndian, Error as CsError, ExtraMode as CsExtraMode,
    Mode as CsMode, Syntax as CsSyntax, NO_EXTRA_MODE,
};

use args::DisasmArg;
use error::{Error, Result};

pub(super) struct Disassembler<'a> {
    cs: Capstone<'a>,
    verbosity: bool,
}

impl<'a> Disassembler<'a> {
    pub fn new(arg: &DisasmArg) -> Result<Self> {
        let cs = Capstone::new_raw(arg.arch, arg.mode, arg.extra_mode.into_iter(), arg.endian)
            .map_err(Error::Capstone)?;
        Ok(Disassembler {
            cs,
            verbosity: arg.verbosity,
        })
    }
}
