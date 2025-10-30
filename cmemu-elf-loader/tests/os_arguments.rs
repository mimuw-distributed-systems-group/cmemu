use cmemu_elf_loader::{ElfArgs, ElfLoader, HostingArgs};
use cmemu_lib::common::{
    Address,
    cmemu_hosting::{OS_DATA_ABI_VER, OS_DATA_ARGC, OS_DATA_ARGV, OS_DATA_ENVIRON, OS_DATA_RANGE},
};
use cmemu_lib::engine::Emulator;
use std::env;
use std::ffi::CStr;

fn test_elf_fixture() -> Vec<u8> {
    std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test.elf")).unwrap()
}

fn read_u32(emulator: &Emulator, address: impl Into<Address>) -> u32 {
    let mut buf = [0u8; 4];
    emulator.read_memory(address.into(), &mut buf).unwrap();
    u32::from_le_bytes(buf)
}

fn read_32b(emulator: &Emulator, address: impl Into<Address>) -> [u8; 32] {
    let mut buf = [0u8; 32];
    emulator.read_memory(address.into(), &mut buf).unwrap();
    buf
}

fn make_emulator(elf: &ElfLoader) -> Emulator {
    let mut emulator = Emulator::new(elf.flash_base(), elf.rom_base());
    elf.load(&mut emulator);
    emulator
}

#[test]
fn unconfigured_is_uninitialized() {
    let flash_mem = test_elf_fixture();
    let rom_mem = None;
    let elf_params = ElfArgs::default();
    let elf = ElfLoader::new(&flash_mem, rom_mem, &elf_params);
    let emulator = make_emulator(&elf);

    // Then...
    assert_eq!(read_u32(&emulator, OS_DATA_ABI_VER), 0);
    assert_eq!(read_u32(&emulator, OS_DATA_ARGC), 0);
    assert_eq!(read_u32(&emulator, OS_DATA_ARGV), 0);
    assert_eq!(read_u32(&emulator, OS_DATA_ENVIRON), 0);
}

#[test]
fn passing_args_with_env() {
    // This is a large test as there is no easy way to rerun it without repeating the load
    let flash_mem = test_elf_fixture();
    let rom_mem = None;
    let mut elf_params = ElfArgs::default();
    elf_params.hosting_args = Some({
        let mut hosting_args = HostingArgs::default();
        hosting_args.inherit_env = false;
        hosting_args.env = vec!["KEY=VALUE".to_owned()];
        hosting_args.arg0 = "APP_TEST".to_owned();
        hosting_args.app_args = vec!["arg1".to_owned(), "arg2".to_owned()];
        hosting_args
    });
    let elf = ElfLoader::new(&flash_mem, rom_mem, &elf_params);
    let emulator = make_emulator(&elf);

    // Then...
    // assert_eq!(name_an_address(&emulator, OS_DATA_ARGC), "OS_DATA_ARGC"); // maybe one day
    assert_eq!(read_u32(&emulator, OS_DATA_ABI_VER), 0);
    // arg0 is the name
    assert_eq!(read_u32(&emulator, OS_DATA_ARGC), 3);
    let argv_addr = Address::from_const(read_u32(&emulator, OS_DATA_ARGV));
    let environ_addr = Address::from_const(read_u32(&emulator, OS_DATA_ENVIRON));
    assert!(argv_addr.is_in_range(&OS_DATA_RANGE));
    assert!(environ_addr.is_in_range(&OS_DATA_RANGE));

    // validate environ
    let environ_at_0 = Address::from_const(read_u32(&emulator, environ_addr));
    assert!(environ_at_0.is_in_range(&OS_DATA_RANGE));
    assert_eq!(read_u32(&emulator, environ_addr.offset(4)), 0);
    assert_eq!(
        CStr::from_bytes_until_nul(&read_32b(&emulator, environ_at_0)).unwrap(),
        c"KEY=VALUE"
    );

    // validate argv
    let argv_at_0 = Address::from_const(read_u32(&emulator, argv_addr));
    assert!(argv_at_0.is_in_range(&OS_DATA_RANGE));
    assert_eq!(
        CStr::from_bytes_until_nul(&read_32b(&emulator, argv_at_0)).unwrap(),
        c"APP_TEST"
    );
    let argv_at_2 = Address::from_const(read_u32(&emulator, argv_addr.offset(4 * 2)));
    assert!(argv_at_2.is_in_range(&OS_DATA_RANGE));
    assert_eq!(
        CStr::from_bytes_until_nul(&read_32b(&emulator, argv_at_2)).unwrap(),
        c"arg2"
    );
    assert_eq!(read_u32(&emulator, argv_addr.offset(4 * 3)), 0);
}

