/* 
    Copyright (C) 2021-2025 CMEmu authors
    
    This file is licensed under the same license as CMEmu (MIT AND/OR Apache 2)
    
    It was changed a bit to work out-of-the box with arm-none-eabi-gcc,
    to compile sources that would work both as native applications for TI CC2650
    and native cmemu-hosted applications.
    
    It means, that the result file has both:
    a) standard vector table (with SP and ResetISR) at the beginning of Flash
    b) constructor/destructor tables used by a CRT from gcc
    
    In particular, the startup may look different in an emulator and on the hardware:
    
    hardware – entrypoint address is at Flash+0x4, the app loads data into SRAM/GPRAM, runs device-specific init
    emulator – entrypoint is in an ELF header, emulator places all segments at their vaddr, _start is ran
*/

/*
 @file       cc26x0f128.lds
 @brief      CC26x0F128 linker configuration file for GNU compiler.

 @Revised     $Date: 2015-11-20 13:18:46 +0100 (fr, 20 nov 2015) $
 @Revision    $Revision: 16377 $

  Copyright (C) 2015 Texas Instruments Incorporated - http:www.ti.com/


  Redistribution and use in source and binary forms, with or without
  modification, are permitted provided that the following conditions
  are met:

    Redistributions of source code must retain the above copyright
    notice, this list of conditions and the following disclaimer.

    Redistributions in binary form must reproduce the above copyright
    notice, this list of conditions and the following disclaimer in the
    documentation and/or other materials provided with the distribution.

    Neither the name of Texas Instruments Incorporated nor the names of
    its contributors may be used to endorse or promote products derived
    from this software without specific prior written permission.

  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
  "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
  LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
  A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
  OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
  SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
  LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
  DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
  THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
  (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
  OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

/* Copyright (C) 2014-2021 Free Software Foundation, Inc.
   Copying and distribution of this script, with or without modification,
   are permitted in any medium without royalty provided the copyright
   notice and this notice are preserved.  */


/* Changed even more to define sections used by libc_nano. Those entries have 2 space indent. */
MEMORY
{
    FLASH (RX) : ORIGIN = 0x0, LENGTH = 0x1D000
    FLASH_FLOG (R) : ORIGIN = 0x1D000, LENGTH = 0x1000
    FLASH_BL (RX) : ORIGIN = 0x1E000, LENGTH = 0x1FA8
    FLASH_CCFG (RX) : ORIGIN = 0x1FFA8, LENGTH = 88

/* How to quickly hack a larger Flash size: */
/*    FLASH (RX) : ORIGIN = 0x0, LENGTH = 0x1D0090*/
/*    FLASH_FLOG (R) : ORIGIN = 0x1D0000, LENGTH = 0x10000*/
/*    FLASH_BL (RX) : ORIGIN = 0x1E0000, LENGTH = 0x1FA8*/
/*    FLASH_CCFG (RX) : ORIGIN = 0x1FFA80, LENGTH = 88*/

    SRAM (RWX) : ORIGIN = 0x20000000, LENGTH = 0x5000
    GPRAM (RWX) : ORIGIN = 0x11000000, LENGTH = 0x2000
}

/* Highest address of the stack. Used in the startup file.*/
_estack = ORIGIN(SRAM) + LENGTH(SRAM) - SIZEOF(.highram_reserve);

_sram = ORIGIN(SRAM);
_esram = ORIGIN(SRAM) + LENGTH(SRAM);

__stack = _estack; /* For crt */
_Min_Stack_Size = 0x200;

ENTRY(_start)

