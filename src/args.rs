#![allow(non_camel_case_types)]

use capstone::{Capstone, 
               Arch as CsArch, 
               Mode as CsMode, 
               ExtraMode as CsExtraMode,
               Syntax as CsSyntax, 
               Endian as CsEndian, 
               Error as CsError};
use structopt::StructOpt;
use strum::{AsStaticRef, IntoEnumIterator};

use std::io::{self as stdio, Read};
use std::string::ToString;
use std::collections::HashMap;

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
    #[strum(serialize="x16")]
    x16,

    #[strum(serialize="x16att")]
    x16tt,

    #[strum(serialize="x32")]
    x32,

    #[strum(serialize="x32att")]
    x32att,

    #[strum(serialize="x64")]
    x64,

    #[strum(serialize="x64att")]
    x64att,
}

#[derive(AsStaticStr, EnumIter)]
enum ArmArchMode {
    #[strum(serialize="arm")]
    arm,

    #[strum(serialize="armbe")]
    armbe,

    #[strum(serialize="thumb")]
    thumb,

    #[strum(serialize="thumbbe")]
    thumbbe,

    #[strum(serialize="cortexm")]
    cortexm,
}

#[derive(AsStaticStr, EnumIter)]
enum Arm64ArchMode {
    #[strum(serialize="arm64")]
    arm64,

    #[strum(serialize="arm64be")]
    arm64be,
}

#[derive(AsStaticStr, EnumIter)]
enum MipsArchMode {
    #[strum(serialize="mips")]
    mips,

    #[strum(serialize="mipsbe")]
    mipsbe,

    #[strum(serialize="mips64")]
    mips64,

    #[strum(serialize="mips64be")]
    mips64be,
}

#[derive(AsStaticStr, EnumIter)]
enum PpcArchMode {
    #[strum(serialize="ppc64")]
    ppc64,

    #[strum(serialize="ppc64be")]
    ppc64be
}

#[derive(AsStaticStr, EnumIter)]
enum SparcArchMode {
    #[strum(serialize="sparc")]
    sparc,
}

#[derive(AsStaticStr, EnumIter)]
enum SystemZArchMode {
    #[strum(serialize="systemz")]
    systemz,
}

#[derive(AsStaticStr, EnumIter)]
enum XCoreArchMode {
    #[strum(serialize="xcore")]
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
            X86ArchMode::x16tt.as_static() => (CsArch::X86, CsMode::Mode16, None, None, Some(CsSyntax::Att)),
            X86ArchMode::x32.as_static() => (CsArch::X86, CsMode::Mode32, None, None, Some(CsSyntax::Intel)),
            X86ArchMode::x32att.as_static() => (CsArch::X86, CsMode::Mode32, None, None, Some(CsSyntax::Att)),
            X86ArchMode::x64.as_static() => (CsArch::X86, CsMode::Mode64, None, None, Some(CsSyntax::Intel)),
            X86ArchMode::x64att.as_static() => (CsArch::X86, CsMode::Mode64, None, None, Some(CsSyntax::Att)),

            ArmArchMode::arm.as_static() => (CsArch::ARM, CsMode::Arm, None, None, None),
            ArmArchMode::armbe.as_static() => (CsArch::ARM, CsMode::Arm, None, Some(CsEndian::Big), None),
            ArmArchMode::thumb.as_static() => (CsArch::ARM, CsMode::Thumb, None, None, None),
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
    #[structopt(name = "mode", short = "m", long = "mode", help = "disassembly mode", 
                raw(possible_values = "&SUPPORTED_ARCH_MODE_NAMES", case_insensitive = "false"), 
                raw(default_value = "&X86ArchMode::x64.as_static()"))]
    mode: String,

    #[structopt(name = "verbosity", short = "v", long = "verbose", parse(from_occurrences))]
    verbosity: u8,
}


pub(super) struct DisasmArg {
    pub mode: CsMode,
    pub arch: CsArch,
    pub syntax: CsSyntax,
    pub endian: Option<CsEndian>,
    pub verbosity: u8,
    pub assembly: Vec<u8>,
}

impl DisasmArg {
    pub fn new() -> Result<Self> {
        fn get_cs_syntax(raw_mode: &str) -> CsSyntax {
            if raw_mode.contains("att") { CsSyntax::Att } else { CsSyntax::Intel }
        }

        fn get_cs_arch(raw_mode: &str) -> CsArch {
            if raw_mode.contains("x16") || raw_mode.contains("x32") || raw_mode.contains("x64") {
                CsArch::X86
            } else if raw_mode.contains("arm") {
                CsArch::ARM
            } else if raw_mode.contains("mips") | raw_mode.contains("thumb") {
                CsArch::MIPS
            } else if raw_mode.contains("ppc") {
                CsArch::PPC
            } else if raw_mode.contains("sparc") {
                CsArch::SPARC
            } else if raw_mode.contains("systemz") {
                CsArch::SYSZ
            } else if raw_mode.contains("xcore") {
                CsArch::XCORE
            } else {
                unreachable!()
            }
        }

        fn get_cs_mode(raw_mode: &str) -> CsMode {
            if raw_mode.contains("x16") {
                CsMode::Mode16
            } else if raw_mode.contains("x32") {
                CsMode::Mode32
            } else if raw_mode.contains("x64") {
                CsMode::Mode64
            } else if raw_mode.contains("arm") {
                CsMode::Arm
            } else if raw_mode.contains("mips") {
                if raw_mode.contains("mips64") { CsMode::MipsGP64 } else { CsMode::Mips3 }
            } else if raw_mode.contains("thumb") {
                CsMode::Thumb
            } else {
                // CsMode::Default
                unreachable!()
            }
        }

        fn get_cs_endian(raw_mode: &str) -> Option<CsEndian> {
            if raw_mode.contains("be") {
                Some(CsEndian::Big)
            } else {
                Some(CsEndian::Little)
            }
        }
        
        // ref: Convert string of hex into vector of bytes. https://bit.ly/2PcO3pG
        fn parse_assembly(hex_asm: &str) -> Vec<u8> {
            let mut hex_bytes = hex_asm.as_bytes().iter().filter_map(|b| {
                match b {
                    b'0'...b'9' => Some(b - b'0'),
                    b'a'...b'f' => Some(b - b'a'),
                    b'A'...b'F' => Some(b - b'A'),
                    _ => None,
                }
            }).fuse();

            let mut bytes = vec![];
            while let(Some(h), Some(l)) = (hex_bytes.next(), hex_bytes.next()) {
                bytes.push(h << 4 |l)
            }

            bytes
        }

        if SUPPORTED_ARCHS.is_empty() {
            return Err(Error::Capstone(CsError::CustomError("No architecture supported")))
        }

        let arg = Arg::from_args();
        let raw_mode = arg.mode;
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

        Ok(DisasmArg { mode: get_cs_mode(&raw_mode), 
                       arch: get_cs_arch(&raw_mode), 
                       syntax: get_cs_syntax(&raw_mode), 
                       endian: get_cs_endian(&raw_mode),
                       verbosity: arg.verbosity,
                       assembly })
    }
}