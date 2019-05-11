// This file defines all the identifier enums and target-aware logic.

use crate::triple::{Endianness, PointerWidth, Triple};
use core::fmt;
use core::str::FromStr;

/// The "architecture" field, which in some cases also specifies a specific
/// subarchitecture.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Architecture {
    Unknown,
    Arm(ArmArchitecture),
    Aarch64(Aarch64Architecture),
    Asmjs,
    I386,
    I586,
    I686,
    Mips,
    Mips64,
    Mips64el,
    Mipsel,
    Msp430,
    Powerpc,
    Powerpc64,
    Powerpc64le,
    Riscv32,
    Riscv32imac,
    Riscv32imc,
    Riscv64,
    S390x,
    Sparc,
    Sparc64,
    Sparcv9,
    Wasm32,
    X86_64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum ArmArchitecture {
    Arm, // Generic arm
    Armeb,
    Armv4,
    Armv4t,
    Armv5t,
    Armv5te,
    Armv5tej,
    Armv6,
    Armv6j,
    Armv6k,
    Armv6z,
    Armv6kz,
    Armv6t2,
    Armv6m,
    Armv7,
    Armv7a,
    Armv7ve,
    Armv7m,
    Armv7r,
    Armv7s,
    Armv8,
    Armv8a,
    Armv8_1a,
    Armv8_2a,
    Armv8_3a,
    Armv8_4a,
    Armv8_5a,
    Armv8mBase,
    Armv8mMain,
    Armv8r,

    Armebv7r,

    Thumbeb,
    Thumbv6m,
    Thumbv7a,
    Thumbv7em,
    Thumbv7m,
    Thumbv7neon,
    Thumbv8mBase,
    Thumbv8mMain,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Aarch64Architecture {
    Aarch64,
    Aarch64be,
}

// #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
// #[allow(missing_docs)]
// pub enum ArmFpu {
//     Vfp,
//     Vfpv2,
//     Vfpv3,
//     Vfpv3Fp16,
//     Vfpv3Xd,
//     Vfpv3XdFp16,
//     Neon,
//     NeonVfpv3,
//     NeonVfpv4,
//     Vfpv4,
//     Vfpv4D16,
//     Fpv4SpD16,
//     Fpv5SpD16,
//     Fpv5D16,
//     FpArmv8,
//     NeonFpArmv8,
//     CryptoNeonFpArmv8,
// }

impl ArmArchitecture {
    pub fn is_thumb(self) -> Result<bool, ()> {
        match self {
            ArmArchitecture::Arm
            | ArmArchitecture::Armeb
            | ArmArchitecture::Armv4
            | ArmArchitecture::Armv4t
            | ArmArchitecture::Armv5t
            | ArmArchitecture::Armv5te
            | ArmArchitecture::Armv5tej
            | ArmArchitecture::Armv6
            | ArmArchitecture::Armv6j
            | ArmArchitecture::Armv6k
            | ArmArchitecture::Armv6z
            | ArmArchitecture::Armv6kz
            | ArmArchitecture::Armv6t2
            | ArmArchitecture::Armv6m
            | ArmArchitecture::Armv7
            | ArmArchitecture::Armv7a
            | ArmArchitecture::Armv7ve
            | ArmArchitecture::Armv7m
            | ArmArchitecture::Armv7r
            | ArmArchitecture::Armv7s
            | ArmArchitecture::Armv8
            | ArmArchitecture::Armv8a
            | ArmArchitecture::Armv8_1a
            | ArmArchitecture::Armv8_2a
            | ArmArchitecture::Armv8_3a
            | ArmArchitecture::Armv8_4a
            | ArmArchitecture::Armv8_5a
            | ArmArchitecture::Armv8mBase
            | ArmArchitecture::Armv8mMain
            | ArmArchitecture::Armv8r
            | ArmArchitecture::Armebv7r => Ok(false),
            ArmArchitecture::Thumbeb
            | ArmArchitecture::Thumbv6m
            | ArmArchitecture::Thumbv7a
            | ArmArchitecture::Thumbv7em
            | ArmArchitecture::Thumbv7m
            | ArmArchitecture::Thumbv7neon
            | ArmArchitecture::Thumbv8mBase
            | ArmArchitecture::Thumbv8mMain => Ok(true),
        }
    }

    // pub fn has_fpu(self) -> Result<&'static [ArmFpu], ()> {

    // }

    pub fn pointer_width(self) -> Result<PointerWidth, ()> {
        match self {
            ArmArchitecture::Arm
            | ArmArchitecture::Armeb
            | ArmArchitecture::Armv4
            | ArmArchitecture::Armv4t
            | ArmArchitecture::Armv5t
            | ArmArchitecture::Armv5te
            | ArmArchitecture::Armv5tej
            | ArmArchitecture::Armv6
            | ArmArchitecture::Armv6j
            | ArmArchitecture::Armv6k
            | ArmArchitecture::Armv6z
            | ArmArchitecture::Armv6kz
            | ArmArchitecture::Armv6t2
            | ArmArchitecture::Armv6m
            | ArmArchitecture::Armv7
            | ArmArchitecture::Armv7a
            | ArmArchitecture::Armv7ve
            | ArmArchitecture::Armv7m
            | ArmArchitecture::Armv7r
            | ArmArchitecture::Armv7s
            | ArmArchitecture::Armv8
            | ArmArchitecture::Armv8a
            | ArmArchitecture::Armv8_1a
            | ArmArchitecture::Armv8_2a
            | ArmArchitecture::Armv8_3a
            | ArmArchitecture::Armv8_4a
            | ArmArchitecture::Armv8_5a
            | ArmArchitecture::Armv8mBase
            | ArmArchitecture::Armv8mMain
            | ArmArchitecture::Armv8r
            | ArmArchitecture::Armebv7r
            | ArmArchitecture::Thumbeb
            | ArmArchitecture::Thumbv6m
            | ArmArchitecture::Thumbv7a
            | ArmArchitecture::Thumbv7em
            | ArmArchitecture::Thumbv7m
            | ArmArchitecture::Thumbv7neon
            | ArmArchitecture::Thumbv8mBase
            | ArmArchitecture::Thumbv8mMain => Ok(PointerWidth::U32),
        }
    }

    pub fn endianness(self) -> Result<Endianness, ()> {
        match self {
            ArmArchitecture::Arm
            | ArmArchitecture::Armv4
            | ArmArchitecture::Armv4t
            | ArmArchitecture::Armv5t
            | ArmArchitecture::Armv5te
            | ArmArchitecture::Armv5tej
            | ArmArchitecture::Armv6
            | ArmArchitecture::Armv6j
            | ArmArchitecture::Armv6k
            | ArmArchitecture::Armv6z
            | ArmArchitecture::Armv6kz
            | ArmArchitecture::Armv6t2
            | ArmArchitecture::Armv6m
            | ArmArchitecture::Armv7
            | ArmArchitecture::Armv7a
            | ArmArchitecture::Armv7ve
            | ArmArchitecture::Armv7m
            | ArmArchitecture::Armv7r
            | ArmArchitecture::Armv7s
            | ArmArchitecture::Armv8
            | ArmArchitecture::Armv8a
            | ArmArchitecture::Armv8_1a
            | ArmArchitecture::Armv8_2a
            | ArmArchitecture::Armv8_3a
            | ArmArchitecture::Armv8_4a
            | ArmArchitecture::Armv8_5a
            | ArmArchitecture::Armv8mBase
            | ArmArchitecture::Armv8mMain
            | ArmArchitecture::Armv8r
            | ArmArchitecture::Thumbv6m
            | ArmArchitecture::Thumbv7a
            | ArmArchitecture::Thumbv7em
            | ArmArchitecture::Thumbv7m
            | ArmArchitecture::Thumbv7neon
            | ArmArchitecture::Thumbv8mBase
            | ArmArchitecture::Thumbv8mMain => Ok(Endianness::Little),
            ArmArchitecture::Armeb | ArmArchitecture::Armebv7r | ArmArchitecture::Thumbeb => {
                Ok(Endianness::Big)
            }
        }
    }
}

impl Aarch64Architecture {
    pub fn is_thumb(self) -> Result<bool, ()> {
        match self {
            Aarch64Architecture::Aarch64 | Aarch64Architecture::Aarch64be => Ok(false),
        }
    }

    // pub fn has_fpu(self) -> Result<&'static [ArmFpu], ()> {

    // }

    pub fn pointer_width(self) -> Result<PointerWidth, ()> {
        match self {
            Aarch64Architecture::Aarch64 | Aarch64Architecture::Aarch64be => Ok(PointerWidth::U64),
        }
    }

    pub fn endianness(self) -> Result<Endianness, ()> {
        match self {
            Aarch64Architecture::Aarch64 => Ok(Endianness::Little),
            Aarch64Architecture::Aarch64be => Ok(Endianness::Big),
        }
    }
}

/// The "vendor" field, which in practice is little more than an arbitrary
/// modifier.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Vendor {
    Unknown,
    Apple,
    Experimental,
    Fortanix,
    Pc,
    Rumprun,
    Sun,
}