SECTIONS
{
    .text :
    {
        _text = .;
        KEEP(*(.vectors))
        *(.text*)
        *(.rodata*)
        KEEP (*(.init))
        KEEP (*(.fini*))
        *(.eh_frame*)
        /* XXX: should we move it after .dtors? */
        _etext = .;
    } > FLASH = 0


  .preinit_array    :
  {
    PROVIDE_HIDDEN (__preinit_array_start = .);
    KEEP (*(.preinit_array))
    PROVIDE_HIDDEN (__preinit_array_end = .);
  } > FLASH
  .init_array    :
  {
    PROVIDE_HIDDEN (__init_array_start = .);
    /* FIXME: why are .ctors.* here? Is this a duplicate? */
    KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))
    KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))
    PROVIDE_HIDDEN (__init_array_end = .);
  } > FLASH
  .fini_array    :
  {
    PROVIDE_HIDDEN (__fini_array_start = .);
    KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))
    KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))
    PROVIDE_HIDDEN (__fini_array_end = .);
  } > FLASH
  .ctors          :
  {
    /* gcc uses crtbegin.o to find the start of
       the constructors, so we make sure it is
       first.  Because this is a wildcard, it
       doesn't matter if the user does not
       actually link against crtbegin.o; the
       linker won't look for a file to match a
       wildcard.  The wildcard also means that it
       doesn't matter which directory crtbegin.o
       is in.  */
    KEEP (*crtbegin.o(.ctors))
    KEEP (*crtbegin?.o(.ctors))
    /* We don't want to include the .ctor section from
       the crtend.o file until after the sorted ctors.
       The .ctor section from the crtend file contains the
       end of ctors marker and it must be last */
    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))
    KEEP (*(SORT(.ctors.*)))
    KEEP (*(.ctors))
  } > FLASH
  .dtors          :
  {
    KEEP (*crtbegin.o(.dtors))
    KEEP (*crtbegin?.o(.dtors))
    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))
    KEEP (*(SORT(.dtors.*)))
    KEEP (*(.dtors))
  } > FLASH

    .ARM.exidx :
    {
        __exidx_start = .;
        *(.ARM.exidx* .gnu.linkonce.armexidx.*)
        __exidx_end = .;
    } > FLASH

    .ARM.extab :
    {
        *(.ARM.extab* .gnu.linkonce.armextab.*)
    } > FLASH

    /* __bss_*__ are for libstd */
    .bss (NOLOAD) : ALIGN(4)
    {
        _bss = .;
        __bss_start__ = .;
        __bss_start = .;
        *(.bigalignbss*)
        *(.bss*)
        *(COMMON)
        _ebss = .;
        __bss_end__ = .;
        __bss_end = .;
        _bss_end__ = .;
    } > SRAM

  .noinit (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(32 / 8);
    PROVIDE (__noinit_start = .);
    *(.noinit .noinit.*)
    . = ALIGN(32 / 8);
    PROVIDE (__noinit_end = .);
  } > SRAM


    .data : ALIGN(4)
    {
        _data = .;
        __data_start = .;
        *(vtable)
        *(.data*)
  *(.gnu.linkonce.d.*)
        _edata = .;
    } > SRAM AT > FLASH
    _ldata = LOADADDR(.data);

    .gpram : ALIGN(4)
    { 
        _gpram = .;
        *(.gpram*)
        _egpram = .;
    } > GPRAM AT > FLASH
    _lgpram = LOADADDR(.gpram);

    /* This section is replaced by a generated NULL-terminated version string,
     * see Makefile.main.
     *
     * Put all sections that uses flash memory before `.appversion` section
     * to avoid overwriting them by the generated version string. */
    .appversion : ALIGN(1)
    {
        KEEP(*(.appversion))
    } > FLASH

    .flog : ALIGN(1)
    {
        _flog = .;
        . = . + LENGTH(FLASH_FLOG);
        _eflog = .;
    } > FLASH_FLOG = 0

    .heap (NOLOAD) : ALIGN(1)
    {
        . = ALIGN(4);
        _heap = .;
        KEEP(*(.heap*))
        _eheap = .;
    } > SRAM

    /* A dummy section, used to check that there is at least _Min_Stack_Size
       bytes of SRAM left */
    .minstack (NOLOAD) :
    {
        . = ALIGN(4);
        _stack = .;
        *(.stack*)
        . = . + _Min_Stack_Size;
        . = ALIGN(4);
    } > SRAM

    /* This is resereved at the END of SRAM. We put it here to:
     * a) assert the space will fit
     *    (i.e., if we overflow into the reserve, we will also overflow SRAM),
     * b) get its size.
     * At the end, the reserve is from _estack to _esram.
     * If there is no such input section, this should be a no-op.
     */
    .highram_reserve (NOLOAD) :
    {
        . = ALIGN(4);
        KEEP(*(.highram_reserve*))
        . = ALIGN(4);
    } > SRAM

    .bootloader :
    {
        _bootloader = .;
        bootloader_entry = .;
        . = . + LENGTH(FLASH_BL);
        _ebootloader = .;
    } > FLASH_BL = 0

    .ccfg :
    { 
        KEEP(*(.ccfg*))
    } > FLASH_CCFG
}
