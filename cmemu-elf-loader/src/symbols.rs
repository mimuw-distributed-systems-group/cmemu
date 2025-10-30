use cmemu_lib::common::Address;
use cmemu_lib::engine::SymbolsService;
use object::{ObjectSymbol, SymbolKind};
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::fmt::Display;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Symbol<'a> {
    start: Address,
    size: Option<u32>,
    type_: Type,
    name: Cow<'a, str>,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum Type {
    Unknown,
    Function,
    Variable,
}

impl Symbol<'_> {
    pub fn function(
        name: impl Into<Cow<'static, str>>,
        addr: Address,
        size: Option<u32>,
    ) -> Symbol<'static> {
        Symbol {
            start: addr.aligned_down_to_2_bytes(),
            size,
            type_: Type::Function,
            name: name.into(),
        }
    }

    pub fn variable(
        name: impl Into<Cow<'static, str>>,
        addr: Address,
        size: u32,
    ) -> Symbol<'static> {
        Symbol {
            start: addr,
            size: Some(size),
            type_: Type::Variable,
            name: name.into(),
        }
    }

    pub fn label(name: impl Into<Cow<'static, str>>, addr: Address) -> Symbol<'static> {
        Symbol {
            start: addr,
            size: None,
            type_: Type::Unknown,
            name: name.into(),
        }
    }
}

impl Symbol<'_> {
    /// Generate symbol address including interworking mechanisms
    #[allow(
        clippy::bool_to_int_with_if,
        reason = "Not a bool to int, but the architecture bit offset."
    )]
    #[must_use]
    pub fn symbol_address(&self) -> Address {
        self.start
            .offset(if self.type_ == Type::Function { 1 } else { 0 })
    }

    #[must_use]
    pub fn after_end(&self) -> Option<Address> {
        self.size.map(|s| self.start.offset(s))
    }

    #[must_use]
    pub fn relative(&self, addr: Address) -> SymbolMatch<'_, '_> {
        SymbolMatch {
            symbol: self,
            offset: addr.offset_from(self.start),
        }
    }

    #[must_use]
    pub fn to_unbound(&self) -> Symbol<'static> {
        Symbol {
            start: self.start,
            size: self.size,
            type_: self.type_,
            name: Cow::Owned(self.name.clone().into_owned()),
        }
    }

    // Internal api
    fn aligned(mut self) -> Self {
        if self.type_ == Type::Function {
            self.start = self.start.aligned_down_to_2_bytes();
        }
        self
    }

    fn phony(address: Address) -> Self {
        Self {
            start: address,
            size: Some(u32::MAX),
            type_: Type::Unknown,
            name: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SymbolMatch<'a, 'b> {
    symbol: &'b Symbol<'a>,
    offset: u32,
}

impl<'a, 'b> SymbolMatch<'a, 'b> {
    pub fn symbol(&self) -> &'b Symbol<'a> {
        self.symbol
    }
    pub fn offset(&self) -> u32 {
        self.offset
    }
    pub fn is_exact_match(&self) -> bool {
        self.offset == 0 || self.symbol.size.is_some_and(|s| self.offset < s)
    }
}

impl Display for SymbolMatch<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol.name)?;
        if self.offset != 0 {
            write!(f, "+{}", self.offset)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Symbols<'a> {
    symbols: BTreeSet<Symbol<'a>>,
}

impl Default for Symbols<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Symbols<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            symbols: BTreeSet::new(),
        }
    }

    fn merge<'s, 'b>(&'s mut self, other: Symbols<'b>)
    where
        'b: 'a,
        'a: 's,
    {
        // self.symbols.extend(other.symbols);
        // it would be faster with Cursor API, but this is not a bottleneck anywhere...
        // dedup same symbols
        for symbol in other.symbols {
            // Note: calling iter_matches() is troublesome with lifetimes.
            let dup = self
                .symbols
                .range(..Symbol::phony(symbol.start))
                .next_back();
            match dup {
                Some(here) if here.start == symbol.start && here.name == symbol.name => {
                    // gt is either a greater range or better type
                    if &symbol > here {
                        let our_symbol = here.clone();
                        self.symbols.remove(&our_symbol);
                        self.symbols.insert(symbol);
                    }
                }
                _ => {
                    self.symbols.insert(symbol);
                }
            }
        }
    }

    pub fn find_best_match(&self, addr: impl Into<Address>) -> Option<SymbolMatch<'_, '_>> {
        let addr = addr.into();
        self.symbols
            .range(..=Symbol::phony(addr))
            .next_back()
            .map(|sym| sym.relative(addr))
    }

    pub fn iter_matches(
        &self,
        addr: impl Into<Address>,
    ) -> impl Iterator<Item = SymbolMatch<'_, '_>> {
        let addr = addr.into();
        self.symbols
            .range(..=Symbol::phony(addr))
            .rev()
            .map(move |sym| sym.relative(addr))
            .take_while(SymbolMatch::is_exact_match)
    }

    #[must_use]
    pub fn into_unbound(self) -> Symbols<'static> {
        self.symbols.into_iter().map(|s| s.to_unbound()).collect()
    }
}

