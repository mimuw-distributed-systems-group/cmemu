# See playground/mm319369/cmemu-progs for more complex Makefile/examples if needed to bring them here as tests.
stdlib_targets := test_syscalls_io.elf test_syscalls.elf panic.elf crypto.elf umull_mla_bug.elf mandelbrot.elf contiki-aes.elf \
                  $(patsubst %.S,%.elf,$(wildcard bugs_mini_tests/*.c))
raw_targets := minimal.elf asm_complex_hosting.elf min_max_example_from_paper.elf $(patsubst %.S,%.elf,$(wildcard bugs_mini_tests/*.S))

all: $(stdlib_targets) $(raw_targets)

# This need to be later, as targets are expanded eagerly
# It is here, however, to allow overriding some flags later
include ../../../../cmemu-elf-loader/cmemu-target/Makefile.base


#OPTIMIZATION_FLAGS = -flto -ffat-lto-objects -Os -ggdb3 -gz
#OPTIMIZATION_FLAGS = -Os
OPTIMIZATION_FLAGS = -Og -ggdb3 -gz

OBJDUMP_FLAGS += --disassemble  --visualize-jumps=extended-color -w
ifdef WITH_SYMBOLS
OBJDUMP_FLAGS += -T -r -t -h
endif

TARGET ?= all

### Don't treat the .elf as intermediate
.PRECIOUS: %.elf
.PHONY: clean all FORCE


mandelbrot.elf: CFLAGS += -Wno-error -w -fno-lto
# Replace suffix
$(stdlib_targets:.elf=-objdump): OBJDUMP_FLAGS += --source

# pattern cannot be in .PHONY
%-objdump: %.elf FORCE
	$(OBJDUMP) $(OBJDUMP_FLAGS) $<

FORCE: ;

clean:
	-rm -f *.elf *.o *.flash
	-cd bugs && rm -f *.elf *.o *.flash
