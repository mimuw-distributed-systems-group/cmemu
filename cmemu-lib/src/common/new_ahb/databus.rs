/// A struct carrying sized data
use crate::Bitstring;
use crate::common::new_ahb::{MasterToSlaveWires, Size, SlaveToMasterWires};
use crate::common::{BitstringUtils, Word};
use crate::utils::IfExpr;
use cmemu_common::Address;
use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;

/// Stores data of specified width in a type-safe manner
///
/// Instead of directly simulating the active wires of a full-width line, as in [ARM-AHB-Lite] 6 Data Buses,
/// we store the size as a variant of this type. It is, so the data cannot be accidentally misused,
/// as the decoding depends both on size and the address alignment.
/// Instead, the `DataBus` can be used as a standalone data holder.
///
/// Many parts of the memory subsystem in CMEmu are specialized to interfaces using the `DataBus`,
/// where a generic type-carrying argument may go, such as [`SlaveToMasterWires`].
// NOTE: using uX instead of [u8; X/8] has a potential to over-assume little-endianness
#[derive(PartialEq, Eq, Clone, Default)]
#[non_exhaustive]
pub enum DataBus {
    /// Represents a high-impedance lines state when not carrying any data.
    #[default]
    HighZ,
    Byte(u8),
    Short(u16),
    Word(u32),
    Quad(u64),
    // TODO(cm4): fix this on memory level instead of bloating up this enum
    #[cfg(feature = "soc-cc2652")]
    FourWord(u128),
}

const DATABUS_MAX_BYTES: usize = if cfg!(feature = "soc-cc2652") { 16 } else { 8 };

#[allow(dead_code)]
pub(crate) type S2MBus = SlaveToMasterWires<DataBus>;
#[allow(dead_code)]
pub(crate) type M2SBus = MasterToSlaveWires<DataBus>;

impl Debug for DataBus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HighZ => f.write_str("HighZ"),
            Self::Byte(b) => write!(f, "Byte({b:#x})"),
            Self::Short(s) => write!(f, "Short({s:#x})"),
            Self::Word(w) => write!(f, "Word({w:#x})"),
            Self::Quad(q) => write!(f, "Quad({q:#x})"),
            #[cfg(feature = "soc-cc2652")]
            Self::FourWord(fw) => write!(f, "FourWord({fw:#x})"),
        }
    }
}

impl Display for DataBus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HighZ => f.write_str("Z"),
            Self::Byte(b) => write!(f, "{b:02x}"),
            Self::Short(s) => write!(f, "{s:04x}"),
            Self::Word(w) => write!(f, "{w:08x}"),
            Self::Quad(q) => write!(f, "{q:016x}"),
            #[cfg(feature = "soc-cc2652")]
            Self::FourWord(fw) => write!(f, "{fw:032x}"),
        }
    }
}

impl DataBus {
    /// Get a u32 if the value would fit, or panic.
    pub(crate) fn raw(self) -> u32 {
        use DataBus::*;
        match self {
            Byte(x) => x.into(),
            Short(x) => x.into(),
            Word(x) => x,
            _ => panic!("Bus too wide to cast into u32"),
        }
    }

    /// Check if any data is present, that is we're not `DataBus::HighZ`.
    pub fn is_present(&self) -> bool {
        !matches!(self, DataBus::HighZ)
    }

    /// Convert a reference `HighZ` to `None`, or just wrap into `Some`.
    #[must_use]
    pub fn as_option(&self) -> Option<&Self> {
        if let Self::HighZ = self {
            None
        } else {
            Some(self)
        }
    }

    /// Convert `HighZ` into `None`, or just wrap into `Some`.
    #[must_use]
    pub fn into_option(self) -> Option<Self> {
        self.is_present().ife(Some(self), None)
    }
}

impl From<DataBus> for Size {
    fn from(db: DataBus) -> Self {
        db.size()
    }
}

impl From<DataBus> for u32 {
    // TODO: from should now panic!
    fn from(db: DataBus) -> Self {
        db.raw()
    }
}

