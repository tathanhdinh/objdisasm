use std::{
    fmt,
    io::{self, BufWriter, Write},
};
use tabwriter::TabWriter;
use zydis::gen as zydisc;
use zydis::{Decoder, Formatter, FormatterProperty};

use args::{DisasmArg, MachineMode};
use error::{Error, Result};

pub(self) struct Printer<'a, Bw: Write> {
    writer: TabWriter<Bw>,
    formatter: Formatter<'a>,
    verbosity: u8,
    inst_strings: Vec<String>,
}

impl<'a, Bw> Printer<'a, Bw>
where
    Bw: Write,
{
    fn new(verbosity: u8, out: Bw) -> Result<Self> {
        let mut formatter =
            Formatter::new(zydisc::ZYDIS_FORMATTER_STYLE_INTEL).map_err(Error::Zydis)?;
        formatter
            .set_property(FormatterProperty::AddressFormat(
                zydisc::ZYDIS_ADDR_FORMAT_RELATIVE_SIGNED,
            ))
            .map_err(Error::Zydis)?;
        formatter
            .set_property(FormatterProperty::ForceMemseg(false))
            .map_err(Error::Zydis)?;
        formatter
            .set_property(FormatterProperty::ForceMemsize(false))
            .map_err(Error::Zydis)?;
        formatter
            .set_property(FormatterProperty::ImmFormat(
                zydisc::ZYDIS_IMM_FORMAT_HEX_SIGNED,
            ))
            .map_err(Error::Zydis)?;
        formatter
            .set_property(FormatterProperty::Uppercase(false))
            .map_err(Error::Zydis)?;
        formatter
            .set_property(FormatterProperty::HexUppercase(false))
            .map_err(Error::Zydis)?;
        formatter
            .set_property(FormatterProperty::HexPaddingAddr(0))
            .map_err(Error::Zydis)?;
        formatter
            .set_property(FormatterProperty::HexPaddingDisp(0))
            .map_err(Error::Zydis)?;
        formatter
            .set_property(FormatterProperty::HexPaddingImm(0))
            .map_err(Error::Zydis)?;

        Ok(Printer {
            writer: TabWriter::new(out).padding(8),
            formatter,
            verbosity,
            inst_strings: vec![],
        })
    }

    fn queue(&mut self, (inst, addr): &(zydisc::ZydisDecodedInstruction, u64)) -> Result<()> {
        let inst_string = {
            let inst_bytes_str = &inst
                .data
                .iter()
                .take(inst.length as usize)
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<_>>()
                .join(" ")[..];

            let inst_str: Box<dyn fmt::Display> = {
                let inst_str = self
                    .formatter
                    .format_instruction(inst, 100, None)
                    .map_err(Error::Zydis)?;

                Box::new(inst_str)
            };

            match self.verbosity {
                0 => format!("{}", inst_str),
                1 => format!("0x{:x}\t{}", addr, inst_str),
                _ => format!("0x{:x}\t{}\t{}", addr, inst_bytes_str, inst_str),
            }
        };

        self.inst_strings.push(inst_string);

        Ok(())
    }

    fn show(&mut self) -> Result<()> {
        let all_inst_strings = self.inst_strings.join("\n");
        writeln!(self.writer, "{}", &all_inst_strings).map_err(Error::Io)?;
        self.writer.flush().map_err(Error::Io)
    }
}

pub(super) struct Disassembler {
    decoder: Decoder,
    verbosity: u8,
}

impl Disassembler {
    pub fn new(arg: &DisasmArg) -> Result<Self> {
        let decoder = match arg.mode {
            MachineMode::x86 => Decoder::new(
                zydisc::ZYDIS_MACHINE_MODE_LONG_COMPAT_32,
                zydisc::ZYDIS_ADDRESS_WIDTH_32,
            ),

            MachineMode::amd64 => Decoder::new(
                zydisc::ZYDIS_MACHINE_MODE_LONG_64,
                zydisc::ZYDIS_ADDRESS_WIDTH_64,
            ),
        }
        .map_err(Error::Zydis)?;

        Ok(Disassembler {
            decoder,
            verbosity: arg.verbosity,
        })
    }

    pub fn disasm(&mut self, code: &[u8], address: u64) -> Result<()> {
        let stdout = io::stdout();
        let stdout = BufWriter::new(stdout.lock());
        let mut printer = Printer::new(self.verbosity, stdout)?;

        self.decoder
            .instruction_iterator(code, address)
            .try_for_each(|i| printer.queue(&i))?;

        printer.show()
    }
}
