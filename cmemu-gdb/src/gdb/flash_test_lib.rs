use crate::DynResult;
use crate::gdb::DebugMonitor;
use crate::gdb::monitor::MonitorError;
use clap::Subcommand;
use cmemu_elf_loader::{Symbol, Symbols};
use cmemu_flash_test_lib::{MemoryFormat, TestDump, TestDumpMemoryChunk};
use cmemu_lib::engine::Emulator;
use gdbstub::target::ext::monitor_cmd::ConsoleOutput;
use gdbstub::{output, outputln};
use std::fmt::Display;

pub(crate) struct FlashTestState {
    pub configuration_name: String,
    pub mem_dump: Vec<TestDumpMemoryChunk>,
}

/// Commands related to a flash-test-case
#[derive(Subcommand, Debug)]
pub(super) enum FlashTestCmd {
    /// Print the path to a generated symbols file
    SymbolsFile,
    /// Get configuration of the test
    Configuration,
    #[command(alias = "summary")]
    /// Display summary about the test
    Info,
    /// Tell me how to print/watch memory
    Howto,
    /// Show saved dump
    Dump {
        symbol: String,
        /// specifies a shown memory format
        memory_format: Option<MemoryFormat>,
    },
    /// Compare memory state with the saved dump
    Compare {
        symbol: String,
        /// specifies a shown memory format
        memory_format: Option<MemoryFormat>,
    },
    // Having "trap on a wrong answer" would be great, but we don't have value-watchpoints
    // Maybe we can hack it with waiting for a next instruction.
}

impl DebugMonitor {
    #[allow(clippy::unit_arg)]
    pub(crate) fn configure_for_test(&mut self, dump: TestDump) -> DynResult<()> {
        let modify_emu =
            move |emu: &mut Emulator| emu.set_nonstandard_entrypoint(Some(dump.emulator_main_addr));
        modify_emu(&mut self.emu);
        self.rerun_hooks
            .push(Box::new(move |_, emu| Ok(modify_emu(emu))));

        self.traps.insert(dump.emulator_cdl_start_addr);
        self.traps.insert(dump.emulator_exit_addr);

        // note: :: separators collide with gdb's modules parsing
        let mut symbols: Symbols<'static> = cc2650_constants::iter_known_registers().collect();
        // let mut symbols: Symbols<'static> = Symbols::new();
        if let Some(ds) = dump.symbols {
            symbols.extend(ds.into_iter().map(|(k, v)| Symbol::label(k, v)));
        } else {
            symbols.extend([
                ("emulator_main", dump.emulator_main_addr),
                ("emulator_exit", dump.emulator_exit_addr),
                ("emulator_cdl_start", dump.emulator_cdl_start_addr),
            ]);
        }
        symbols.extend(dump.mem_dump.iter().map(|m| {
            Symbol::variable(
                m.symbol_name.clone(),
                m.addr,
                u32::try_from(m.content.len()).unwrap(),
            )
        }));
        let tmp_file = tempfile::Builder::new()
            .suffix(".elf")
            .prefix("cmemu-test-syms")
            .tempfile()?;
        symbols.write_stub_to_file(tmp_file.path())?;
        self.symbols_file = Some(tmp_file);
        // There is no new alloc as we stole the strings from TestDump
        self.emu.set_symbols_service(Some(Box::new(symbols)));

        self.flash_test = Some(FlashTestState {
            configuration_name: dump.configuration_name,
            mem_dump: dump.mem_dump,
        });
        Ok(())
    }

    pub(super) fn flash_test_status(&self, out: &mut ConsoleOutput<'_>) {
        if let Some(ref _test) = self.flash_test {
            outputln!(out, "This is a flash test runtime: mo flash-test info");
            outputln!(out);
            out.flush();
        }
    }