impl From<DataBus> for [u8; 4] {
    fn from(d: DataBus) -> Self {
        // TODO: from should now panic!
        let raw = match d {
            DataBus::Byte(b) => u32::from(b),
            DataBus::Short(s) => u32::from(s),
            DataBus::Word(w) => w,
            _ => unreachable!("[u8; 4] handle only up to word"),
        };
        raw.to_le_bytes()
    }
}

impl From<DataBus> for [u8; 8] {
    fn from(d: DataBus) -> Self {
        // TODO: from should now panic!
        let raw = match d {
            DataBus::HighZ => 0u64,
            DataBus::Byte(b) => u64::from(b),
            DataBus::Short(s) => u64::from(s),
            DataBus::Word(w) => u64::from(w),
            DataBus::Quad(q) => q,
            #[cfg(feature = "soc-cc2652")]
            _ => unreachable!("[u8; 8] handle only up to quad"),
        };
        raw.to_le_bytes()
    }
}

impl From<u32> for DataBus {
    fn from(x: u32) -> Self {
        DataBus::Word(x)
    }
}
impl From<u16> for DataBus {
    fn from(x: u16) -> Self {
        DataBus::Short(x)
    }
}
impl From<u8> for DataBus {
    fn from(x: u8) -> Self {
        DataBus::Byte(x)
    }
}

impl From<[u8; 4]> for DataBus {
    fn from(x: [u8; 4]) -> Self {
        DataBus::Word(u32::from_le_bytes(x))
    }
}

impl From<[u8; 8]> for DataBus {
    fn from(x: [u8; 8]) -> Self {
        DataBus::Quad(u64::from_le_bytes(x))
    }
}

type BitWord = Word;
type BitShort = Bitstring![16];
type BitByte = Bitstring![8];

impl From<BitWord> for DataBus {
    fn from(w: BitWord) -> Self {
        DataBus::Word(w.into())
    }
}

impl From<BitShort> for DataBus {
    fn from(w: BitShort) -> Self {
        DataBus::Short(w.into())
    }
}

impl From<BitByte> for DataBus {
    fn from(w: BitByte) -> Self {
        DataBus::Byte(w.into())
    }
}

#[derive(Error, Debug, Clone, Copy)]
pub enum DataConversionError {
    #[error("Value cannot fit the destination")]
    OverflowError,
    #[error("Not sure whether to 0-extend or sign-extend value.")]
    UnderflowError,
}

impl TryFrom<DataBus> for BitWord {
    type Error = DataConversionError;

    fn try_from(value: DataBus) -> Result<Self, Self::Error> {
        if let DataBus::Word(w) = value {
            Ok(Self::from(w))
        } else if Self::bits_width() > value.size().bits() {
            Err(DataConversionError::UnderflowError)
        } else {
            Err(DataConversionError::OverflowError)
        }
    }
}

impl TryFrom<DataBus> for BitShort {
    type Error = DataConversionError;

    fn try_from(value: DataBus) -> Result<Self, Self::Error> {
        if let DataBus::Short(w) = value {
            Ok(Self::from(w))
        } else if Self::bits_width() > value.size().bits() {
            Err(DataConversionError::UnderflowError)
        } else {
            Err(DataConversionError::OverflowError)
        }
    }
}

impl TryFrom<DataBus> for BitByte {
    type Error = DataConversionError;

    fn try_from(value: DataBus) -> Result<Self, Self::Error> {
        if let DataBus::Byte(w) = value {
            Ok(Self::from(w))
        } else if Self::bits_width() > value.size().bits() {
            Err(DataConversionError::UnderflowError)
        } else {
            Err(DataConversionError::OverflowError)
        }
    }
}

// Helpers for operating with Word
impl DataBus {
    /// Assuming the `w` word is aligned (`addr&!3`), extract `size`-ed data from the address.
    ///
    /// # Panics
    ///
    /// Panics if the requested bytes range would wrap to the next word.
    #[inline(always)]
    #[must_use]
    pub fn extract_from_word(w: Word, addr: Address, size: Size) -> DataBus {
        DataBus::from(w).extract_from_aligned(addr, size)
    }

