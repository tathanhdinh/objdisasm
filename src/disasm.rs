use ansi_term::Colour;
use capstone::{Capstone, Insn};
use std::io::{self, BufWriter, Write};
use tabwriter::TabWriter;

use args::DisasmArg;
use error::{Error, Result};

pub(self) struct Printer<Bw: Write> {
    writer: TabWriter<Bw>,
    verbosity: u8,
    inst_strings: Vec<String>,
}

impl<Bw> Printer<Bw>
where
    Bw: Write,
{
    fn new(verbosity: u8, out: Bw) -> Self {
        Printer {
            writer: TabWriter::new(out),
            verbosity,
            inst_strings: vec![],
        }
    }

    fn queue(&mut self, inst: &Insn) -> Result<()> {
        let inst_string = {
            let inst_bytes = &inst
                .bytes()
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<_>>()
                .join(" ")[..];

            let inst_str = &{
                let mnemonic = inst
                    .mnemonic()
                    .ok_or_else(|| application_error!("cannot get instruction mnemonic"))?;

                let op_str = inst.op_str().unwrap_or(&"");
                format!("{} {}", mnemonic, op_str)
            }[..];

            let inst_str = Colour::RGB(66, 158, 244).paint(inst_str);

            match self.verbosity {
                0 => format!("{}", inst_str),
                1 => format!("0x{:016x}\t{}", inst.address(), inst_str),
                _ => format!("0x{:016x}\t{:45}\t{}", inst.address(), inst_bytes, inst_str),
            }
        };

        self.inst_strings.push(inst_string);

        Ok(())
    }

    fn show(&mut self) -> Result<()> {
        let all_inst_strings = self.inst_strings.join("\n");
        writeln!(self.writer, "{}", &all_inst_strings);
        self.writer.flush().map_err(Error::Io)
    }
}

pub(super) struct Disassembler<'a> {
    cs: Capstone<'a>,
    verbosity: u8,
}

impl<'a> Disassembler<'a> {
    pub fn new(arg: &DisasmArg) -> Result<Self> {
        let mut cs = Capstone::new_raw(arg.arch, arg.mode, arg.extra_mode.into_iter(), arg.endian)
            .map_err(Error::Capstone)?;

        cs.set_detail(arg.detail).map_err(Error::Capstone)?;
        if let Some(syntax) = arg.syntax {
            cs.set_syntax(syntax).map_err(Error::Capstone)?;
        }

        Ok(Disassembler {
            cs,
            verbosity: arg.verbosity,
        })
    }

    pub fn disasm(&mut self, code: &[u8], address: u64) -> Result<()> {
        let stdout = io::stdout();
        let stdout = BufWriter::new(stdout.lock());
        let mut printer = Printer::new(self.verbosity, stdout);

        let insts = self.cs.disasm_all(code, address).map_err(Error::Capstone)?;
        insts.iter().try_for_each(|i| printer.queue(&i))?;

        printer.show()
    }
}
