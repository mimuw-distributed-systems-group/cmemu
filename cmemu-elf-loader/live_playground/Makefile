stdlib_targets := $(patsubst %.c,%.elf,$(wildcard *.c))
raw_targets := $(patsubst %.S,%.elf,$(wildcard *.S))

all: $(stdlib_targets) $(raw_targets)

# This need to be later, as targets are expanded eagerly
# It is here, however, to allow overriding some flags later
include ../cmemu-target/Makefile.base


#OPTIMIZATION_FLAGS ?= -Os -g -ffunction-sections -fdata-sections
#OPTIMIZATION_FLAGS ?= -O0 -g -ffunction-sections -fdata-sections
#OPTIMIZATION_FLAGS = -flto -ffat-lto-objects -Os -ggdb3 -gz
OPTIMIZATION_FLAGS = -Og -ggdb3 -gz

OBJDUMP_FLAGS += --disassemble  --visualize-jumps=extended-color -w
ifdef WITH_SYMBOLS
OBJDUMP_FLAGS += -T -r -t -h
endif

#LDFLAGS += -Wl,-Map=app.map,--cref,--no-warn-mismatch

### Don't treat the .elf as intermediate
.PRECIOUS: %.elf %.flash %.o
.PHONY: clean all FORCE

# Replace suffix
$(stdlib_targets:.elf=-objdump): OBJDUMP_FLAGS += --source

# pattern cannot be in .PHONY
%-objdump: %.elf FORCE
	$(OBJDUMP) $(OBJDUMP_FLAGS) $<

FORCE: ;

clean:
	-rm -f *.elf *.o *.flash
