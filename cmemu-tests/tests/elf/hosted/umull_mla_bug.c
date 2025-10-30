#include <stdbool.h>

unsigned test_bug(bool trigger) {
    unsigned volatile div_stall_op = 1<<29;
    __asm volatile (
        // Set the correct destination as the "stale" LR
            "adr.w lr, continue%=+1\n"
            // Conditionally (trigger != 0) set a wrong destination as the "stale" LR
            "cbz.n %[trigger], skip%=\n"
            "nop.n\n"
            "adr.w lr, bad%=+1\n"
            // Prepare the register values for a simple computation:
            // r6 = label << 1; r8 = 1 << 31;
            // So r6 * r8 = label << 32; i.e., label is at the higher word
            "skip%=: adr.w r6, continue%=+1\n"
            "mov.w r8, #0x80000000\n"
            "lsl.w r6, r6, 1 \n"
            "mov.w r5, #1\n"
            "mov.w r4, #0\n"
            ".align 4 \n"
            "isb.w \n"
            // A very long instruction to make the test reliable with slow memories
            "udiv.w %[div_in], %[div_in], %[two] \n"

            // ---- the issue snippet --- >
            "umull.w r9, lr, r8, r6 \n" // LR:R9 = R6 * r8
            "mla.w r9, r5, r4, lr\n"    // R9 = (R5*R6)[31:0] + LR
            "bx.n lr\n"
            // <--- end of the snippet ----

            // A padding checking for extra execution
            ".rept 8; add.n %[div_in], %[div_in]; .endr \n"
            // The wrong destination of the "stale" LR
            // "bad%=:  udf 42\n" // Some compilers complain, so we just put an opcode
            "bad%=:  .short 0xde2a\n"
            ".space 32\n"
            // The correct destination - end of asm block
            "continue%=: \n"
            : [div_in] "+l" (div_stall_op)
    : [two] "r" (2), [trigger] "l" (trigger)
    : "cc", "lr", "r9", "r8", "r6", "r5", "r4"
    );
    return div_stall_op;
}

int main() {
    test_bug(0); // stale LR == correct LR
    test_bug(1); // stale LR -> Undefined Instruction
    return 42;
}