#[test]
fn initialized_no_args() {
    let flash_mem = test_elf_fixture();
    let rom_mem = None;
    let mut elf_params = ElfArgs::default();
    elf_params.hosting_args = Some({
        let mut hosting_args = HostingArgs::default();
        hosting_args.inherit_env = false;
        hosting_args.no_arg0 = true;
        hosting_args
    });
    let elf = ElfLoader::new(&flash_mem, rom_mem, &elf_params);
    let emulator = make_emulator(&elf);

    // Then...
    // assert_eq!(name_an_address(&emulator, OS_DATA_ARGC), "OS_DATA_ARGC"); // maybe one day
    assert_eq!(read_u32(&emulator, OS_DATA_ABI_VER), 0);
    assert_eq!(read_u32(&emulator, OS_DATA_ARGC), 0);

    let argv_addr = Address::from_const(read_u32(&emulator, OS_DATA_ARGV));
    assert!(argv_addr.is_in_range(&OS_DATA_RANGE));
    assert_eq!(read_u32(&emulator, argv_addr), 0);

    let environ_addr = Address::from_const(read_u32(&emulator, OS_DATA_ENVIRON));
    assert!(environ_addr.is_in_range(&OS_DATA_RANGE));
    // default environ is not empty
    // assert_eq!(read_u32(&emulator, environ_addr), 0);
}

#[test]
fn inherit_env() {
    // Our environment should be pretty large.
    let env_var_key = "CARGO_MANIFEST_DIR";
    let cargo_manifest_dir = env::var(env_var_key).unwrap();

    let flash_mem = test_elf_fixture();
    let rom_mem = None;
    let mut elf_params = ElfArgs::default();
    elf_params.hosting_args = Some({
        let mut hosting_args = HostingArgs::default();
        hosting_args.inherit_env = true;
        hosting_args.arg0 = "APP_TEST".to_owned();
        hosting_args
    });
    let elf = ElfLoader::new(&flash_mem, rom_mem, &elf_params);
    let emulator = make_emulator(&elf);

    // Then...
    // assert_eq!(name_an_address(&emulator, OS_DATA_ARGC), "OS_DATA_ARGC"); // maybe one day
    assert_eq!(read_u32(&emulator, OS_DATA_ABI_VER), 0);
    assert_eq!(read_u32(&emulator, OS_DATA_ARGC), 1);

    let mut environ_addr = Address::from_const(read_u32(&emulator, OS_DATA_ENVIRON));
    assert!(environ_addr.is_in_range(&OS_DATA_RANGE));

    let mut buf;
    // walk until we find the cargo var!
    let in_emulator_manifest_dir = loop {
        let environ_at_i = Address::from_const(read_u32(&emulator, environ_addr));
        assert!(
            environ_at_i.is_in_range(&OS_DATA_RANGE),
            "We're out of range: {environ_at_i:?}"
        );
        buf = read_32b(&emulator, environ_at_i);
        let var = if let Ok(x) = CStr::from_bytes_until_nul(&buf) {
            x
        } else {
            // Truncate the string
            *buf.last_mut().unwrap() = 0;
            CStr::from_bytes_with_nul(&buf).unwrap()
        };

        if dbg!(var).to_string_lossy().starts_with(env_var_key) {
            break var;
        }

        environ_addr = environ_addr.offset(4);
    };
    assert_eq!(buf[env_var_key.len()], b'=');
    let got_manifest_dir_value = &in_emulator_manifest_dir[env_var_key.len() + 1..];
    assert!(cargo_manifest_dir.starts_with(valid_str_prefix(got_manifest_dir_value.to_bytes())));
}

fn valid_str_prefix(inp: &[u8]) -> &str {
    std::str::from_utf8(inp)
        .unwrap_or_else(|e| std::str::from_utf8(&inp[..e.valid_up_to()]).unwrap())
}