/// The "operating system" field, which sometimes implies an environment, and
/// sometimes isn't an actual operating system.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum OperatingSystem {
    Unknown,
    Bitrig,
    Cloudabi,
    Darwin,
    Dragonfly,
    Emscripten,
    Freebsd,
    Fuchsia,
    Haiku,
    Hermit,
    Ios,
    L4re,
    Linux,
    Nebulet,
    Netbsd,
    None_,
    Openbsd,
    Redox,
    Solaris,
    Uefi,
    Windows,
}

/// The "environment" field, which specifies an ABI environment on top of the
/// operating system. In many configurations, this field is omitted, and the
/// environment is implied by the operating system.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Environment {
    Unknown,
    Android,
    Androideabi,
    Eabi,
    Eabihf,
    Gnu,
    Gnuabi64,
    Gnueabi,
    Gnueabihf,
    Gnuspe,
    Gnux32,
    Musl,
    Musleabi,
    Musleabihf,
    Msvc,
    Uclibc,
    Sgx,
}

/// The "binary format" field, which is usually omitted, and the binary format
/// is implied by the other fields.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum BinaryFormat {
    Unknown,
    Elf,
    Coff,
    Macho,
    Wasm,
}

impl Architecture {
    /// Return the endianness of this architecture.
    pub fn endianness(self) -> Result<Endianness, ()> {
        match self {
            Architecture::Unknown => Err(()),
            Architecture::Arm(arm) => arm.endianness(),
            Architecture::Aarch64(aarch) => aarch.endianness(),
            Architecture::Asmjs
            | Architecture::I386
            | Architecture::I586
            | Architecture::I686
            | Architecture::Mips64el
            | Architecture::Mipsel
            | Architecture::Msp430
            | Architecture::Powerpc64le
            | Architecture::Riscv32
            | Architecture::Riscv32imac
            | Architecture::Riscv32imc
            | Architecture::Riscv64
            | Architecture::Wasm32
            | Architecture::X86_64 => Ok(Endianness::Little),
            Architecture::Mips
            | Architecture::Mips64
            | Architecture::Powerpc
            | Architecture::Powerpc64
            | Architecture::S390x
            | Architecture::Sparc
            | Architecture::Sparc64
            | Architecture::Sparcv9 => Ok(Endianness::Big),
        }
    }

