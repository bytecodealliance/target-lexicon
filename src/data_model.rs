/// The size in bits of a type.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Size {
    U16,
    U32,
    U64,
}

impl Size {
    /// Return the number of bits in a size.
    pub fn bits(self) -> u8 {
        match self {
            Size::U16 => 16,
            Size::U32 => 32,
            Size::U64 => 64,
        }
    }

    /// Return the number of bytes in a size.
    ///
    /// For these purposes, there are 8 bits in a byte.
    pub fn bytes(self) -> u8 {
        match self {
            Size::U16 => 2,
            Size::U32 => 4,
            Size::U64 => 8,
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
            CDataModel::LP32 | CDataModel::ILP32 => Size::U32,
            CDataModel::LLP64 | CDataModel::LP64 | CDataModel::ILP64 => Size::U64,
        }
    }
    /// The size of a C `short`. This is required to be at least 16 bits.
    pub fn short_size(&self) -> Size {
        match self {
            CDataModel::LP32 | CDataModel::ILP32 | CDataModel::LLP64 | CDataModel::LP64 | CDataModel::ILP64 => Size::U16,
        }
    }
    /// The size of a C `int`. This is required to be at least 16 bits.
    pub fn int_size(&self) -> Size {
        match self {
            CDataModel::LP32 => Size::U16,
            CDataModel::ILP32 | CDataModel::LLP64 | CDataModel::LP64 | CDataModel::ILP64 => Size::U32,
        }
    }
    /// The size of a C `long`. This is required to be at least 32 bits.
    pub fn long_size(&self) -> Size {
        match self {
            CDataModel::LP32 | CDataModel::ILP32 | CDataModel::LLP64 | CDataModel::ILP64 => Size::U32,
            CDataModel::LP64 => Size::U64,
        }
    }
    /// The size of a C `long long`. This is required (in C99+) to be at least 64 bits.
    pub fn long_long_size(&self) -> Size {
        match self {
            CDataModel::LP32 | CDataModel::ILP32 | CDataModel::LLP64 | CDataModel::ILP64 | CDataModel::LP64 => Size::U64,
        }
    }
    /// The size of a C `float`.
    pub fn float_size(&self) -> Size {
        // TODO: this is probably wrong on at least one architecture
        Size::U32
    }
    /// The size of a C `double`.
    pub fn double_size(&self) -> Size {
        // TODO: this is probably wrong on at least one architecture
        Size::U64
    }
}
