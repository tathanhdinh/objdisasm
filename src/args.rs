#![allow(non_camel_case_types)]
use zydis;

use structopt::StructOpt;
use strum::{AsStaticRef, IntoEnumIterator};

use std::{
    io::{self as stdio, Read},
    string::ToString,
};

use error::{Error, Result};

macro_rules! add_arch_names {
    ($arch_names:ident, $arch_iter:expr) => {
        $arch_names.extend_from_slice(&$arch_iter.map(|e| e.as_static()).collect::<Vec<_>>());
    };
}

#[derive(AsStaticStr, EnumString, ToString, EnumIter)]
pub(crate) enum MachineMode {
    #[strum(serialize = "x86")]
    x86,

    #[strum(serialize = "amd64")]
    amd64,
}

lazy_static! {
    static ref SUPPORTED_MACHINE_MODE_NAMES: Vec<&'static str> = {
        let mut value_strings = vec![];
        add_arch_names!(value_strings, MachineMode::iter());
        value_strings
    };
    static ref ABOUT_MESSAGE: String = {
        let (major, minor, patch, build) = zydis::get_version();
        format!(
            "A simple objdump (using Zydis disassembler library v{}.{}.{}.{})",
            major, minor, patch, build
        )
    };
}

fn try_parse_number(num_str: &str) -> Result<u64> {
    // TODO: reimplement
    if num_str.find(|c: char| !c.is_ascii_digit()).is_some() {
        let num_str = &num_str
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()[..];

        u64::from_str_radix(num_str, 16).map_err(|_| {
            application_error!(format!("{} is not a valid hexadecimal number", num_str))
        })
    } else {
        u64::from_str_radix(num_str, 10)
            .map_err(|_| application_error!(format!("{} is not a valid decimal number", num_str)))
    }
}

#[derive(StructOpt)]
#[structopt(name = "disasm", raw(about = "ABOUT_MESSAGE.as_str()"))]
struct Arg {
    #[structopt(
        name = "assembly",
        help = "Assembly hex string or read from stdin"
    )]
    hex_asm: Option<String>,

    // ref: https://bit.ly/2MuWga7
    #[structopt(
        name = "machine_mode",
        short = "m",
        help = "Disassembly architecture and mode combination",
        raw(
            possible_values = "&SUPPORTED_MACHINE_MODE_NAMES",
            case_insensitive = "false"
        ),
        raw(default_value = "&MachineMode::amd64.as_static()")
    )]
    mode: MachineMode,

    #[structopt(
        name = "base_address",
        short = "a",
        long = "address",
        default_value = "0",
        parse(try_from_str = "try_parse_number"),
        help = "Base address (hex or decimal)"
    )]
    address: u64,

    #[structopt(
        name = "show_detail",
        short = "d",
        long = "detail",
        help = "Show instruction detail"
    )]
    detail: bool,

    #[structopt(
        name = "asm_highlighting",
        short = "l",
        help = "Hilighting instructions"
    )]
    hilight: bool,

    #[structopt(
        name = "verbosity",
        short = "v",
        long = "verbose",
        parse(from_occurrences),
        help = "Verbosity"
    )]
    verbosity: u8,
}

pub(crate) struct DisasmArg {
    pub mode: MachineMode,
    pub address: u64,
    pub detail: bool,
    pub hilight: bool,
    pub verbosity: u8,
    pub assembly: Vec<u8>,
}

impl DisasmArg {
    pub fn new() -> Result<Self> {
        const DEFAULT_INPUT_SIZE: usize = 1024;

        // ref: Convert string of hex into vector of bytes. https://bit.ly/2PcO3pG
        fn parse_assembly(hex_asm: &str) -> Vec<u8> {
            let mut hex_bytes = hex_asm
                .as_bytes()
                .iter()
                .filter_map(|b| match b {
                    b'0'...b'9' => Some(b - b'0'),
                    b'a'...b'f' => Some(b - b'a' + 10),
                    b'A'...b'F' => Some(b - b'A' + 10),
                    _ => None,
                }).fuse();

            let mut bytes = vec![];
            while let (Some(h), Some(l)) = (hex_bytes.next(), hex_bytes.next()) {
                bytes.push(h << 4 | l)
            }

            bytes
        }

        let arg = Arg::from_args();

        let assembly = {
            if let Some(ref hex_asm) = arg.hex_asm {
                parse_assembly(hex_asm)
            } else {
                let stdin = stdio::stdin();
                let mut buf = Vec::with_capacity(DEFAULT_INPUT_SIZE);
                stdin.lock().read_to_end(&mut buf).map_err(Error::Io)?;
                buf
            }
        };

        Ok(DisasmArg {
            mode: arg.mode,
            address: arg.address,
            detail: arg.detail,
            hilight: arg.hilight,
            verbosity: arg.verbosity,
            assembly,
        })
    }
}
