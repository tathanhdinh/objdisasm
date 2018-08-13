use capstone::{Capstone, Arch as CsArch, 
               Mode as CsMode, ExtraMode as CsExtraMode, 
               Syntax as CsSyntax, Endian as CsEndian, Error as CsError};

use error::{Error, Result};

struct Disassembler {
    cs: Capstone,
    verbosity: u8,
}

impl Disassembler {
    fn new(arch: Arch, ) -> Result<Self> 
}