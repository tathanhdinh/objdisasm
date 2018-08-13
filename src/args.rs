use capstone::{Capstone, Arch};
use structopt::StructOpt;

macro_rules! select_supported_archs {
    ($($arch:expr),*) => {
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

lazy_static! {
    pub(crate) static ref SUPPORTED_ARCHS: Vec<Arch> = select_supported_archs![Arch::X86, Arch::ARM, Arch::ARM64, Arch::MIPS, Arch::PPC, Arch::SPARC, Arch::SYSZ, Arch::XCORE];
    static ref SUPPORTED_ARCH_MODE_VALUES: Vec<&'static str> = {
        let mut value_strings = vec![];
        SUPPORTED_ARCHS.iter().for_each(|arch| {
            match arch {
                Arch::X86 => {
                    value_strings.extend_from_slice(&["x16", "x32", "x64", "x16att", "x32att", "x64att"]);
                },

                Arch::ARM => {
                    value_strings.extend_from_slice(&["arm", "armbe", "thumb", "thumbbe", "cortexm"]);
                },

                Arch::ARM64 => {
                    value_strings.extend_from_slice(&["arm64", "arm64be"]);
                },

                Arch::MIPS => {
                    value_strings.extend_from_slice(&["mips", "mipsbe", "mips64", "mips64be"]);
                },

                Arch::PPC => {
                    value_strings.extend_from_slice(&["ppc64", "ppc64be"]);
                },

                Arch::SPARC => {
                    value_strings.extend_from_slice(&["sparc"]);
                },

                Arch::SYSZ => {
                    value_strings.extend_from_slice(&["systemz"]);
                },

                Arch::XCORE => {
                    value_strings.extend_from_slice(&["xcore"]);
                },
            }
        });
        value_strings
    };
    // static ref DEFAULT_MODE_VALUE: &'static str = "x64";
    static ref ABOUT_MESSAGE: String = {
        let (major_ver, minor_ver) = Capstone::lib_version();
        format!("A better objdump (using Capstone disassembler engine v{}.{})", major_ver, minor_ver)
    };
}

static DEFAULT_MODE_VALUE: &'static str = "x64";

#[derive(StructOpt)]
#[structopt(name = "disasm", raw(about = "ABOUT_MESSAGE.as_str()"))]
struct Arg {
    #[structopt(name = "input", help = "assembly hex string")]
    asm: String,

    #[structopt(name = "mode", short = "m", long = "mode", help = "disassembly mode", 
                raw(possible_values = "&SUPPORTED_ARCH_MODE_VALUES", case_insensitive = "false"), 
                raw(default_value = "&DEFAULT_MODE_VALUE"))]
    mode: String,

    #[structopt(name = "verbosity", short = "v", long = "verbose", parse(from_occurrences))]
    verbosity: u8,
}


struct DisasmArg {
    
}