    /// Clip a [`Word`] to a given size and return as `DataBus` variant
    #[inline]
    #[must_use]
    #[allow(clippy::cast_possible_truncation, reason = "We want to trunc")]
    pub fn clip_word(w: Word, size: Size) -> DataBus {
        match size {
            Size::Byte => DataBus::Byte(u32::from(w) as u8),
            Size::Halfword => DataBus::Short(u32::from(w) as u16),
            Size::Word => DataBus::Word(w.into()),
            _ => unreachable!(),
        }
    }

    /// Assuming the `w` word is aligned (`addr&!3`), modify `data` at the offset specified
    /// by the address.
    ///
    /// # Panics
    ///
    /// Panics if the requested bytes range would wrap to the next word.
    #[inline(always)]
    #[must_use]
    pub fn emplace_in_word(w: Word, addr: Address, data: Self) -> Word {
        DataBus::from(w)
            .emplace_in_aligned(addr, data)
            .try_into()
            .unwrap()
    }

    /// Sign extend carried data into a [`Word`]
    ///
    /// # Panics
    ///
    /// Panics if the data are empty or wider than `DataBus::Word`
    #[inline]
    #[must_use]
    pub fn sign_extend_into_word(self) -> Word {
        match self {
            DataBus::HighZ => panic!("Trying to extend non-existent data"),
            DataBus::Byte(b) => BitByte::from(b).sign_extend(),
            DataBus::Short(s) => BitShort::from(s).sign_extend(),
            DataBus::Word(w) => BitWord::from(w).sign_extend(),
            _ => panic!("Wide transfer cannot be converted to word"),
        }
    }

    /// Zero extend carried data into a [`Word`]
    ///
    /// # Panics
    ///
    /// Panics if the data are empty or wider than `DataBus::Word`
    #[inline]
    #[must_use]
    pub fn zero_extend_into_word(self) -> Word {
        match self {
            DataBus::HighZ => panic!("Trying to extend non-existent data"),
            DataBus::Byte(b) => BitByte::from(b).zero_extend(),
            DataBus::Short(s) => BitShort::from(s).zero_extend(),
            DataBus::Word(w) => BitWord::from(w).zero_extend(),
            _ => panic!("Wide transfer cannot be converted to word"),
        }
    }

    /// Assuming an aligned word (`addr&!3`), create a bit mask of `size` bytes
    /// at the offset specified by the address.
    #[inline(always)]
    #[must_use]
    pub fn build_word_mask(addr: Address, size: Size) -> Word {
        Self::emplace_in_word(
            Word::from_const(0),
            addr,
            Self::clip_word(Word::from_const(!0), size),
        )
    }

    /// Convert `DataBus::Word` into [`Word`]
    ///
    /// # Panics
    /// Panics if not `DataBus::Word`.
    pub fn unwrap_word(self) -> Word {
        Word::try_from(self).unwrap()
    }
}

impl DataBus {
    /// Get corresponding [`Size`] of the data represented
    #[must_use]
    pub fn size(&self) -> Size {
        match self {
            DataBus::HighZ => panic!("Getting size of non-transfer"),
            DataBus::Byte(_) => Size::Byte,
            DataBus::Short(_) => Size::Halfword,
            DataBus::Word(_) => Size::Word,
            DataBus::Quad(_) => Size::Doubleword,
            #[cfg(feature = "soc-cc2652")]
            Self::FourWord(_) => Size::FourWord,
        }
    }

    /// Returns an array and a size (in bytes) to make a slice that would fit `size`.
    #[must_use]
    #[inline]
    pub fn make_slice(size: impl Into<Size>) -> ([u8; DATABUS_MAX_BYTES], usize) {
        let size: Size = size.into();
        debug_assert!(size.bytes() <= DATABUS_MAX_BYTES);
        ([0u8; DATABUS_MAX_BYTES], size.bytes())
    }

