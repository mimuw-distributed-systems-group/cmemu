use cmemu_common::Address;

mod generated;

pub fn get_register_name(addr: Address) -> Option<&'static str> {
    generated::REGISTER_NAMES.get(&addr.to_const()).copied()
}

pub fn iter_known_registers() -> impl Iterator<Item = (&'static str, Address)> {
    generated::REGISTER_NAMES
        .entries()
        .map(|(&addr, &name)| (name, Address::from_const(addr)))
}
