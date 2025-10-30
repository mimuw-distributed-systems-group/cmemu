#![allow(clippy::cast_possible_truncation)]

use cmemu_elf_loader::{ElfArgs, ElfLoader};
use cmemu_lib::common::Address;
use cmemu_lib::engine::Emulator;
use object::{Object, ObjectSection, SectionKind};

fn test_elf_fixture() -> Vec<u8> {
    std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test.elf")).unwrap()
}

fn read_word(emulator: &Emulator, address: impl Into<Address>) -> [u8; 4] {
    let mut buf = [0u8; 4];
    emulator.read_memory(address.into(), &mut buf).unwrap();
    buf
}

fn name_an_address(emulator: &Emulator, address: impl Into<Address>) -> String {
    emulator.name_an_address(address.into()).to_string()
}

fn make_emulator(elf: &ElfLoader) -> Emulator {
    let mut emulator = Emulator::new(elf.flash_base(), elf.rom_base());
    elf.load(&mut emulator);
    emulator
}

#[test]
fn everything_loaded() {
    // This is a large test as there is no easy way to rerun it without repeating the load
    let flash_mem = test_elf_fixture();
    let rom_mem = None;
    let elf_params = ElfArgs::default();
    let elf = ElfLoader::new(&flash_mem, rom_mem, &elf_params);
    let emulator = make_emulator(&elf);

    // Then...
    // address naming works
    assert_eq!(
        name_an_address(&emulator, cc2650_constants::SRAM::ADDR_SPACE.end),
        "_esram"
    );
    assert_eq!(
        name_an_address(&emulator, cc2650_constants::GPRAM::ADDR.offset(1)),
        "gp_data+1"
    );

    // reset vector looks alright
    let reservation_amount = -256_i32;
    let stack_init = cc2650_constants::FLASHMEM::ADDR.offset(0);
    assert_eq!(
        u32::from_le_bytes(read_word(&emulator, stack_init)),
        cc2650_constants::SRAM::ADDR_SPACE
            .end
            .offset(reservation_amount.cast_unsigned())
            .to_const()
    );

    // GPRAM is loaded
    let gp_data_addr =
        cmemu_elf_loader::get_symbol_address(elf.program_elf().unwrap(), "gp_data").unwrap();
    assert!(gp_data_addr.is_in_range(&cc2650_constants::GPRAM::ADDR_SPACE));
    assert_eq!(read_word(&emulator, gp_data_addr), [1, 7, 13, 33]);
    // and still present in Flash!
    let gpram_in_flash_addr =
        cmemu_elf_loader::get_symbol_address(elf.program_elf().unwrap(), "_lgpram").unwrap();
    assert!(gpram_in_flash_addr.is_in_range(&cc2650_constants::FLASHMEM::ADDR_SPACE));
    assert_eq!(read_word(&emulator, gpram_in_flash_addr), [1, 7, 13, 33]);
}

#[test]
fn not_loading() {
    // This is a large test as there is no easy way to rerun it without repeating the load
    let flash_mem = test_elf_fixture();
    let rom_mem = None;
    let mut elf_params = ElfArgs::default();
    elf_params.no_load = true;
    let elf = ElfLoader::new(&flash_mem, rom_mem, &elf_params);
    let emulator = make_emulator(&elf);

    // Then...

    // GPRAM is NOT loaded
    let gp_data_addr =
        cmemu_elf_loader::get_symbol_address(elf.program_elf().unwrap(), "gp_data").unwrap();
    assert!(gp_data_addr.is_in_range(&cc2650_constants::GPRAM::ADDR_SPACE));
    assert_eq!(name_an_address(&emulator, gp_data_addr), "gp_data");
    assert_eq!(read_word(&emulator, gp_data_addr), [0, 0, 0, 0]);

    // but it is present in Flash!
    let gpram_in_flash_addr =
        cmemu_elf_loader::get_symbol_address(elf.program_elf().unwrap(), "_lgpram").unwrap();
    assert!(gpram_in_flash_addr.is_in_range(&cc2650_constants::FLASHMEM::ADDR_SPACE));
    assert_eq!(read_word(&emulator, gpram_in_flash_addr), [1, 7, 13, 33]);
}

#[test]
fn only_loading() {
    // This is a large test as there is no easy way to rerun it without repeating the load
    let flash_mem = test_elf_fixture();
    let rom_mem = None;
    let mut elf_params = ElfArgs::default();
    elf_params.load = true;
    let elf = ElfLoader::new(&flash_mem, rom_mem, &elf_params);
    let emulator = make_emulator(&elf);

    // Then...

    // GPRAM is loaded
    let gp_data_addr =
        cmemu_elf_loader::get_symbol_address(elf.program_elf().unwrap(), "gp_data").unwrap();
    assert!(gp_data_addr.is_in_range(&cc2650_constants::GPRAM::ADDR_SPACE));
    assert_eq!(name_an_address(&emulator, gp_data_addr), "gp_data");
    assert_eq!(read_word(&emulator, gp_data_addr), [1, 7, 13, 33]);

    // but it is NOT present in Flash!
    let gpram_in_flash_addr =
        cmemu_elf_loader::get_symbol_address(elf.program_elf().unwrap(), "_lgpram").unwrap();
    assert!(gpram_in_flash_addr.is_in_range(&cc2650_constants::FLASHMEM::ADDR_SPACE));
    assert_eq!(read_word(&emulator, gpram_in_flash_addr), [0, 0, 0, 0]);
}

#[test]
#[cfg_attr(not(cmemu_has_rom = "driverlib"), ignore)]
fn rom_loading() {
    // This is a large test as there is no easy way to rerun it without repeating the load
    let flash_mem = test_elf_fixture();
    // Note: we're using option_env! here, as ignored tests are built, but just not ran!
    let rom_mem = Some(std::fs::read(dbg!(option_env!("DRIVERLIB_PATH")).unwrap()).unwrap());
    let mut elf_params = ElfArgs::default();
    elf_params.load = true;

    let elf = ElfLoader::new(&flash_mem, rom_mem.as_deref(), &elf_params);
    let emulator = make_emulator(&elf);

    // Then...
    // ROM vectors
    // ROM reset stack is in GPRAM
    let rom_stack = Address::from_const(u32::from_le_bytes(read_word(
        &emulator,
        cc2650_constants::BROM::ADDR,
    )));
    assert!(
        rom_stack.is_in_range(&cc2650_constants::GPRAM::ADDR_SPACE)
            || rom_stack == cc2650_constants::GPRAM::ADDR_SPACE.end
    );
    // naming space is not polluted
    assert!(
        !name_an_address(&emulator, rom_stack)
            .to_ascii_lowercase()
            .starts_with("stack")
    );

    // reset isr is okay
    let rom_reset = Address::from_const(u32::from_le_bytes(read_word(
        &emulator,
        cc2650_constants::BROM::ADDR.offset(4),
    )));
    assert!(rom_reset.is_in_range(&cc2650_constants::BROM::ADDR_SPACE));
    // naming space works here
    assert_eq!(name_an_address(&emulator, rom_reset), "ResetISR_ROM+1");

    // ROM_CODE bss is not overwriting elf data
    let rom_sram_section = elf.rom_elf().unwrap().section_by_name("RAM_CODE").unwrap();
    assert_eq!(rom_sram_section.kind(), SectionKind::UninitializedData);
    let mut buf = vec![0u8; rom_sram_section.size() as usize];
    emulator
        .read_memory(
            Address::from_const(rom_sram_section.address() as u32),
            &mut buf,
        )
        .unwrap();
    assert_ne!(*buf.iter().max().unwrap(), 0);
}
