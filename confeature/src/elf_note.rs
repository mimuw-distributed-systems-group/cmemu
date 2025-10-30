#[repr(C, align(4))]
struct AlignedCString<const N: usize>([u8; N]);

/// A properly formatted entry in an ELF NOTE section.
///
/// It may be used directly as shown below, but the recommended way is using the [`put_note`] macro.
/// ```
/// #[used]
/// #[unsafe(link_section = ".note.custom")]
/// static VAR8: ElfNoteEntry<10, { yaml.len() }> = ElfNoteEntry::new(0x42, *b"CONFEATURE", *yaml);
/// ```
#[allow(
    missing_debug_implementations,
    reason = "This is not to be constructed."
)]
#[repr(C, align(4))]
pub struct ElfNoteEntry<const V: usize, const N: usize> {
    // Note: The ints are in native endian.
    namesz: u32,
    descsz: u32,
    /// Tag is vendor-defined
    note_type: u32,
    // Looking at readelf, NULL bytes are only for padding.
    // A len(n) divisible by 4 should not start another word
    // just for the NULL BYTE
    /// Vendor
    name: AlignedCString<V>,
    desc: AlignedCString<N>,
}

impl<const V: usize, const N: usize> ElfNoteEntry<V, N> {
    #[must_use]
    #[allow(
        clippy::cast_possible_truncation,
        reason = "The spec says the field is 32 bit."
    )]
    /// Creates a new [`ElfNoteEntry`]. Do not use this method directly.
    ///
    /// Use the [`put_note!`] macro instead.
    pub const fn new(note_type: u32, name: [u8; V], desc: [u8; N]) -> Self {
        Self {
            namesz: V as u32,
            descsz: N as u32,
            note_type,
            name: AlignedCString(name),
            desc: AlignedCString(desc),
        }
    }
}

/// A macro to create an entry in an ELF NOTE section
///
/// First, we need a variable name for the compiler to place the bytes in the output.
/// Example:
/// ```
/// put_note!(random_symbol, b"This is a description");
/// put_note! {
///     var: _self,
///     section: ".note.source",
///     vendor: b"CONFEATURE",
///     type: 0x42,
///     note: include_bytes!("confeature.yml"),
/// }
/// ```
#[macro_export]
macro_rules! put_note {
    ($var:ident, $note:literal) => {
        put_note! {
            var: $var,
            section: ".note.confeature",
            vendor: b"CONFEATURE",
            type: 0x1337,
            note: $note,
        }
    };
    (
        var: $var:ident,
        section: $section:literal,
        vendor: $vendor:expr,
        type: $type:expr,
        note: $note:expr $(,)?
    ) => {
        #[used]
        #[unsafe(link_section = $section)]
        static $var: $crate::elf_note::ElfNoteEntry<{ $vendor.len() }, { $note.len() }> =
            $crate::elf_note::ElfNoteEntry::new($type, *$vendor, *$note);
    };
}

// Export the macro in this namespace
pub use put_note;
