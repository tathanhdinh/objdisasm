#![allow(non_camel_case_types)]

use capstone::{
    Arch as CsArch, Capstone, Endian as CsEndian, Error as CsError, ExtraMode as CsExtraMode,
    Mode as CsMode, Syntax as CsSyntax,
};
use structopt::StructOpt;
use strum::{AsStaticRef, IntoEnumIterator};

use std::{
    collections::HashMap,
    io::{self as stdio, Read},
    string::ToString,
};

use error::{Error, Result};

macro_rules! select_supported_archs {
    ($($arch:path),*) => {
        {
            let mut supported_archs = vec![];
            $(
                if Capstone::supports_arch($arch) {
                    supported_archs.push($arch);
                }
            )*
            supported_archs
        }
    };
}

macro_rules! add_arch_names {
    ($arch_names:ident, $arch_iter:expr) => {
        $arch_names.extend_from_slice(&$arch_iter.map(|e| e.as_static()).collect::<Vec<_>>());
    };
}

#[derive(AsStaticStr, EnumIter)]
enum X86ArchMode {
    #[strum(serialize = "x16")]
    x16,

    #[strum(serialize = "x16att")]
    x16att,

    #[strum(serialize = "x32")]
    x32,

    #[strum(serialize = "x32att")]
    x32att,

    #[strum(serialize = "x64")]
    x64,

    #[strum(serialize = "x64att")]
    x64att,
}

#[derive(AsStaticStr, EnumIter)]
enum ArmArchMode {
    #[strum(serialize = "arm")]
    arm,

    #[strum(serialize = "armbe")]
    armbe,

    #[strum(serialize = "thumb")]
    thumb,

    #[strum(serialize = "thumbbe")]
    thumbbe,

    #[strum(serialize = "cortexm")]
    cortexm,
}

#[derive(AsStaticStr, EnumIter)]
enum Arm64ArchMode {
    #[strum(serialize = "arm64")]
    arm64,

    #[strum(serialize = "arm64be")]
    arm64be,
}

#[derive(AsStaticStr, EnumIter)]
enum MipsArchMode {
    #[strum(serialize = "mips")]
    mips,

    #[strum(serialize = "mipsbe")]
    mipsbe,

    #[strum(serialize = "mips64")]
    mips64,

    #[strum(serialize = "mips64be")]
    mips64be,
}

#[derive(AsStaticStr, EnumIter)]
enum PpcArchMode {
    #[strum(serialize = "ppc64")]
    ppc64,

    #[strum(serialize = "ppc64be")]
    ppc64be,
}

#[derive(AsStaticStr, EnumIter)]
enum SparcArchMode {
    #[strum(serialize = "sparc")]
    sparc,
}

#[derive(AsStaticStr, EnumIter)]
enum SystemZArchMode {
    #[strum(serialize = "systemz")]
    systemz,
}

#[derive(AsStaticStr, EnumIter)]
enum XCoreArchMode {
    #[strum(serialize = "xcore")]
    xcore,
}

lazy_static! {
    pub(crate) static ref SUPPORTED_ARCHS: Vec<CsArch> =
        select_supported_archs![CsArch::X86, CsArch::ARM, CsArch::ARM64, CsArch::MIPS,
                                CsArch::PPC, CsArch::SPARC, CsArch::SYSZ, CsArch::XCORE];

    static ref SUPPORTED_ARCH_MODE_NAMES: Vec<&'static str> = {
        let mut value_strings = vec![];
        SUPPORTED_ARCHS.iter().for_each(|arch| {
            match arch {
                CsArch::X86 => {
                    // let x = X86ArchMode::iter().map(|e| e.to_string());
                    // let x: Vec<_> = X86ArchMode::iter().map(|e| e.as_static()).collect();
                    // value_strings.extend_from_slice(&["x16", "x32", "x64", "x16att", "x32att", "x64att"]);
                    add_arch_names!(value_strings, X86ArchMode::iter());
                    // value_strings.extend_from_slice(&X86ArchMode::iter().map(|e| e.as_static()).collect::<Vec<_>>());
                },

                CsArch::ARM => {
                    // value_strings.extend_from_slice(&["arm", "armbe", "thumb", "thumbbe", "cortexm"]);
                    add_arch_names!(value_strings, ArmArchMode::iter());
                },

                CsArch::ARM64 => {
                    // value_strings.extend_from_slice(&["arm64", "arm64be"]);
                    add_arch_names!(value_strings, Arm64ArchMode::iter());
                },

                CsArch::MIPS => {
                    // value_strings.extend_from_slice(&["mips", "mipsbe", "mips64", "mips64be"]);
                    add_arch_names!(value_strings, MipsArchMode::iter());
                },

                CsArch::PPC => {
                    // value_strings.extend_from_slice(&["ppc64", "ppc64be"]);
                    add_arch_names!(value_strings, PpcArchMode::iter());
                },

                CsArch::SPARC => {
                    // value_strings.extend_from_slice(&["sparc"]);
                    add_arch_names!(value_strings, SparcArchMode::iter());
                },

                CsArch::SYSZ => {
                    // value_strings.extend_from_slice(&["systemz"]);
                    add_arch_names!(value_strings, SystemZArchMode::iter());
                },

                CsArch::XCORE => {
                    // value_strings.extend_from_slice(&["xcore"]);
                    add_arch_names!(value_strings, XCoreArchMode::iter());
                },
            }
        });
        value_strings
    };

    static ref ALL_ARCH_MODE_COMBINATIONS:
        HashMap<&'static str,
                (CsArch, CsMode, Option<CsExtraMode>, Option<CsEndian>, Option<CsSyntax>)> = {
        hashmap! {
            X86ArchMode::x16.as_static() => (CsArch::X86, CsMode::Mode16, None, None, Some(CsSyntax::Intel)),
            X86ArchMode::x16att.as_static() => (CsArch::X86, CsMode::Mode16, None, None, Some(CsSyntax::Att)),
            X86ArchMode::x32.as_static() => (CsArch::X86, CsMode::Mode32, None, None, Some(CsSyntax::Intel)),
            X86ArchMode::x32att.as_static() => (CsArch::X86, CsMode::Mode32, None, None, Some(CsSyntax::Att)),
            X86ArchMode::x64.as_static() => (CsArch::X86, CsMode::Mode64, None, None, Some(CsSyntax::Intel)),
            X86ArchMode::x64att.as_static() => (CsArch::X86, CsMode::Mode64, None, None, Some(CsSyntax::Att)),

            ArmArchMode::arm.as_static() => (CsArch::ARM, CsMode::Arm, None, None, None),
            ArmArchMode::armbe.as_static() => (CsArch::ARM, CsMode::Arm, None, Some(CsEndian::Big), None),
            ArmArchMode::thumb.as_static() => (CsArch::ARM, CsMode::Thumb, None, None, None),
            ArmArchMode::thumbbe.as_static() => (CsArch::ARM, CsMode::Thumb, None, Some(CsEndian::Big), None),
            ArmArchMode::cortexm.as_static() => (CsArch::ARM, CsMode::Arm, Some(CsExtraMode::MClass), None, None),

            Arm64ArchMode::arm64.as_static() => (CsArch::ARM64, CsMode::Arm, None, Some(CsEndian::Little), None),
            Arm64ArchMode::arm64be.as_static() => (CsArch::ARM64, CsMode::Arm, None, Some(CsEndian::Big), None),
        }
    };

    static ref ABOUT_MESSAGE: String = {
        let (major_ver, minor_ver) = Capstone::lib_version();
        format!("A better objdump (using Capstone disassembler engine v{}.{})", major_ver, minor_ver)
    };
}