    fn display_symbols_path(&self) -> impl Display + '_ {
        self.symbols_file.as_ref().unwrap().path().display()
    }

    #[allow(clippy::too_many_lines, reason = "Simple flow, mostly text.")]
    pub(super) fn flash_test_command(
        &mut self,
        cmd: FlashTestCmd,
        out: &mut ConsoleOutput<'_>,
    ) -> Result<(), MonitorError> {
        let Some(ref test) = self.flash_test else {
            return Err(MonitorError::Runtime("Not a flash-test file".to_owned()));
        };
        match cmd {
            FlashTestCmd::SymbolsFile => {
                outputln!(out, "{}", self.display_symbols_path());
            }
            FlashTestCmd::Configuration => {
                outputln!(out, "{}", test.configuration_name);
            }
            FlashTestCmd::Info => {
                outputln!(out, "Configuration: {}", test.configuration_name);
                outputln!(out, "Test path: {}", self.image_path.display());
                outputln!(out, "Fake symbols ELF: {}", self.display_symbols_path());
                outputln!(out, "\nSaved symbols:");
                out.flush();
                for region in &test.mem_dump {
                    outputln!(
                        out,
                        "  {} at {:?}: {} bytes",
                        region.symbol_name,
                        region.addr,
                        region.content.len()
                    );
                    out.flush();
                }
            }
            FlashTestCmd::Howto => {
                outputln!(out, "GDB doesn't know the type of our saved symbols,");
                outputln!(out, "so you need to cast them explicitly.");
                outputln!(
                    out,
                    "After casting, you may use `print` or `watch` commands."
                );
                outputln!(
                    out,
                    "Cast to an array works for 'print', but hardware watch needs more tricks."
                );
                outputln!(
                    out,
                    "You can use labels like '__Ltrigger_*' for breakpoints."
                );
                outputln!(out, "We've set traps at test start/end for you.");
                outputln!(out);
                out.flush();

                for region in &test.mem_dump {
                    let len = region.content.len();
                    outputln!(out, "Symbol {}:", region.symbol_name);
                    outputln!(out, "print/d (char[{}]) {}", len, region.symbol_name);
                    outputln!(out, "watch -l *(char[{}]*) &{}", len, region.symbol_name);
                    if len & 3 == 0 {
                        outputln!(out, "watch -l *(int[{}]*) &{}", len / 4, region.symbol_name);
                    }
                    outputln!(out);
                    out.flush();
                }
            }
            FlashTestCmd::Dump {
                symbol,
                memory_format,
            } => {
                let region = find_dump(&test.mem_dump, &symbol)?;
                outputln!(
                    out,
                    "{}",
                    memory_format
                        .unwrap_or_default()
                        .display(&region.content, true)
                );
            }
            FlashTestCmd::Compare {
                symbol,
                memory_format,
            } => {
                let region = find_dump(&test.mem_dump, &symbol)?;
                let mut mem = vec![0; region.content.len()];
                self.emu
                    .read_memory(region.addr, &mut mem)
                    .map_err(|e| MonitorError::RuntimeDyn(e.into()))?;
                let fmt = memory_format.unwrap_or_default();
                let chunk_size = fmt.elem_len();
                let styles = self.styles();
                let bad = styles.get_invalid();
                let good = styles.get_valid();
                let mut first_err = None;

                for (pos, (got, exp)) in mem
                    .chunks(chunk_size)
                    .zip(region.content.chunks(chunk_size))
                    .enumerate()
                {
                    if got != exp {
                        first_err.get_or_insert(pos);
                        output!(out, "{bad}{}{bad:#} != ", fmt.display(got, false));
                    }
                    output!(out, "{good}{}{good:#}", fmt.display(exp, false));
                    output!(out, ", ");
                    out.flush();
                }
                outputln!(out);

                if let Some(pos) = first_err {
                    outputln!(
                        out,
                        "First error at item {pos} (offset {:+x}). Watch for it with:",
                        pos * chunk_size
                    );
                    outputln!(
                        out,
                        "watch -l *(({}*) &{} + {pos})",
                        if chunk_size == 4 { "int" } else { "char" },
                        region.symbol_name,
                    );
                }
            }
        }
        Ok(())
    }
}

fn find_dump<'a>(
    dumps: &'a [TestDumpMemoryChunk],
    symbol: &String,
) -> Result<&'a TestDumpMemoryChunk, MonitorError> {
    dumps
        .iter()
        .find(|region| &region.symbol_name == symbol)
        .ok_or_else(|| MonitorError::Runtime("No such symbol.".into()))
}