impl SymbolsService for Symbols<'_> {
    fn display_named_address(
        &self,
        addr: Address,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if let Some(symbol) = self.find_best_match(addr) {
            Display::fmt(&symbol, f)?;
        }
        Ok(())
    }
}

impl<'i: 's, 's> FromIterator<(&'i str, Address)> for Symbols<'s> {
    fn from_iter<I: IntoIterator<Item = (&'i str, Address)>>(iter: I) -> Self {
        Self {
            symbols: iter
                .into_iter()
                .map(|(name, addr)| Symbol {
                    start: addr,
                    size: None,
                    type_: Type::Unknown,
                    name: Cow::Borrowed(name),
                })
                .collect(),
        }
    }
}
impl<'i: 's, 's> Extend<(&'i str, Address)> for Symbols<'s> {
    fn extend<T: IntoIterator<Item = (&'i str, Address)>>(&mut self, iter: T) {
        self.merge(iter.into_iter().collect());
    }
}

impl<'i: 's, 's> FromIterator<Symbol<'i>> for Symbols<'s> {
    fn from_iter<I: IntoIterator<Item = Symbol<'i>>>(iter: I) -> Self {
        Self {
            symbols: iter.into_iter().collect(),
        }
    }
}
impl<'i: 's, 's> Extend<Symbol<'i>> for Symbols<'s> {
    fn extend<T: IntoIterator<Item = Symbol<'i>>>(&mut self, iter: T) {
        self.merge(iter.into_iter().collect());
    }
}

