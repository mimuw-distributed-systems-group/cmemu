use clap::Args;
use cmemu_lib::common::{Address, BitstringUtils, Word};
use cmemu_lib::engine::Emulator;
use cmemu_lib::paranoid;
use log::{debug, error, info, trace, warn};
use object::FileKind::Elf32;
use object::read::elf::{ElfFile32, ElfSegment32, ProgramHeader};
use object::{
    Architecture, BinaryFormat, Endianness, File, Object, ObjectKind, ObjectSection, ObjectSegment,
    ObjectSymbol, Section, SectionKind, elf,
};
use parse_int::parse;
use std::process;

mod os;
mod symbols;

pub use crate::os::HostingArgs;
pub use symbols::{Symbol, Symbols};

const _RESET_SP_ADDRESS: Address = Address::from_const(0x0000_0000);
const RESET_ISR_ADDRESS: Address = Address::from_const(0x0000_0004);
const NULL_ADDRESS: Address = Address::from_const(0);

#[derive(Args, Default, Debug)]
#[non_exhaustive]
pub struct ElfArgs {
    #[arg(long, group = "load_flag")]
    /// Act as a true system loader that would place segments *only* at their virtual addresses.
    ///
    /// By default, we duplicate such segments by placing them at both virtual and physical addresses.
    /// In practice, for .data, *physical* means Flash and *virtual* means SRAM address.
    /// Barebone embedded applications need to initialize their SRAM.
    ///
    /// Without this flag, hosted applications will see data at addresses, which would be otherwise deemed invalid.
    pub load: bool,

    #[arg(long, group = "load_flag")]
    /// Do not act as a program loader. The executed program manages the loading as on an actual device.
    ///
    /// Instead of loading segments (e.g., to RAM), they will be placed at the 'physical' address (Flash).
    /// This is enabled automatically when we detect no entrypoint in the ELF file.
    ///
    /// In most of the cases, an application should be fine with overloading already loaded memory segments;
    /// however, this may be problematic for some apps with reentrant start.
    pub no_load: bool,

    #[arg(long)]
    /// Override the entrypoint (Reset ISR handler) to the provided symbol.
    ///
    /// Note: if the Reset ISR is null, we will replace it with the entrypoint from ELF headers.
    pub entrypoint: Option<String>,

    #[command(flatten, next_help_heading = "Hosting options")]
    pub hosting_args: Option<HostingArgs>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum LoadMode {
    LoadAtVirtual,
    PlaceAtPhysical,
    Both,
}

#[derive(Debug)]
pub struct ElfLoader<'a> {
    args: &'a ElfArgs,
    elf: Option<File<'a>>,
    elf_data: &'a [u8],
    rom: Option<File<'a>>,
    rom_data: Option<&'a [u8]>,
    load_mode: LoadMode,
}

fn check_elf(elf: &File, kind: &str) {
    assert_eq!(elf.format(), BinaryFormat::Elf, "on {kind}");
    // TODO: support .o?
    assert_eq!(elf.kind(), ObjectKind::Executable, "on {kind}");
    assert_eq!(elf.architecture(), Architecture::Arm, "on {kind}");
    assert_eq!(elf.endianness(), Endianness::Little, "on {kind}");

    // See: https://developer.arm.com/documentation/dui0805/h/fromelf-command-line-options/--decode-build-attributes
    let attrs = elf.section_by_name(".ARM.attributes");
    // .expect("Missing .ARM.attributes - what compiler are you using?");
    if let Some(attrs) = attrs {
        const TAG_CPU_NAME: u8 = 5;
        let cpu_name: &[u8] = attrs
            .data()
            .unwrap()
            .split(|b| *b == 0u8)
            // byte tag + null-delimited ascii
            .find_map(|s| (s.len() > 1 && s[0] == TAG_CPU_NAME).then(|| &s[1..]))
            .expect("CPU_name tag not found");
        assert!(
            cpu_name.eq_ignore_ascii_case(b"cortex-m3") || cpu_name.eq_ignore_ascii_case(b"7-M"),
            "Invalid CPU target {:?} for {}",
            std::str::from_utf8(cpu_name),
            kind
        );
    }
}