// static DEFAULT_MODE_NAME: &'static str = "x64";

#[derive(StructOpt)]
#[structopt(name = "disasm", raw(about = "ABOUT_MESSAGE.as_str()"))]
struct Arg {
    #[structopt(name = "hex", long = "hex", help = "assembly hex string")]
    hex_asm: Option<String>,

    // ref: https://bit.ly/2MuWga7
    #[structopt(
        name = "<arch+mode>",
        short = "m",
        long = "am",
        help = "disassembly architecture and mode combination",
        raw(
            possible_values = "&SUPPORTED_ARCH_MODE_NAMES",
            case_insensitive = "false"
        ),
        raw(default_value = "&X86ArchMode::x64.as_static()")
    )]
    arch_mode: String,

    #[structopt(
        name = "base_address",
        short = "a",
        long = "address",
        help = "base address"
    )]
    address: u64,

    #[structopt(name = "verbosity", short = "v", long = "verbose")]
    verbosity: bool,
}

pub(crate) struct DisasmArg {
    pub arch: CsArch,
    pub mode: CsMode,
    pub extra_mode: Option<CsExtraMode>,
    pub endian: Option<CsEndian>,
    pub syntax: Option<CsSyntax>,
    pub base_address: u64,
    pub verbosity: bool,
    pub assembly: Vec<u8>,
}

impl DisasmArg {
    pub fn new() -> Result<Self> {
        // ref: Convert string of hex into vector of bytes. https://bit.ly/2PcO3pG
        fn parse_assembly(hex_asm: &str) -> Vec<u8> {
            let mut hex_bytes = hex_asm
                .as_bytes()
                .iter()
                .filter_map(|b| match b {
                    &b'0'...b'9' => Some(b - b'0'),
                    &b'a'...b'f' => Some(b - b'a'),
                    &b'A'...b'F' => Some(b - b'A'),
                    _ => None,
                }).fuse();

            let mut bytes = vec![];
            while let (Some(h), Some(l)) = (hex_bytes.next(), hex_bytes.next()) {
                bytes.push(h << 4 | l)
            }

            bytes
        }

        if SUPPORTED_ARCHS.is_empty() {
            return Err(Error::Capstone(CsError::CustomError(
                "No architecture supported",
            )));
        }

        let arg = Arg::from_args();
        let arch_mode = ALL_ARCH_MODE_COMBINATIONS
            .get(&arg.arch_mode[..])
            .ok_or(application_error!("Unsupported <arch+mode>"))?;
        let assembly = {
            if let Some(ref hex_asm) = arg.hex_asm {
                parse_assembly(hex_asm)
            } else {
                let stdin = stdio::stdin();
                let mut buf = Vec::with_capacity(1024);
                stdin.lock().read_to_end(&mut buf).map_err(Error::Io)?;
                buf
            }
        };

        Ok(DisasmArg {
            arch: arch_mode.0,
            mode: arch_mode.1,
            extra_mode: arch_mode.2,
            endian: arch_mode.3,
            syntax: arch_mode.4,
            base_address: arg.address,
            verbosity: arg.verbosity,
            assembly,
        })
    }
}