    /// Return the pointer bit width of this target's architecture.
    pub fn pointer_width(self) -> Result<PointerWidth, ()> {
        match self {
            Architecture::Unknown => Err(()),
            Architecture::Msp430 => Ok(PointerWidth::U16),
            Architecture::Arm(arm) => arm.pointer_width(),
            Architecture::Aarch64(aarch) => aarch.pointer_width(),
            Architecture::Asmjs
            | Architecture::I386
            | Architecture::I586
            | Architecture::I686
            | Architecture::Mipsel
            | Architecture::Riscv32
            | Architecture::Riscv32imac
            | Architecture::Riscv32imc
            | Architecture::Sparc
            | Architecture::Wasm32
            | Architecture::Mips
            | Architecture::Powerpc => Ok(PointerWidth::U32),
            Architecture::Mips64el
            | Architecture::Powerpc64le
            | Architecture::Riscv64
            | Architecture::X86_64
            | Architecture::Mips64
            | Architecture::Powerpc64
            | Architecture::S390x
            | Architecture::Sparc64
            | Architecture::Sparcv9 => Ok(PointerWidth::U64),
        }
    }
}

/// Return the binary format implied by this target triple, ignoring its
/// `binary_format` field.
pub fn default_binary_format(triple: &Triple) -> BinaryFormat {
    match triple.operating_system {
        OperatingSystem::None_ => BinaryFormat::Unknown,
        OperatingSystem::Darwin | OperatingSystem::Ios => BinaryFormat::Macho,
        OperatingSystem::Windows => BinaryFormat::Coff,
        OperatingSystem::Nebulet | OperatingSystem::Emscripten | OperatingSystem::Unknown => {
            match triple.architecture {
                Architecture::Wasm32 => BinaryFormat::Wasm,
                _ => BinaryFormat::Unknown,
            }
        }
        _ => BinaryFormat::Elf,
    }
}