fn try_parse<'a>(elf: &'a [u8], part: &str) -> Option<File<'a>> {
    match object::FileKind::parse(elf) {
        Ok(Elf32) => Some(File::parse(elf).unwrap_or_else(|e| {
            error!("Cannot parse {} Elf file: {:?}", part, e);
            process::exit(1)
        })),
        Ok(kind) => {
            error!("Invalid {} file type: {:?}", part, kind);
            process::exit(1);
        }
        Err(_) => {
            info!("{} file format not recognized, assuming raw binary", part);
            None
        }
    }
}

#[must_use]
pub fn get_symbol_address(elf: &File, symbol_name: &str) -> Option<Address> {
    elf.symbols()
        .find(|symbol| symbol.name().unwrap() == symbol_name)
        .map(|s| u32::try_from(s.address()).unwrap().into())
}

impl<'a> ElfLoader<'a> {
    #[must_use]
    pub fn new(elf: &'a [u8], rom: Option<&'a [u8]>, args: &'a ElfArgs) -> Self {
        let mut this = Self {
            args,
            elf: try_parse(elf, "App"),
            elf_data: elf,
            rom: rom.and_then(|r| try_parse(r, "ROM")),
            rom_data: rom,
            load_mode: LoadMode::Both,
        };

        if args.load {
            this.load_mode = LoadMode::LoadAtVirtual;
        } else if args.no_load {
            this.load_mode = LoadMode::PlaceAtPhysical;
        } else if !this.app_has_entrypoint() {
            // Apparently, most embedded toolchains doesn't set the elf entrypoint,
            // but contiki-ng is a notable exception.
            this.load_mode = LoadMode::PlaceAtPhysical;
        }

        if this.load_mode == LoadMode::PlaceAtPhysical {
            warn!("Binary is assumed to perform the loading. Placing segments at physical addr.");
        }

        this
    }

    /// Returns a fragment that would be located at the start of Flash memory
    ///
    /// This is mainly as a filler for the emulator constructor...
    #[must_use]
    pub fn flash_base(&'a self) -> &'a [u8] {
        if let Some(ref elf) = self.elf {
            check_elf(elf, "App");
            // We're going to pass actual data later...
            &[]
        } else {
            self.elf_data
        }
    }

    /// Parsed the main program with object's generic interface.
    #[must_use]
    pub fn program_elf(&self) -> Option<&File<'a>> {
        self.elf.as_ref()
    }

    /// Returns a fragment that would be located at the start of ROM memory
    ///
    /// This is mainly as a filler for the emulator constructor...
    #[must_use]
    pub fn rom_base(&'a self) -> Option<&'a [u8]> {
        if let Some(ref rom) = self.rom {
            check_elf(rom, "ROM");
            // Just a part
            Some(
                rom.section_by_name("INT_VEC_ROM")
                    .expect("Provided elf doesn't seem like a ROM file")
                    .data()
                    .unwrap(),
            )
        } else {
            self.rom_data
        }
    }

    /// Parsed ROM with object's generic interface (i.e., when passed "driverlib.elf").
    #[must_use]
    pub fn rom_elf(&self) -> Option<&File<'a>> {
        self.rom.as_ref()
    }

