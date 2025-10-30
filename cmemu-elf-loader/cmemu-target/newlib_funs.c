// This is a syscalls implementation file for Newlib (libc for embedded)
// See https://www.sourceware.org/newlib/libc.html 2.1 for reference on implementing functions here
#include "semihosting.h"

#include <errno.h>
#include <sys/types.h>
#include <sys/time.h>
#include <sys/times.h>

#define HWREG(x) (*((volatile unsigned long *)(x)))


// standard newlib trick
#undef errno
extern int errno;

// end of preallocated heap
extern caddr_t _eheap;
typedef struct Magic Magic;
// min stack point
extern Magic _estack;
extern Magic _Min_Stack_Size;

/**
 * Extend the stack for at least ``nbytes``.
 * We have a predetermined end-of-free-RAM-for-heap in the .heap section,
 * but we need to duplicate some logic from .lds, as there is no "place at end" notion.
 * This can still clash with a deep stack larger than the _Min_Stack_Size!
 */
caddr_t _sbrk(int nbytes) {
    static caddr_t heap_ptr = NULL;
    caddr_t last_addr = ((caddr_t) &_estack) - (uint32_t) &_Min_Stack_Size;
    caddr_t base;

    if (heap_ptr == NULL) {
        heap_ptr = (caddr_t) &_eheap;
    }

    if (heap_ptr + nbytes < last_addr) {
        base = heap_ptr;
        heap_ptr += nbytes;
        return (base);
    } else {
        errno = ENOMEM;
        return ((caddr_t) -1);
    }
}


int _times(struct tms *ts) {
    volatile uint32_t *counter = (void *) CYCCNT_ADDR;
    if (ts != NULL) {
        ts->tms_cstime = 0;
        ts->tms_cutime = 0;
        ts->tms_utime = *(clock_t *) VTIME_USEC;
        // TODO: return real time here
        ts->tms_stime = 0;
    }
    return *counter;
}

clock_t clock() {
    return (clock_t) HWREG(CYCCNT_ADDR);
}

int _gettimeofday(struct timeval *tv, struct timezone *tz) {
    // Consistent if pipelined
    time_t sec = *(time_t *) (void *) UNIX_SEC;
    suseconds_t usec = *(suseconds_t *) (void *) UNIX_USEC;
    tv->tv_sec = sec;
    tv->tv_usec = usec;
    return 0;
}

int _isatty(int file) {
    return 1;
}

int _write(int file, char *ptr, int len) {
    int written = len;

    if ((file != 1) && (file != 2)) {
        errno = EBADF;
        return -1;
    }

    volatile char *dest_alias = file == 1 ? (void *) STDOUT_ALIAS : (void *) STDERR_ALIAS;

    // FIXME: why this code is broken? -- it is broken at the emulation of  memory routing with stms!
//  while (len > 0) {
//      int chunk = len > IO_ALIAS_SIZE ? IO_ALIAS_SIZE : len;
//      memcpy(dest_alias, ptr, chunk);
//      len -= chunk;
//      ptr += chunk;
//
//  }
    for (; len > 0; --len) {
        *dest_alias = *ptr++;
    }
    return written;
}

int _read(int file, char *ptr, int len) {
    int read = 0;
    uint32_t result;

    if (file != 0) {
        errno = EBADF;
        return -1;
    }

    /*
    volatile uint8_t *src = (void *) STDIN_ADDR;
    for (; len > 0; --len) {
        *ptr++ = *src;
        read++;
    }
     */

    volatile uint32_t *src = (void *) STDIN_OR_EOF_ADDR;
    for (; len > 0; --len) {
        result = *src;
        if (result >= 0x100)
            break;
        *ptr++ = (uint8_t) result;
        read++;
    }

    return read;
}

void __attribute__((noreturn)) _exit(int ret) {
    for (;;) {
        HWREG(EXIT_ADDR) = ret;
    }
}

void __attribute__((noreturn)) abort() {
    HWREG(PANIK_ADDR) = 222;
    _exit(222);
}

void __attribute__((constructor, used)) __init_libc_nano() {
    // Always start performance counters
    HWREG(DEBUG_ENABLE_ADDR) = DEBUG_ENABLE_VAL;
    HWREG(DWT_CTRL) |= CYCCNT_ENABLE_VAL;
}