impl fmt::Display for ArmArchitecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            ArmArchitecture::Arm => "arm",
            ArmArchitecture::Armeb => "armeb",
            ArmArchitecture::Armv4 => "armv4",
            ArmArchitecture::Armv4t => "armv4t",
            ArmArchitecture::Armv5t => "armv5t",
            ArmArchitecture::Armv5te => "armv5te",
            ArmArchitecture::Armv5tej => "armv5tej",
            ArmArchitecture::Armv6 => "armv6",
            ArmArchitecture::Armv6j => "armv6j",
            ArmArchitecture::Armv6k => "armv6k",
            ArmArchitecture::Armv6z => "armv6z",
            ArmArchitecture::Armv6kz => "armv6kz",
            ArmArchitecture::Armv6t2 => "armv6t2",
            ArmArchitecture::Armv6m => "armv6m",
            ArmArchitecture::Armv7 => "armv7",
            ArmArchitecture::Armv7a => "armv7a",
            ArmArchitecture::Armv7ve => "armv7ve",
            ArmArchitecture::Armv7m => "armv7m",
            ArmArchitecture::Armv7r => "armv7r",
            ArmArchitecture::Armv7s => "armv7s",
            ArmArchitecture::Armv8 => "armv8",
            ArmArchitecture::Armv8a => "armv8a",
            ArmArchitecture::Armv8_1a => "armv8.1a",
            ArmArchitecture::Armv8_2a => "armv8.2a",
            ArmArchitecture::Armv8_3a => "armv8.3a",
            ArmArchitecture::Armv8_4a => "armv8.4a",
            ArmArchitecture::Armv8_5a => "armv8.5a",
            ArmArchitecture::Armv8mBase => "armv8m.base",
            ArmArchitecture::Armv8mMain => "armv8m.main",
            ArmArchitecture::Armv8r => "armv8r",
            ArmArchitecture::Thumbeb => "thumbeb",
            ArmArchitecture::Thumbv6m => "thumbv6m",
            ArmArchitecture::Thumbv7a => "thumbv7a",
            ArmArchitecture::Thumbv7em => "thumbv7em",
            ArmArchitecture::Thumbv7m => "thumbv7m",
            ArmArchitecture::Thumbv7neon => "thumbv7neon",
            ArmArchitecture::Thumbv8mBase => "thumbv8m.base",
            ArmArchitecture::Thumbv8mMain => "thumbv8m.main",
            ArmArchitecture::Armebv7r => "armebv7r",
        };
        f.write_str(s)
    }
}