    /// Dynamically create a `DataBus` variant based on the `slice`.
    ///
    /// # Panics
    /// Panics if `slice.len()` is cannot be encoded as `DataBus`
    #[must_use]
    #[inline(always)]
    pub fn from_slice(slice: &[u8]) -> Self {
        match slice.len() {
            0 => DataBus::HighZ,
            1 => DataBus::Byte(slice[0]),
            2 => DataBus::Short(u16::from_le_bytes(slice.try_into().unwrap())),
            4 => DataBus::Word(u32::from_le_bytes(slice.try_into().unwrap())),
            8 => DataBus::Quad(u64::from_le_bytes(slice.try_into().unwrap())),
            #[cfg(feature = "soc-cc2652")]
            16 => DataBus::FourWord(u128::from_le_bytes(slice.try_into().unwrap())),
            l => unreachable!("Invalid slice size {}", l),
        }
    }

    /// Run a slice-accepting closure on the underlying data
    #[inline(always)]
    pub fn map_into_slice<F, R>(self, f: F) -> R
    where
        F: FnOnce(&[u8]) -> R,
    {
        match self {
            DataBus::HighZ => f(&[0u8; 0]),
            DataBus::Byte(d) => f(&d.to_le_bytes()),
            DataBus::Short(d) => f(&d.to_le_bytes()),
            DataBus::Word(d) => f(&d.to_le_bytes()),
            DataBus::Quad(d) => f(&d.to_le_bytes()),
            #[cfg(feature = "soc-cc2652")]
            Self::FourWord(d) => f(&d.to_le_bytes()),
        }
    }

    /// Write the held data into a slice of a matching size
    ///
    /// # Panics
    /// Panics if `slice.len()` is different than `self.size().bytes()`
    #[inline(always)]
    pub fn write_into_slice(self, slice: &mut [u8]) {
        assert_eq!(slice.len(), self.size().bytes());
        self.map_into_slice(|x| slice.copy_from_slice(x));
    }

    /// Assuming `self` represents aligned data (`addr&!self.size`),
    /// extract `size`-ed data from the address.
    ///
    /// # Panics
    ///
    /// Panics if the requested bytes range would wrap to past the provided data.
    #[must_use]
    #[inline(always)]
    pub fn extract_from_aligned(self, addr: Address, size: Size) -> Self {
        let this_size = self.size();
        let offset = this_size.offset_from_aligned(addr);
        debug_assert!(
            offset + size.bytes() <= self.size().bytes(),
            "Cannot extract past provided data: {offset}+{size} > {self:?}"
        );

        let (mut slice, range) = Self::make_slice(this_size);
        self.write_into_slice(&mut slice[..range]);
        let ret = DataBus::from_slice(&slice[offset..offset + size.bytes()]);

        debug_assert!(ret.size() == size);
        ret
    }

    /// Assuming `self` represents aligned data (`addr&!self.size`),
    /// modify sub-`data` at the offset specified by the address.
    ///
    /// # Panics
    ///
    /// Panics if the requested bytes range would wrap to past the provided data.
    #[inline(always)]
    pub fn emplace_in_aligned(self, addr: Address, data: Self) -> Self {
        let this_size = self.size();
        let size = data.size();
        let offset = this_size.offset_from_aligned(addr);
        debug_assert!(
            offset + size.bytes() <= this_size.bytes(),
            "Cannot emplace past provided data: {offset}+{size} > {self:?}",
        );

        let (mut slice, range) = Self::make_slice(this_size);
        self.write_into_slice(&mut slice[..range]);
        data.write_into_slice(&mut slice[offset..offset + size.bytes()]);
        let ret = DataBus::from_slice(&slice[..range]);

        debug_assert!(ret.size() == this_size);
        ret
    }
}

impl S2MBus {
    /// Check if the wires carry the empty state
    pub fn is_inert(&self) -> bool {
        self.meta.is_done() && !self.data.is_present()
    }
}

impl M2SBus {
    /// Check if the wires carry the empty state.
    /// The previous state may make this one non-inert (e.g. reads)
    pub fn is_inert(&self) -> bool {
        self.addr_phase.meta.is_idle() && !self.data_phase.data.is_present()
    }
}