impl<'i: 's, 's> FromIterator<object::Symbol<'i, 'i>> for Symbols<'s> {
    fn from_iter<I: IntoIterator<Item = object::Symbol<'i, 'i>>>(iter: I) -> Self {
        Self {
            symbols: iter
                .into_iter()
                // We do a tricky filtering by ?-ing a None
                .filter_map(|osym| {
                    Some(
                        Symbol {
                            start: Address::from_const(u32::try_from(osym.address()).unwrap()),
                            size: if osym.size() == 0 {
                                None
                            } else {
                                Some(u32::try_from(osym.size()).unwrap())
                            },
                            type_: match osym.kind() {
                                SymbolKind::Text => Type::Function,
                                SymbolKind::Data => Type::Variable,
                                SymbolKind::Label | SymbolKind::Tls | SymbolKind::Unknown => {
                                    Type::Unknown
                                }
                                _ if osym.is_definition() => Type::Unknown,
                                _ => None?, // filter out
                            },
                            name: Cow::Borrowed(osym.name().ok()?),
                        }
                        .aligned(),
                    )
                })
                .collect(),
        }
    }
}
impl<'i: 's, 's> Extend<object::Symbol<'i, 'i>> for Symbols<'s> {
    fn extend<T: IntoIterator<Item = object::Symbol<'i, 'i>>>(&mut self, iter: T) {
        self.merge(iter.into_iter().collect());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::once;

    #[test]
    fn symbols() {
        let ss = Symbols::from_iter([("aaa", 8.into()), ("bbb", 7.into())]);
        println!("{ss:?}");
        let for_8 = ss.find_best_match(8).unwrap();
        assert_eq!(for_8.offset(), 0);
        assert!(for_8.is_exact_match());
        assert_eq!(for_8.symbol.name, "aaa");
        let for_7 = ss.find_best_match(7).unwrap();
        assert_eq!(for_7.offset(), 0);
        assert!(for_7.is_exact_match());
        assert_eq!(for_7.to_string(), "bbb");

        assert!(ss.find_best_match(1).is_none());

        let mut ss = ss;
        ss.extend([("ccc", 15.into())]);

        let for_10 = ss.find_best_match(10).unwrap();
        assert_eq!(for_10.offset(), 2);
        assert!(!for_10.is_exact_match());
        assert_eq!(for_10.to_string(), "aaa+2");

        assert!(ss.iter_matches(10).next().is_none());
    }

    #[test]
    fn symbols_iterators() {
        let part1 = [
            Symbol::function("func", 13.into(), Some(12)),
            Symbol {
                start: 4.into(),
                size: None,
                type_: Type::Unknown,
                name: String::from("unk").into(),
            },
        ];
        let part2 = (5u32..10).map(|i| Symbol {
            start: Address::from_const(i * 2),
            size: Some(i),
            type_: Type::Variable,
            name: format!("var{}", i * 2).into(),
        });
        let part3 = once(Symbol::label("label", 15.into()));
        let mut ss: Symbols = part1.into_iter().chain(part2).collect();
        ss.extend(part3);
        println!("{ss:?}");
        let mut for_15 = ss.iter_matches(15);
        assert_eq!(for_15.next().unwrap().to_string(), "label");
        assert_eq!(for_15.next().unwrap().to_string(), "var14+1");
        assert_eq!(for_15.next().unwrap().to_string(), "func+3"); // sic! alignment
        assert_eq!(for_15.next().unwrap().to_string(), "var12+3"); // after because of size
        assert!(for_15.next().is_none(), "var10+5 is not in 10..15");
    }
}

#[cfg(feature = "builder")]
mod build {
    use super::{Symbols, Type};
    use cc2650_constants::{FLASHMEM, SRAM};
    use object::Endianness;
    use object::build::ByteString;
    use object::build::elf::{Builder, SectionData};
    use object::elf::{
        EF_ARM_ABI_FLOAT_SOFT, EF_ARM_EABI_VER5, ELFOSABI_NONE, EM_ARM, ET_EXEC, SHF_ALLOC,
        SHF_EXECINSTR, SHF_WRITE, SHN_ABS, SHT_NOBITS, SHT_STRTAB, SHT_SYMTAB, STB_GLOBAL,
        STT_FUNC, STT_NOTYPE, STT_OBJECT,
    };
    use object::write::{StreamingBuffer, WritableBuffer};
    use std::fs;
    use std::io::{BufWriter, Write};
    use std::path::Path;

    impl<'a> Symbols<'a> {
        fn fill_text_data_or_abs<'b, 'data>(&'b self, builder: &mut Builder<'data>)
        where
            'a: 'data,
            'b: 'data,
        {
            // These sections are needed for GDB to actually consider our symbols for real
            // We could fill the contents with FLASH data, but
            // a) we don't have &Emulator here
            // b) it works already
            let text = builder.sections.add();
            text.name = ".text".into();
            text.sh_type = SHT_NOBITS;
            text.sh_flags = u64::from(SHF_ALLOC | SHF_EXECINSTR);
            text.sh_addralign = 8;
            text.sh_addr = u64::from(FLASHMEM::ADDR.to_const());
            text.data = SectionData::UninitializedData(u64::from(FLASHMEM::SIZE));
            let text = text.id();

            let data = builder.sections.add();
            data.name = ".data".into();
            data.sh_type = SHT_NOBITS;
            data.sh_addr = u64::from(SRAM::ADDR.to_const());
            data.sh_flags = u64::from(SHF_ALLOC | SHF_WRITE);
            data.sh_addralign = 8;
            data.data = SectionData::UninitializedData(u64::from(SRAM::SIZE));
            let data = data.id();

            for sym in &self.symbols {
                let new = builder.symbols.add();
                new.name = ByteString::from(sym.name.as_ref());
                new.st_size = u64::from(sym.size.unwrap_or(0));

                new.st_value = u64::from(sym.symbol_address().to_const());
                if sym.start.is_in_range(&FLASHMEM::ADDR_SPACE) {
                    new.section = Some(text);
                } else if sym.start.is_in_range(&SRAM::ADDR_SPACE) {
                    new.section = Some(data);
                } else {
                    new.st_shndx = SHN_ABS;
                }

                new.set_st_info(
                    STB_GLOBAL,
                    match sym.type_ {
                        Type::Unknown => STT_NOTYPE,
                        // Note, there is STT_ARM_TFUNC, but arm-none-eabi-gcc makes STT_FUNC
                        Type::Function => STT_FUNC,
                        Type::Variable => STT_OBJECT,
                    },
                );
            }
        }