impl fmt::Display for Aarch64Architecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Aarch64Architecture::Aarch64 => "aarch64",
            Aarch64Architecture::Aarch64be => "aarch64be",
        };
        f.write_str(s)
    }
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Architecture::Arm(arm) => arm.fmt(f),
            Architecture::Aarch64(aarch) => aarch.fmt(f),
            Architecture::Unknown => f.write_str("unknown"),
            Architecture::Asmjs => f.write_str("asmjs"),
            Architecture::I386 => f.write_str("i386"),
            Architecture::I586 => f.write_str("i586"),
            Architecture::I686 => f.write_str("i686"),
            Architecture::Mips => f.write_str("mips"),
            Architecture::Mips64 => f.write_str("mips64"),
            Architecture::Mips64el => f.write_str("mips64el"),
            Architecture::Mipsel => f.write_str("mipsel"),
            Architecture::Msp430 => f.write_str("msp430"),
            Architecture::Powerpc => f.write_str("powerpc"),
            Architecture::Powerpc64 => f.write_str("powerpc64"),
            Architecture::Powerpc64le => f.write_str("powerpc64le"),
            Architecture::Riscv32 => f.write_str("riscv32"),
            Architecture::Riscv32imac => f.write_str("riscv32imac"),
            Architecture::Riscv32imc => f.write_str("riscv32imc"),
            Architecture::Riscv64 => f.write_str("riscv64"),
            Architecture::S390x => f.write_str("s390x"),
            Architecture::Sparc => f.write_str("sparc"),
            Architecture::Sparc64 => f.write_str("sparc64"),
            Architecture::Sparcv9 => f.write_str("sparcv9"),
            Architecture::Wasm32 => f.write_str("wasm32"),
            Architecture::X86_64 => f.write_str("x86_64"),
        }
    }
}

impl FromStr for ArmArchitecture {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "arm" => ArmArchitecture::Arm,
            "armeb" => ArmArchitecture::Armeb,
            "armv4" => ArmArchitecture::Armv4,
            "armv4t" => ArmArchitecture::Armv4t,
            "armv5t" => ArmArchitecture::Armv5t,
            "armv5te" => ArmArchitecture::Armv5te,
            "armv5tej" => ArmArchitecture::Armv5tej,
            "armv6" => ArmArchitecture::Armv6,
            "armv6j" => ArmArchitecture::Armv6j,
            "armv6k" => ArmArchitecture::Armv6k,
            "armv6z" => ArmArchitecture::Armv6z,
            "armv6kz" => ArmArchitecture::Armv6kz,
            "armv6t2" => ArmArchitecture::Armv6t2,
            "armv6m" => ArmArchitecture::Armv6m,
            "armv7" => ArmArchitecture::Armv7,
            "armv7a" => ArmArchitecture::Armv7a,
            "armv7ve" => ArmArchitecture::Armv7ve,
            "armv7m" => ArmArchitecture::Armv7m,
            "armv7r" => ArmArchitecture::Armv7r,
            "armv7s" => ArmArchitecture::Armv7s,
            "armv8" => ArmArchitecture::Armv8,
            "armv8a" => ArmArchitecture::Armv8a,
            "armv8.1a" => ArmArchitecture::Armv8_1a,
            "armv8.2a" => ArmArchitecture::Armv8_2a,
            "armv8.3a" => ArmArchitecture::Armv8_3a,
            "armv8.4a" => ArmArchitecture::Armv8_4a,
            "armv8.5a" => ArmArchitecture::Armv8_5a,
            "armv8m.base" => ArmArchitecture::Armv8mBase,
            "armv8m.main" => ArmArchitecture::Armv8mMain,
            "armv8r" => ArmArchitecture::Armv8r,
            "thumbeb" => ArmArchitecture::Thumbeb,
            "thumbv6m" => ArmArchitecture::Thumbv6m,
            "thumbv7a" => ArmArchitecture::Thumbv7a,
            "thumbv7em" => ArmArchitecture::Thumbv7em,
            "thumbv7m" => ArmArchitecture::Thumbv7m,
            "thumbv7neon" => ArmArchitecture::Thumbv7neon,
            "thumbv8m.base" => ArmArchitecture::Thumbv8mBase,
            "thumbv8m.main" => ArmArchitecture::Thumbv8mMain,
            "armebv7r" => ArmArchitecture::Armebv7r,
            _ => return Err(()),
        })
    }
}

