/// The size of a type.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub struct Size {
    /// The size in bits of this type.
    bits: u8,
    /// The number of bits in a byte.
    char_bit: u8,
}

impl Size {
    /// Return the number of bits this `Size` represents.
    pub fn bits(self) -> u8 {
        self.bits
    }

    /// Return the number of bytes in a size.
    ///
    /// Depending on the target, a byte may not necessarily be 8 bits.
    pub fn bytes(self) -> u8 {
        let bytes = self.bits / self.char_bit;
        // make sure that self.bits is a multiple of char_bit
        assert_eq!(bytes * self.char_bit, self.bits);
        bytes
    }
    /// Create a new `Size`, assuming that `char_bit` is 8.
    fn with_bits(bits: u8) -> Self {
        Self {
            bits,
            char_bit: 8,
        }
    }
}

/// The C data model used on a target.
///
/// See also https://en.cppreference.com/w/c/language/arithmetic_types
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CDataModel {
    /// The data model used most commonly on Win16
    LP32,
    /// The data model used most commonly on Win32 and 32-bit Unix systems
    ILP32,
    /// The data model used most commonly on Win64
    LLP64,
    /// The data model used most commonly on 64-bit Unix systems
    LP64,
    /// A rare data model used on early 64-bit Unix systems
    ILP64,
}

impl CDataModel {
    /// The width of a pointer (in the default address space).
    pub fn pointer_width(&self) -> Size {
        match self {
            CDataModel::LP32 | CDataModel::ILP32 => Size::with_bits(32),
            CDataModel::LLP64 | CDataModel::LP64 | CDataModel::ILP64 => Size::with_bits(64),
        }
    }
    /// The size of a C `short`. This is required to be at least 16 bits.
    pub fn short_size(&self) -> Size {
        match self {
            CDataModel::LP32 | CDataModel::ILP32 | CDataModel::LLP64 | CDataModel::LP64 | CDataModel::ILP64 => Size::with_bits(16),
        }
    }
    /// The size of a C `int`. This is required to be at least 16 bits.
    pub fn int_size(&self) -> Size {
        match self {
            CDataModel::LP32 => Size::with_bits(16),
            CDataModel::ILP32 | CDataModel::LLP64 | CDataModel::LP64 | CDataModel::ILP64 => Size::with_bits(32),
        }
    }
    /// The size of a C `long`. This is required to be at least 32 bits.
    pub fn long_size(&self) -> Size {
        match self {
            CDataModel::LP32 | CDataModel::ILP32 | CDataModel::LLP64 | CDataModel::ILP64 => Size::with_bits(32),
            CDataModel::LP64 => Size::with_bits(64),
        }
    }
    /// The size of a C `long long`. This is required (in C99+) to be at least 64 bits.
    pub fn long_long_size(&self) -> Size {
        match self {
            CDataModel::LP32 | CDataModel::ILP32 | CDataModel::LLP64 | CDataModel::ILP64 | CDataModel::LP64 => Size::with_bits(64),
        }
    }
    /// The size of a C `float`.
    pub fn float_size(&self) -> Size {
        // TODO: this is probably wrong on at least one architecture
        Size::with_bits(32)
    }
    /// The size of a C `double`.
    pub fn double_size(&self) -> Size {
        // TODO: this is probably wrong on at least one architecture
        Size::with_bits(64)
    }
}