        pub fn write_stub(&self, w: impl Write) -> Result<(), Box<dyn std::error::Error>> {
            let mut stream = StreamingBuffer::new(w);
            self.write_stub_to_buffer(&mut stream)?;
            stream.result()?;
            Ok(())
        }

        pub fn write_stub_to_buffer(
            &self,
            w: &mut impl WritableBuffer,
        ) -> Result<(), Box<dyn std::error::Error>> {
            let mut builder = Self::minimal_builder();
            self.fill_text_data_or_abs(&mut builder);
            builder.write(w)?;
            Ok(())
        }

        pub fn write_stub_to_file(
            &self,
            path: impl AsRef<Path>,
        ) -> Result<(), Box<dyn std::error::Error>> {
            let f = fs::File::create(path)?;
            self.write_stub(&mut BufWriter::new(f))
        }

        fn minimal_builder<'data>() -> Builder<'data> {
            let mut builder = Builder::new(Endianness::Little, false);
            let hdr = &mut builder.header;
            hdr.os_abi = ELFOSABI_NONE;
            hdr.e_type = ET_EXEC;
            hdr.e_machine = EM_ARM;
            hdr.e_flags = EF_ARM_ABI_FLOAT_SOFT | EF_ARM_EABI_VER5;

            let strtab = builder.sections.add();
            strtab.name = ".strtab".into();
            strtab.sh_type = SHT_STRTAB;
            strtab.sh_addralign = 1;
            strtab.data = SectionData::String;

            let symtab = builder.sections.add();
            symtab.name = ".symtab".into();
            symtab.sh_type = SHT_SYMTAB;
            symtab.sh_addralign = 4;
            symtab.sh_entsize = 16;
            symtab.data = SectionData::Symbol;

            let shstrtab = builder.sections.add();
            shstrtab.name = ".shstrtab".into();
            shstrtab.sh_type = SHT_STRTAB;
            shstrtab.sh_addralign = 1;
            shstrtab.data = SectionData::SectionString;
            builder
        }
    }

    #[cfg(test)]
    mod tests {
        use super::super::*;
        use object::Object;

        #[test]
        fn roundtrip() {
            let ss = Symbols::from_iter([
                Symbol::function("func", 12.into(), Some(34)),
                Symbol::label("labella", 15.into()),
                Symbol::variable("var", 15.into(), 3),
                Symbol::phony(1337.into()),
            ]);
            let mut buf = Vec::new();
            ss.write_stub_to_buffer(&mut buf).unwrap();

            let obj = object::File::parse(&*buf).unwrap();
            let ss2: Symbols = obj.symbols().collect();
            for (s1, s2) in ss.symbols.iter().zip(ss2.symbols.iter()) {
                assert_eq!(s1, s2);
                println!("{s1:?}");
            }
        }

        #[test]
        fn empty() {
            let ss = Symbols::new();
            let mut buf = Vec::new();
            ss.write_stub(&mut buf).unwrap();
            object::File::parse(&*buf).unwrap();
        }

        #[test]
        fn error_catching() {
            let ss = Symbols::from_iter([("aaa", 12.into()), ("bbb", 13.into())]);
            let mut buf = [0u8; 64];
            assert!(ss.write_stub(&mut buf[..]).is_err());
        }
    }
}