impl FromStr for Aarch64Architecture {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "aarch64" => Aarch64Architecture::Aarch64,
            "arm64" => Aarch64Architecture::Aarch64,
            "aarch64be" => Aarch64Architecture::Aarch64be,
            _ => return Err(()),
        })
    }
}

impl FromStr for Architecture {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "unknown" => Architecture::Unknown,
            "asmjs" => Architecture::Asmjs,
            "i386" => Architecture::I386,
            "i586" => Architecture::I586,
            "i686" => Architecture::I686,
            "mips" => Architecture::Mips,
            "mips64" => Architecture::Mips64,
            "mips64el" => Architecture::Mips64el,
            "mipsel" => Architecture::Mipsel,
            "msp430" => Architecture::Msp430,
            "powerpc" => Architecture::Powerpc,
            "powerpc64" => Architecture::Powerpc64,
            "powerpc64le" => Architecture::Powerpc64le,
            "riscv32" => Architecture::Riscv32,
            "riscv32imac" => Architecture::Riscv32imac,
            "riscv32imc" => Architecture::Riscv32imc,
            "riscv64" => Architecture::Riscv64,
            "s390x" => Architecture::S390x,
            "sparc" => Architecture::Sparc,
            "sparc64" => Architecture::Sparc64,
            "sparcv9" => Architecture::Sparcv9,
            "wasm32" => Architecture::Wasm32,
            "x86_64" => Architecture::X86_64,
            _ => {
                if let Ok(arm) = ArmArchitecture::from_str(s) {
                    Architecture::Arm(arm)
                } else if let Ok(aarch64) = Aarch64Architecture::from_str(s) {
                    Architecture::Aarch64(aarch64)
                } else {
                    return Err(());
                }
            }
        })
    }
}

impl fmt::Display for Vendor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Vendor::Unknown => "unknown",
            Vendor::Apple => "apple",
            Vendor::Experimental => "experimental",
            Vendor::Fortanix => "fortanix",
            Vendor::Pc => "pc",
            Vendor::Rumprun => "rumprun",
            Vendor::Sun => "sun",
        };
        f.write_str(s)
    }
}

impl FromStr for Vendor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "unknown" => Vendor::Unknown,
            "apple" => Vendor::Apple,
            "experimental" => Vendor::Experimental,
            "fortanix" => Vendor::Fortanix,
            "pc" => Vendor::Pc,
            "rumprun" => Vendor::Rumprun,
            "sun" => Vendor::Sun,
            _ => return Err(()),
        })
    }
}

impl fmt::Display for OperatingSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            OperatingSystem::Unknown => "unknown",
            OperatingSystem::Bitrig => "bitrig",
            OperatingSystem::Cloudabi => "cloudabi",
            OperatingSystem::Darwin => "darwin",
            OperatingSystem::Dragonfly => "dragonfly",
            OperatingSystem::Emscripten => "emscripten",
            OperatingSystem::Freebsd => "freebsd",
            OperatingSystem::Fuchsia => "fuchsia",
            OperatingSystem::Haiku => "haiku",
            OperatingSystem::Hermit => "hermit",
            OperatingSystem::Ios => "ios",
            OperatingSystem::L4re => "l4re",
            OperatingSystem::Linux => "linux",
            OperatingSystem::Nebulet => "nebulet",
            OperatingSystem::Netbsd => "netbsd",
            OperatingSystem::None_ => "none",
            OperatingSystem::Openbsd => "openbsd",
            OperatingSystem::Redox => "redox",
            OperatingSystem::Solaris => "solaris",
            OperatingSystem::Uefi => "uefi",
            OperatingSystem::Windows => "windows",
        };
        f.write_str(s)
    }
}