    /// Load program and ROM segments into the binary: our main entrypoint
    pub fn load(&'a self, emulator: &mut Emulator) {
        let mut symbols: Symbols<'_> = cc2650_constants::iter_known_registers().collect();
        if let Some(ref elf) = self.elf {
            Self::load_segments_of(self, emulator, elf, self.elf_data, self.load_mode);

            if let Some(ref emulator_entrypoint) = self.args.entrypoint {
                let address = get_symbol_address(elf, emulator_entrypoint)
                    .expect("Specified entrypoint not found in the elf binary");
                Self::override_entrypoint(emulator, address);
            } else if Self::get_entrypoint(emulator) == NULL_ADDRESS
                && self.get_elf_entrypoint().is_some()
            {
                let elf_entrypoint = self.get_elf_entrypoint().unwrap();
                Self::override_entrypoint(emulator, elf_entrypoint.into());
            } else {
                assert_ne!(
                    Self::get_entrypoint(emulator),
                    NULL_ADDRESS,
                    "Invalid Reset ISR address (entrypoint). Try overriding it with --entrypoint option."
                );
            }
            // TODO: detect that there is no vector-table on address 0  + set SP to _estack

            symbols.extend(elf.symbols());
        } else if let Some(addr) = self
            .args
            .entrypoint
            .as_ref()
            .and_then(|p| parse::<u32>(p).ok())
        {
            let address = Address::from_const(addr);
            Self::override_entrypoint(emulator, address);
        } else if self.args.entrypoint.is_some() {
            panic!("When passing a Flash image (not elf), only numeric entrypoints are supported.")
        }

        if let Some(ref rom) = self.rom {
            // Note: ROM has parts in SRAM and GPRAM for the bootloader and we should never load them!
            // Fortunately, they are not in the segments!
            Self::load_segments_of(
                self,
                emulator,
                rom,
                self.rom_data.unwrap(),
                LoadMode::LoadAtVirtual,
            );
            // We need to filter out these symbols for that bootloader too!
            symbols.extend(rom.symbols().filter(|s| {
                Address::from_const(u32::try_from(s.address()).unwrap())
                    .is_in_range(&cc2650_constants::BROM::ADDR_SPACE)
            }));
        }

        // Symbols reference the File now, we need to move them to heap.
        emulator.set_symbols_service(Some(Box::new(symbols.into_unbound())));

        if let Some(ref hosting) = self.args.hosting_args {
            hosting.process_args(emulator);
        }
    }

    fn override_entrypoint(emulator: &mut Emulator, emulator_entrypoint: Address) {
        info!("Switching entrypoint to: {:?}", emulator_entrypoint);
        assert!(
            Word::from(emulator_entrypoint).get_bit(0),
            "Entrypoint has to have THUMB bit set."
        );
        emulator.set_nonstandard_entrypoint(Some(emulator_entrypoint));
    }

    /// Gets the entrypoint from reset vector (aka `ResetISR`)
    fn get_entrypoint(emulator: &Emulator) -> Address {
        let mut entrypoint = 0u32.to_le_bytes();
        emulator
            .read_memory(RESET_ISR_ADDRESS, &mut entrypoint)
            .unwrap();
        Address::from_const(u32::from_le_bytes(entrypoint))
    }

    /// Gets the entrypoint from elf headers
    fn get_elf_entrypoint(&'a self) -> Option<u32> {
        self.elf.as_ref().map(|e| u32::try_from(e.entry()).unwrap())
    }

    fn app_has_entrypoint(&'a self) -> bool {
        self.get_elf_entrypoint().is_some_and(|e| e != 0)
    }

    fn load_segments_of(
        &'a self,
        emulator: &mut Emulator,
        elf: &File,
        elf_raw: &'a [u8],
        load_mode: LoadMode,
    ) {
        assert_eq!(elf.format(), BinaryFormat::Elf);
        let elffile = ElfFile32::parse(elf_raw).unwrap();
        for segment in elffile.segments() {
            let endian = segment.elf_file().endian();
            let phdr = segment.elf_program_header();
            let same_addr = phdr.p_paddr(endian) == phdr.p_vaddr(endian);

            if load_mode == LoadMode::PlaceAtPhysical || (load_mode == LoadMode::Both && !same_addr)
            {
                Self::load_elf_segment(self, emulator, &segment, true);
            }
            if load_mode == LoadMode::LoadAtVirtual || load_mode == LoadMode::Both {
                Self::load_elf_segment(self, emulator, &segment, false);
            }
        }
    }