impl FromStr for OperatingSystem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "unknown" => OperatingSystem::Unknown,
            "bitrig" => OperatingSystem::Bitrig,
            "cloudabi" => OperatingSystem::Cloudabi,
            "darwin" => OperatingSystem::Darwin,
            "dragonfly" => OperatingSystem::Dragonfly,
            "emscripten" => OperatingSystem::Emscripten,
            "freebsd" => OperatingSystem::Freebsd,
            "fuchsia" => OperatingSystem::Fuchsia,
            "haiku" => OperatingSystem::Haiku,
            "hermit" => OperatingSystem::Hermit,
            "ios" => OperatingSystem::Ios,
            "l4re" => OperatingSystem::L4re,
            "linux" => OperatingSystem::Linux,
            "nebulet" => OperatingSystem::Nebulet,
            "netbsd" => OperatingSystem::Netbsd,
            "none" => OperatingSystem::None_,
            "openbsd" => OperatingSystem::Openbsd,
            "redox" => OperatingSystem::Redox,
            "solaris" => OperatingSystem::Solaris,
            "uefi" => OperatingSystem::Uefi,
            "windows" => OperatingSystem::Windows,
            _ => return Err(()),
        })
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Environment::Unknown => "unknown",
            Environment::Android => "android",
            Environment::Androideabi => "androideabi",
            Environment::Eabi => "eabi",
            Environment::Eabihf => "eabihf",
            Environment::Gnu => "gnu",
            Environment::Gnuabi64 => "gnuabi64",
            Environment::Gnueabi => "gnueabi",
            Environment::Gnueabihf => "gnueabihf",
            Environment::Gnuspe => "gnuspe",
            Environment::Gnux32 => "gnux32",
            Environment::Musl => "musl",
            Environment::Musleabi => "musleabi",
            Environment::Musleabihf => "musleabihf",
            Environment::Msvc => "msvc",
            Environment::Uclibc => "uclibc",
            Environment::Sgx => "sgx",
        };
        f.write_str(s)
    }
}

impl FromStr for Environment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "unknown" => Environment::Unknown,
            "android" => Environment::Android,
            "androideabi" => Environment::Androideabi,
            "eabi" => Environment::Eabi,
            "eabihf" => Environment::Eabihf,
            "gnu" => Environment::Gnu,
            "gnuabi64" => Environment::Gnuabi64,
            "gnueabi" => Environment::Gnueabi,
            "gnueabihf" => Environment::Gnueabihf,
            "gnuspe" => Environment::Gnuspe,
            "gnux32" => Environment::Gnux32,
            "musl" => Environment::Musl,
            "musleabi" => Environment::Musleabi,
            "musleabihf" => Environment::Musleabihf,
            "msvc" => Environment::Msvc,
            "uclibc" => Environment::Uclibc,
            "sgx" => Environment::Sgx,
            _ => return Err(()),
        })
    }
}

impl fmt::Display for BinaryFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            BinaryFormat::Unknown => "unknown",
            BinaryFormat::Elf => "elf",
            BinaryFormat::Coff => "coff",
            BinaryFormat::Macho => "macho",
            BinaryFormat::Wasm => "wasm",
        };
        f.write_str(s)
    }
}

impl FromStr for BinaryFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "unknown" => BinaryFormat::Unknown,
            "elf" => BinaryFormat::Elf,
            "coff" => BinaryFormat::Coff,
            "macho" => BinaryFormat::Macho,
            "wasm" => BinaryFormat::Wasm,
            _ => return Err(()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::string::ToString;

    #[test]
    fn rust_targets() {
        // At the time of writing this, these are all the targets emitted by
        // "rustup target list" and "rustc --print target-list".
        let targets = [
            "aarch64-apple-ios",
            "aarch64-fuchsia",
            "aarch64-linux-android",
            "aarch64-pc-windows-msvc",
            "aarch64-unknown-cloudabi",
            "aarch64-unknown-freebsd",
            "aarch64-unknown-hermit",
            "aarch64-unknown-linux-gnu",
            "aarch64-unknown-linux-musl",
            "aarch64-unknown-netbsd",
            "aarch64-unknown-none",
            "aarch64-unknown-openbsd",
            "armebv7r-none-eabi",
            "armebv7r-none-eabihf",
            "arm-linux-androideabi",
            "arm-unknown-linux-gnueabi",
            "arm-unknown-linux-gnueabihf",
            "arm-unknown-linux-musleabi",
            "arm-unknown-linux-musleabihf",
            "armv4t-unknown-linux-gnueabi",
            "armv5te-unknown-linux-gnueabi",
            "armv5te-unknown-linux-musleabi",
            "armv6-unknown-netbsd-eabihf",
            "armv7-apple-ios",
            "armv7-linux-androideabi",
            "armv7r-none-eabi",
            "armv7r-none-eabihf",
            "armv7s-apple-ios",
            "armv7-unknown-cloudabi-eabihf",
            "armv7-unknown-linux-gnueabihf",
            "armv7-unknown-linux-musleabihf",
            "armv7-unknown-netbsd-eabihf",
            "asmjs-unknown-emscripten",
            "i386-apple-ios",
            "i586-pc-windows-msvc",
            "i586-unknown-linux-gnu",
            "i586-unknown-linux-musl",
            "i686-apple-darwin",
            "i686-linux-android",
            "i686-pc-windows-gnu",
            "i686-pc-windows-msvc",
            "i686-unknown-cloudabi",
            "i686-unknown-dragonfly",
            "i686-unknown-freebsd",
            "i686-unknown-haiku",
            "i686-unknown-linux-gnu",
            "i686-unknown-linux-musl",
            "i686-unknown-netbsd",
            "i686-unknown-openbsd",
            "mips64el-unknown-linux-gnuabi64",
            "mips64-unknown-linux-gnuabi64",
            "mipsel-unknown-linux-gnu",
            "mipsel-unknown-linux-musl",
            "mipsel-unknown-linux-uclibc",
            "mips-unknown-linux-gnu",
            "mips-unknown-linux-musl",
            "mips-unknown-linux-uclibc",
            "msp430-none-elf",
            "powerpc64le-unknown-linux-gnu",
            "powerpc64le-unknown-linux-musl",
            "powerpc64-unknown-linux-gnu",
            "powerpc64-unknown-linux-musl",
            "powerpc-unknown-linux-gnu",
            "powerpc-unknown-linux-gnuspe",
            "powerpc-unknown-linux-musl",
            "powerpc-unknown-netbsd",
            "riscv32imac-unknown-none-elf",
            "riscv32imc-unknown-none-elf",
            "s390x-unknown-linux-gnu",
            "sparc64-unknown-linux-gnu",
            "sparc64-unknown-netbsd",
            "sparc-unknown-linux-gnu",
            "sparcv9-sun-solaris",
            "thumbv6m-none-eabi",
            "thumbv7a-pc-windows-msvc",
            "thumbv7em-none-eabi",
            "thumbv7em-none-eabihf",
            "thumbv7m-none-eabi",
            "thumbv7neon-linux-androideabi",
            "thumbv7neon-unknown-linux-gnueabihf",
            "thumbv8m.base-none-eabi",
            "thumbv8m.main-none-eabi",
            "thumbv8m.main-none-eabihf",
            "wasm32-experimental-emscripten",
            "wasm32-unknown-emscripten",
            "wasm32-unknown-unknown",
            "x86_64-apple-darwin",
            "x86_64-apple-ios",
            "x86_64-fortanix-unknown-sgx",
            "x86_64-fuchsia",
            "x86_64-linux-android",
            "x86_64-pc-windows-gnu",
            "x86_64-pc-windows-msvc",
            "x86_64-rumprun-netbsd",
            "x86_64-sun-solaris",
            "x86_64-unknown-bitrig",
            "x86_64-unknown-cloudabi",
            "x86_64-unknown-dragonfly",
            "x86_64-unknown-freebsd",
            "x86_64-unknown-haiku",
            "x86_64-unknown-hermit",
            "x86_64-unknown-l4re-uclibc",
            "x86_64-unknown-linux-gnu",
            "x86_64-unknown-linux-gnux32",
            "x86_64-unknown-linux-musl",
            "x86_64-unknown-netbsd",
            "x86_64-unknown-openbsd",
            "x86_64-unknown-redox",
            "x86_64-unknown-uefi",
        ];

        for target in targets.iter() {
            let t = Triple::from_str(target).expect("can't parse target");
            assert_ne!(t.architecture, Architecture::Unknown);
            assert_eq!(t.to_string(), *target);
        }
    }
}