    #[allow(clippy::unused_self)]
    fn load_elf_segment(&'a self, emulator: &mut Emulator, segment: &ElfSegment32, physical: bool) {
        let endianness = segment.elf_file().endian();
        let phdr = segment.elf_program_header();
        let ptype = phdr.p_type(endianness);
        let mut skip = match ptype {
            elf::PT_LOAD => false,
            elf::PT_ARM_EXIDX | elf::PT_NOTE | elf::PT_GNU_STACK => true,
            elf::PT_DYNAMIC => unimplemented!("Dynamic segments not implemented"),
            elf::PT_INTERP => unimplemented!("Program cannot be interpreted"),
            unknown => {
                paranoid!(warn, "Unsupported segment type {}", unknown);
                true
            }
        };
        // Segments have Physical and Virtual destination addresses
        // In our case, Virtual is the one during actual runtime,
        // and Physical is the data storage in Flash for on-device loading.
        let paddr = phdr.p_paddr(endianness);
        let vaddr = phdr.p_vaddr(endianness);
        let filesz = phdr.p_filesz(endianness);
        let memsz = phdr.p_memsz(endianness);
        let start_address = Address::from_const(if physical { paddr } else { vaddr });
        // Segments should be zero padded if size is not matching,
        // but we should not extend this for physical addresses (as that would place zeros from .bss there)
        let zero_pad = memsz != filesz && !physical;
        skip |= filesz == 0 && !zero_pad;
        info!(
            "{} elf segment to 0x{start_address:?} {} TYPE: {ptype:x} OFF: {off:x} VADDR: {vaddr:x} PADDR: {paddr:x} FILESZ: {filesz:x} MEMSZ: {memsz:x}",
            if skip { "Skipping" } else { "Loading" },
            if physical { "(nested loader)" } else { "" },
            off = phdr.p_offset(endianness),
        );

        if skip {
            return;
        }

        if zero_pad {
            let zeros = vec![0u8; filesz as usize];
            emulator
                .write_memory(start_address, &zeros)
                .expect("wrong segment addr");
        }

        emulator
            .write_memory(start_address, segment.data().unwrap())
            .expect("Loading failed");
    }

    #[allow(dead_code)] // unused for now, keep for a while if we need to load ET_REL files
    fn load_sections_of(emulator: &mut Emulator, elf: &File) {
        for section in elf.sections() {
            match section.kind() {
                // should we load text again? may there be more than just .text?
                SectionKind::Text | SectionKind::Data | SectionKind::ReadOnlyData => {
                    Self::load_section(emulator, &section);
                }
                SectionKind::UninitializedData => {
                    debug!(
                        "Uninitialized elf section {:?} at {:x} size: {:x}",
                        section.name(),
                        section.address(),
                        section.size()
                    );
                }
                SectionKind::Other if section.name().unwrap().starts_with(".gpram") => {
                    Self::load_section(emulator, &section);
                }
                SectionKind::Elf(
                    elf::SHT_INIT_ARRAY
                    | elf::SHT_FINI_ARRAY
                    | elf::SHT_PREINIT_ARRAY
                    | elf::SHT_ARM_EXIDX,
                ) => {
                    // if section.flags().into() & u64::from(elf::SHF_ALLOC) != 0
                    Self::load_section(emulator, &section);
                }
                _ => {
                    trace!("Skipping section {:?}", section);
                }
            }
        }
    }

    #[allow(dead_code)] // unused for now, keep for a while if we need to load ET_REL files
    fn load_section(emulator: &mut Emulator, section: &Section) {
        info!(
            "Loading elf section {:?} at {:x} size: {:x}",
            section.name(),
            section.address(),
            section.size()
        );
        emulator
            .write_memory(
                Address::from_const(u32::try_from(section.address()).unwrap()),
                section.data().unwrap(),
            )
            .expect("Loading failed");
    }
}
